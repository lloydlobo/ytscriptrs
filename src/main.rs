// #![allow(dead_code)]
// TODO: remove filler,
// * (audience clapping) * I wanna thank you Guy`--``
// * `-` Well, thank you.
// PERF: Change to find() method.
// PERF: Use prefix methods.
// # PERF: Replace "  " to "_".
// [src/main.rs:70] &line = "[info] Writing video subtitles to: Guy Kawasaki： The Top 10 Mistakes
// of Entrepreneurs [HHjgK6p4nrw].en-ehkg1hFWq8A.ttml" use rayon::iter::IntoParallelIterator;
// use rayon::iter::IntoParallelRefIterator;
// use rayon::prelude::IntoParallelRefMutIterator;

use std::{fs::File, process::Command};

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use rayon::{iter::ParallelIterator, prelude::ParallelBridge};
use xml::reader::{EventReader, XmlEvent};

static ARGS: Lazy<[&'static str; 6]> = Lazy::new(|| {
    ["--all-subs", "--skip-download", "--sub-format", "ttml/vtt/best", "--sub-langs", "en"]
});

fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let url = "https://youtu.be/HHjgK6p4nrw";
    let xml_file_path: String = download_youtube_subs(url)?;
    let subtitles_global = extract_subtitles_xml(&xml_file_path)?;
    let output_csv = format!("sub_{filename}.csv", filename = xml_file_path);
    csv_write_subtitles(&output_csv, &subtitles_global)?;

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////

fn download_youtube_subs(url: &str) -> Result<String> {
    let mut args = (*ARGS).to_vec();
    args.push(url);
    let output = Command::new("yt-dlp").args(args).output()?;
    match output.status.code().unwrap() {
        0 => println!("Subtitles downloaded successfully!"),
        _ => println!("Error downloading subtitles: {}", String::from_utf8(output.stderr)?),
    }
    let out = find_subtitle_filename(&String::from_utf8(output.stdout)?)?;
    Ok(out)
}

fn find_subtitle_filename(output: &str) -> Result<String> {
    let term_log_message = "[info] Writing video subtitles to: ";
    let found = output.lines().par_bridge().find_first(|line| line.contains(term_log_message));
    match found {
        Some(val) => Ok(val.replace(term_log_message, "").trim().to_owned()),
        None => Err(anyhow!("No matches found")),
    }
}

//////////////////////////////////////////////////////////////////////////////////

fn extract_subtitles_xml(xml_file: &str) -> Result<Vec<String>> {
    let file = File::open(xml_file)?;
    let parser = EventReader::new(file);
    let mut subtitles: Vec<String> = vec![];
    for event in parser.into_iter() {
        match event {
            Ok(XmlEvent::Characters(s)) => {
                if !s.trim().is_empty() {
                    subtitles.push(s);
                }
            }
            /* Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "p" {
                    let mut text = String::new();
                    attributes
                        .iter()
                        .for_each(|attr| match attr.name.local_name.as_str() {
                            "begin" => text += &format!("{} - ", attr.value),
                            "end" => text += &format!("{}\n", attr.value),
                            _ => {}
                        });
                    // NOTE: Don't need this now.
                    // subtitles.push(text)
                }
            } */
            _ => {}
        }
    }
    Ok(subtitles)
}

//////////////////////////////////////////////////////////////////////////////////

fn csv_write_subtitles(path: &str, subtitles_global: &[String]) -> anyhow::Result<()> {
    // Change delimiter to something other than comma.
    let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').from_path(path)?;

    // Write header row. Write subtitle rows.
    wtr.write_record(["subtitle"])?;
    for subtitle in subtitles_global.iter() {
        wtr.write_record([subtitle]).unwrap()
    }
    wtr.flush()?; // Flush the writer to ensure all data is written to the file.

    println!("Subtitles written successfully to `{}`", path);

    Ok(())
}
