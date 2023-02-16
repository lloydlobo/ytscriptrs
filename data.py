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
"""
import csv

PATH_CSV = "data.csv"

# Open the CSV file and read its contents.
with open(PATH_CSV, newline="") as csv_file:
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

PATH_TXT = PATH_CSV.replace(".csv", ".txt")

with open(PATH_TXT, "w", newline="") as txt_file:
    txt_file.write(paragraph)
    print(f"Written to {PATH_TXT} successfully!")
