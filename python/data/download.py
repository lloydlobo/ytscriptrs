from __future__ import unicode_literals

import youtube_dl

###############################################################################


class MyLogger(object):
    """
    A custom logger class to handle logging in youtube_dl.
    """

    def debug(self, msg):
        """
        Prints debug level messages to console.
        """
        print(msg)
        pass

    def warning(self, msg):
        """
        Prints warning level messages to console.
        """
        print(msg)
        pass

    def error(self, msg):
        """
        Prints error level messages to console.
        """
        print(msg)


###############################################################################


def my_hook(d):
    """
    Callback function that prints download progress.
    """
    print(">> d", d)
    if d["status"] == "finished":
        print("Done downloading, now converting ...")


###############################################################################


ydl_opts = {
    "progress_hooks": [my_hook],
    "verbose": "true",
    "skip_download": True,  # Don't download the video itself
    "writesubtitles": True,  # Enable subtitle downloading
    "writeautomaticsub": True,  # Write the automatically generated subtitles to a file
    #                           # (requires writesubtitles or writeautomaticsub)
    "logger": MyLogger(),
    "allsubtitles": True,  # Downloads all the subtitles of the video
    "subtitlesformat": "vtt",  # Specify the subtitle format
    "subtitleslangs": ["en"],  # Only download English subtitles
    "outtmpl": "file.vtt",  # Specify the output path
}

video_url = "https://youtu.be/BaW_jenozKc"

###############################################################################


def main():
    """
    Tries to extract subtitle information for the specified video.
    """
    with youtube_dl.YoutubeDL(ydl_opts) as ydl:
        info = ydl.extract_info(video_url)
        if info is not None:
            video_id = info.get("id")
            subs = ydl.process_subtitles(
                video_id, info.get("normal_subtitles"), info.get("automatic_captions")
            )
            if subs is not None:
                with open("youtube-dl_subtitle.vtt", "w") as f:
                    f.writelines(subs)

    pass


if __name__ == "__main__":
    main()

###############################################################################

# ytscriptrs on  master [!?] via 🦀 v1.69.0-nightly
# ❯ python3 python/data/download.py
# [debug] Encodings: locale UTF-8, fs utf-8, out utf-8, pref UTF-8
# [debug] youtube-dl version 2021.12.17
# [debug] Python version 3.11.1 (CPython) - Linux-6.1.10-200.fc37.x86_64-x86_64-with-glibc2.36
# [debug] exe versions: ffmpeg 5.1.2, ffprobe 5.1.2
# [debug] Proxy map: {}
# [youtube] BaW_jenozKc: Downloading webpage
# [debug] Default format spec: bestvideo+bestaudio/best
# [info] Writing video subtitles to: file.vtt.en.vtt


# [youtube] BaW_jenozKc: Downloading webpage
# [debug] Default format spec: bestvideo+bestaudio/best
# [info] Writing video subtitles to: file.vtt.en.vtt
# Available subtitles for BaW_jenozKc:
# Language formats
# en       vtt, ttml, srv3, srv2, srv1
# {'nocheckcertificate': False, 'progress_hooks': [<function my_hook at 0x7fec15941f80>], 'verbose': 'true', 'skip_download': True, 'writesubtitles': True, 'writeautomaticsub': True, 'l
# ogger': <__main__.MyLogger object at 0x7fec159a7390>, 'allsubtitles': True, 'subtitlesformat': 'vtt', 'subtitleslangs': ['en'], 'outtmpl': 'file.vtt'}
# BaW_jenozKc None None
# tried processing subtitles
# None

# "format": "bestaudio/best",
# "postprocessors": [ { "key": "FFmpegExtractAudio", "preferredcodec": "mp3", "preferredquality": "192", } ],

# with youtube_dl.YoutubeDL(ydl_opts) as ydl:
#     info_dict = ydl.extract_info(video_url)
#     if info_dict is not None:
#         video_id = info_dict.get("id")
#         automatic_captions = info_dict.get("automatic_captions")
#         normal_subtitles = info_dict.get("normal_subtitles")
#         x = ydl.list_subtitles(video_id, info_dict.get("subtitles"))
#         print(ydl.params)
#         print(video_id, normal_subtitles, automatic_captions)
#         x = ydl.process_subtitles(video_id, normal_subtitles, automatic_captions)
#         print(f"No errors when processing subtitles: {x}")
#         if x is not None:
#             with open("youtube-dl_subtitle.vtt", "w", newline="") as write_file:
#                 write_file.writelines(x)
#                 print(x)
#                 print("Wrote to file")
#                 pass
#             pass
#         pass
#     pass
#
# # pass
