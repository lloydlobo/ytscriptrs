"""
Clean up subtitles csv file and write to .txt.

~~~~~~~~~~~~~~~~~~~

Assuming the CSV data is stored in a file called data.csv, this script reads
the file using the csv module, which is part of Python's standard library.
The data is stored in a list of lists, where each inner list
represents a row of the CSV file.

The script then joins the non-empty cells of each row with spaces
to create a single line of text, and adds a double newline to separate
each line into paragraphs. The final result is stored in the paragraph
variable and printed to the console.

Note that this script assumes that the CSV file uses commas as separators
and double quotes as quote characters. You may need to adjust the delimiter
and quotechar arguments of the csv.reader function to match the format of
your CSV data.

Install the model in your terminal or command prompt:
    python -m spacy download en_core_web_sm
"""
import csv

import spacy

#############################################################################

PATH_CSV_DATA = "data.csv"
PATH_TXT = PATH_CSV_DATA.replace(".csv", ".txt")
PATH_CSV_DATA_LABEL = PATH_CSV_DATA.replace("data", "data_label")

#############################################################################

# Open the CSV file and read its contents.
with open(PATH_CSV_DATA, newline="") as csv_file:
    data_reader = csv.reader(csv_file, delimiter=",", quotechar='"')
    data = list(data_reader)
    pass

# Join the CSV data into a single paragraph.
paragraph = ""
for row in data:
    if row:
        # Join the non-empty cells of each row with spaces
        paragraph += " ".join(row) + " "
    else:
        # Add a newline to separate the text into paragraphs
        paragraph += "\n\n"


with open(PATH_TXT, "w", newline="") as txt_file:
    txt_file.write(paragraph)
    print(f"Written to {PATH_TXT} successfully!")

#############################################################################

"""
This code will use the spaCy library to process the paragraph variable,
which contains the text data. The nlp object is created by loading the
pre-trained English language model from the spaCy library. The doc object
is the result of running the NLP pipeline on the paragraph.

The code then uses the ents attribute of the doc object to extract named
entities from the text, such as people, organizations, and locations.
The code will print out each entity and its label, which indicates the
type of named entity it is.

Customize the NLP pipeline by adding or removing components,
such as entity recognition, dependency parsing, or part-of-speech tagging,
depending on the specific tasks you need to perform.
"""


nlp = spacy.load("en_core_web_sm")

entities = []

# Process the input text
doc = nlp(paragraph)

# Find named entities in the text
for entity in doc.ents:
    print(entity.text, entity.label_)
    entry = {
        "text": entity.text,
        "label": entity.label,
    }
    entities.append(entry)


def csv_write_entries(path):
    """Write text & label to the CSV file, from the list of dictionary."""
    with open(path, "w", newline="") as csv_file:
        fieldnames = [
            "text",
            "label",
        ]
        csv_writer = csv.DictWriter(csv_file, fieldnames=fieldnames)
        csv_writer.writeheader()
        for entry in entities:
            csv_writer.writerow(entry)
    pass


csv_write_entries(PATH_CSV_DATA_LABEL)

#############################################################################
