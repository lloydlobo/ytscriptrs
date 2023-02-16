"""
Create a chatbot from extracted subtitles.

~~~~~~~~~~~~~~~~~~~

* Extract the subtitles from the video file using a tool or library that can parse the subtitle format used in the video file.
* Process the subtitles to remove unwanted elements such as timestamps, speaker names, and other irrelevant information.
* Preprocess the cleaned subtitle text by tokenizing it, removing stop words, and stemming or lemmatizing the words to reduce the vocabulary size.
* Create a dictionary that maps the cleaned and preprocessed words to numerical values that can be used as input to a machine learning model.
* Train a machine learning model on the preprocessed subtitle data. You can use a simple model like a rule-based model or a more complex model like a neural network.
* Create a chatbot that takes user input and responds based on the machine learning model's output. The chatbot can use a simple rule-based approach to select the most appropriate response from a list of predefined responses.
* Test the chatbot with sample inputs to ensure it responds appropriately.

Use advanced natural language processing techniques and machine learning
algorithms to build a high-quality chatbot

~~~~~~~~~~~~~~~~~~~

Mac/Unix

    Install NLTK: run pip install --user -U nltk

    Install Numpy (optional): pip install --user -U numpy

    Test installation: run python then type import nltk

For older versions of Python it might be necessary to install setuptools
"""
import random
import string
import xml.etree.ElementTree as ET
from typing import List

from nltk.chat.util import Chat
from nltk.chat.util import reflections

# Import the get_subtitle function from previous implementation


def get_subtitle(file_path: str) -> List[str]:
    tree = ET.parse(file_path)
    root = tree.getroot()
    namespace = {"ttml": "http://www.w3.org/ns/ttml"}
    subtitle_elements = root.findall(".//ttml:p", namespace)
    subtitles = []
    for subtitle in subtitle_elements:
        if subtitle.text is not None:
            subtitles.append(subtitle.text.strip())
    return subtitles


# Define the chatbot's rules for generating responses
chatbot_pairs = [
    [r"hi|hello|hey", ["Hello!", "Hi there!", "Howdy!"]],
    [r"what is your name?", ["My name is Chatbot. How can I assist you today?"]],
    [
        r"what do you do?",
        [
            "I'm here to help you! Ask me anything.",
            "I can answer any questions you have.",
        ],
    ],
    [
        r"how are you?",
        ["I'm doing well, thanks for asking.", "I'm great, how are you?"],
    ],
    [r"bye|goodbye", ["Goodbye!", "Have a nice day!"]],
    [
        r"(.*)",
        [
            "I'm not sure I understand. Can you please rephrase your question?",
            "Can you please provide more context?",
        ],
    ],
]

# Define a function to run the chatbot


def run_chatbot():
    # Create a new chatbot instance with the defined pairs and reflections
    chatbot = Chat(chatbot_pairs, reflections)

    # Ask the user to enter a file path
    file_path = input("Please enter the path to the subtitle file: ")

    # Extract the subtitles using the get_subtitle function
    subtitles = get_subtitle(file_path)

    # Join the subtitles into a single string for the chatbot to process
    subtitle_string = " ".join(subtitles)

    # Set the seed for the random number generator for consistent responses
    random.seed(42)

    # Use the chatbot to process the subtitle string and generate responses
    print("Chatbot: Hello! How can I assist you today?")
    while True:
        user_input = input("User: ")
        if user_input.lower() in ["exit", "quit", "goodbye"]:
            print("Chatbot: Goodbye!")
            break
        else:
            response = chatbot.respond(user_input)
            if response:
                print("Chatbot: " + response)
            else:
                print(
                    "Chatbot: I'm not sure I understand. Can you please rephrase your question?"
                )
