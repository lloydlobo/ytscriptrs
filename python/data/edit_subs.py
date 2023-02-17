import pysubs2

# pysubs2 is a Python library for editing subtitle files. It’s based on SubStation Alpha, the native format of Aegisub; it also supports SubRip (SRT), MicroDVD, MPL2, TMP and WebVTT formats and OpenAI Whisper captions.

# Open the SRT file
subs = pysubs2.load("subtitle.srt")

# Print the subtitle text and start/end times
for line in subs:
    print(line.text)
    print(line.start, line.end)

# https://pysubs2.readthedocs.io/en/latest/

"""
$ pip install pysubs2
$ pysubs2 --shift 0.3s *.srt
$ pysubs2 --to srt *.ass

mport pysubs2
subs = pysubs2.load("my_subtitles.ass", encoding="utf-8")
subs.shift(s=2.5)
for line in subs:
    line.text = "{\\be1}" + line.text
subs.save("my_subtitles_edited.ass")
"""
