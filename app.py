"""
Extracting and summarizing a transcript from YouTube.

~~~~~~~~~~~~~~~~~~~

yt-dlp: A youtube-dl fork with additional features and fixes.
        Download videos from YouTube and other websites.

Extracted the auto-generated transcript like this:
yt-dlp --all-subs --skip-download \
  --sub-format ttml/vtt/best \
  https://m.youtube.com/watch?v=XlAqrS-fSAI

yt-dlp --write-auto-subs --skip-download \
  --sub-format ttml/vtt/best \
  --sub-langs en \
  https://m.youtube.com/watch?v=XlAqrS-fSAI

Source: https://gist.github.com/simonw/9932c6f10e241cfa6b19a4e08b283ca9
"""
import csv
import os
import subprocess
import uuid
import xml.etree.ElementTree as ET

###############################################################################

# Define the path to the Bash script
SCRIPT_PATH = os.path.join(os.path.dirname(__file__), "script.sh")
CMD_LS = "ls -l"

###############################################################################

subtitles_global = []

###############################################################################


def run_script(script_path: str) -> tuple[str, str, int]:
    """Run a Bash script and return the output and exit code."""
    process = subprocess.Popen(
        script_path, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True
    )
    output, error = process.communicate()
    exit_code = process.returncode
    return output.decode("utf-8"), error.decode("utf-8"), exit_code


def try_run_script():
    """Run the Bash script."""
    output, error, exit_code = run_script(SCRIPT_PATH)
    print(f"Output: {output}")
    print(f"Error: {error}")
    print(f"Exit code: {exit_code}")


###############################################################################


def run_cmd(command):
    """Run a bash command directly from a Python file."""
    print(f"$ {command}")
    result = subprocess.run(
        command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True
    )
    if result.returncode == 0:
        print(result.stdout.decode("utf-8"))
    else:
        print(result.stderr.decode("utf-8"))
    pass


###############################################################################


def extract_subtitles_xml(xml_file):
    """Extract subtitle text from an XML file and return as list of strings."""
    # Parse the XML file.
    tree = ET.parse(xml_file)
    root = tree.getroot()

    # Find all <p> elements and extract their text content.
    subtitles = []
    for p in root.iter("{http://www.w3.org/ns/ttml}p"):
        text = p.text
        if text is not None:
            text = text.strip()
            if len(text) > 0:
                subtitles.append(text)
    return subtitles


###############################################################################


def download_youtube_subs(url):
    """
    Run the Bash commands.

    yt-dlp --all-subs --skip-download \
      --sub-format ttml/vtt/best \
      --sub-langs en \
      https://youtu.be/HHjgK6p4nrw
    """
    command = [
        "yt-dlp",
        "--all-subs",
        "--skip-download",
        "--sub-format",
        "ttml/vtt/best",
        "--sub-langs",
        "en",
        url,
    ]
    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    output, error = process.communicate()
    exit_code = process.returncode

    if exit_code == 0:
        print("Subtitles downloaded successfully!")
    else:
        print(f"Error downloading subtitles: {error.decode()}")

    # write output log to a file
    with open("app_log.txt", "wb") as f:
        f.write(output)
    # # read and print file contents
    # # with open("app_log.txt", "rb") as f:
    # #   # print(f.read().decode())
    return find_subtitle_filename(output)


###############################################################################


# <p begin="00:00:05.810" end="00:00:07.370" style="s2">
#     - Thank you all for coming tonight,
# </p>
def append_to_global_subtitles(out_subtitles):
    """Write expenses to global list of subtitles dictionary."""
    counter = 0
    for sub in out_subtitles:
        subtitle = {
            "id": str(uuid.uuid4()),
            "index": counter,
            "subtitle": sub,
        }
        subtitles_global.append(subtitle)
        counter += 1
        pass
    pass


###############################################################################


def find_subtitle_filename(output: bytes):
    """Scrape yt-dlp subtitle output filename."""
    term_log_info = "Writing video subtitles to: "
    term_log = "[info] "
    lines = (output.decode()).splitlines()
    filename = ""
    for i in range(len(lines)):
        line = lines[i]
        if ("Writing video subtitles to:").lower() in str(line.lower()):
            filename = (line).replace(term_log, "").replace(term_log_info, "")
    if len(filename) == 0:
        print("No matches found")
    return filename


def csv_write_subtitles(path):
    """Write expenses to the CSV file, from the list of dictionaries."""
    with open(path, "w", newline="") as csv_file:
        fieldnames = [
            "id",
            "index",
            "subtitle",
        ]
        csv_writer = csv.DictWriter(csv_file, fieldnames=fieldnames)
        csv_writer.writeheader()
        for subtitle in subtitles_global:
            csv_writer.writerow(subtitle)
    pass


###############################################################################


def main(url: str):
    """Run all the all 'app.py' sequences."""
    xml_filename = download_youtube_subs(url)

    out_subtitles = extract_subtitles_xml(xml_filename)
    append_to_global_subtitles(out_subtitles)

    output_csv = f"subtitles_{xml_filename}.csv"
    csv_write_subtitles(output_csv)

    print(f"Subtitles written successfully to '{output_csv}'!")
    pass


if __name__ == "__main__":
    url = "https://youtu.be/HHjgK6p4nrw"
    main(url)
