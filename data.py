"""
Clean up subtitles csv file and process with nlp the entities to data.txt.

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


def read_csv_to_paragraph(path_csv):
    """Read CSV file and join cells into a single paragraph."""
    with open(path_csv, newline="") as csv_file:
        data_reader = csv.reader(csv_file, delimiter=",", quotechar='"')
        data = list(data_reader)

    paragraph = ""
    for row in data:
        if row:
            # Join the non-empty cells of each row with spaces
            paragraph += " ".join(row) + " "
        else:
            # Add a newline to separate the text into paragraphs
            paragraph += "\n\n"

    return paragraph


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


def extract_named_entities(paragraph):
    """Extract named entities from paragraph using spaCy model."""
    nlp = spacy.load("en_core_web_sm")
    entities = []

    # Process the input text
    doc = nlp(paragraph)

    # Find named entities in the text
    for entity in doc.ents:
        entry = {
            "text": entity.text,
            "label": entity.label_,
        }
        entities.append(entry)

    return entities


def write_entries_to_csv(path_csv, entities):
    """Write named entities to CSV file."""
    with open(path_csv, "w", newline="") as csv_file:
        fieldnames = [
            "text",
            "label",
        ]
        csv_writer = csv.DictWriter(csv_file, fieldnames=fieldnames)
        csv_writer.writeheader()
        for entry in entities:
            csv_writer.writerow(entry)


#############################################################################

# Set paths for input and output files
path_csv_data = "data.csv"
path_txt = path_csv_data.replace(".csv", ".txt")
path_csv_data_label = path_csv_data.replace("data", "data_label")

#############################################################################

# Read CSV data, extract named entities, and write to CSV file
paragraph = read_csv_to_paragraph(path_csv_data)
entities = extract_named_entities(paragraph)
write_entries_to_csv(path_csv_data_label, entities)

# Write paragraph to TXT file
with open(path_txt, "w", newline="") as txt_file:
    txt_file.write(paragraph)
    print(f"Written to {path_txt} successfully!")

#############################################################################
