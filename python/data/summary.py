import fileinput
import sys

from summa import keywords
from summa import summarizer

text = """"""

# https://stackoverflow.com/a/38497356
with fileinput.input() as f_input:  # This gets the piped data for you
    # print(f_input)
    for line in f_input:
        # print(line)
        text += line
        # do stuff with line of piped data
        # Process the text here
    pass
pass

# 2      90     565
# https://pypi.org/project/summa/
# text = """Automatic summarization is the process of reducing a text document with a \
# computer program in order to create a summary that retains the most important points \
# of the original document. As the problem of information overload has grown, and as \
# the quantity of data has increased, so has interest in automatic summarization. \
# Technologies that can make a coherent summary take into account variables such as \
# length, writing style and syntax. An example of the use of summarization technology \
# is search engines such as Google. Document summarization is another."""

print(f"# Data\n{text}")
print()
print(f"# Keywords\n{keywords.keywords(text)}")
print()
print(f"# Summary\n{summarizer.summarize(text)}")

"""
Some famous speeches by John F. Kennedy and Martin Luther King Jr.:

John F. Kennedy:

    Inaugural Address (January 20, 1961)
    Civil Rights Address (June 11, 1963)
    Address at American University (June 10, 1963)

Martin Luther King Jr.:

    "I Have a Dream" (August 28, 1963)
    "I've Been to the Mountaintop" (April 3, 1968)
    "Our God Is Marching On!" (March 25, 1965)

Summary 1: Martin Luther King Jr. delivered his famous "I Have a Dream" speech on August 28, 1963, on the steps of the Lincoln Memorial in Washington D.C. The speech called for racial equality and an end to discrimination against African Americans. It has become one of the most iconic speeches in American history.

Summary 2: "I Have a Dream" is a speech given by Martin Luther King Jr. during the March on Washington for Jobs and Freedom on August 28, 1963. In the speech, King calls for an end to racism in the United States and for civil and economic rights to be extended to all Americans, regardless of their race.

Summary 3: "I Have a Dream" is a historic speech delivered by civil rights activist Martin Luther King Jr. during the March on Washington on August 28, 1963. The speech is considered one of the most significant speeches in American history, and is a powerful call for racial equality and an end to discrimination against African Americans.
"""

"""
pip install gensim
pip install smart-open

NumPy for number crunching.
smart_open for transparently opening files on remote storages or compressed files.

# Core Concepts

* Document: some text.
* Corpus: a collection of documents.
* Vector: a mathematically convenient representation of a document.
* Model: an algorithm for transforming vectors from one representation to another.
❯ pip install summa
  Downloading summa-1.2.0.tar.gz (54 kB)
     ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 54.9/54.9 kB 786.3 kB/s eta 0:00:00
Summa uses extractive summarization, which means that it selects the most
important sentences from the text to create a summary. If the input text is short, it can be difficult to extract meaningful sentences that summarize the content effectively. It's a good idea to experiment with different text lengths to find the best summary.
"""

"""
# https://stackoverflow.com/a/38497356

This works with python's print output just as easily:

>test.py  # This prints the contents of some_textfile.txt
with open('some_textfile.txt', 'r') as f:
    for line in f:
        print(line)

$ ./test.py | ./myscript.py
"""
