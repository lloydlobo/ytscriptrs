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

use std::{
    fs::File,
    process::{Command, Output},
};

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use rayon::{iter::ParallelIterator, prelude::ParallelBridge};
use xml::reader::{EventReader, XmlEvent};

// static ARGS: Lazy<[&'static str; 6]> = Lazy::new(|| {
//     ["--all-subs", "--skip-download", "--sub-format", "ttml/vtt/best", "--sub-langs", "en"]
// });
static URL: Lazy<&'static str> = Lazy::new(|| "https://youtu.be/HHjgK6p4nrw");
// static TERM_LOG_MESSAGE: Lazy<&'static str> = Lazy::new(|| "[info] Writing video subtitles to:
// ");
static ARGS: [&str; 6] =
    ["--all-subs", "--skip-download", "--sub-format", "ttml/vtt/best", "--sub-langs", "en"];
static TERM_LOG_MESSAGE: &str = "[info] Writing video subtitles to: ";

fn main() {
    try_main().unwrap();
}
// * Removed the once_cell crate, as the static values can be defined directly without it.
// * Replaced the rayon::iter::ParallelIterator trait with the simpler rayon::prelude::* module
//   import, which includes the trait as well as a number of other commonly-used types and
//   functions.
// * Simplified the find_subtitle_filename function using the find_map method and the Option type's
//   map and ok_or_else methods.
fn try_main() -> anyhow::Result<()> {
    let output: Output = download_youtube_subs(*URL).unwrap();
    let file_api_video_title: &str = &String::from_utf8(output.stdout)?;
    let xml_file_path = {
        match file_api_video_title
            .lines()
            .par_bridge()
            .find_last(|line| line.contains(TERM_LOG_MESSAGE))
        {
            Some(val) => {
                let mut filename = val.replace(TERM_LOG_MESSAGE, "").trim().to_owned();
                filename.shrink_to_fit(); // Optional: to reduce capacity.
                Ok(filename)
            }
            None => Err(anyhow!("No matches found")),
        }
    }?;
    let subtitles_global = extract_subtitles_xml(&xml_file_path)?;
    let output_csv = format!("sub_{filename}.csv", filename = xml_file_path);
    csv_write_subtitles(&output_csv, &subtitles_global)?;

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////

fn download_youtube_subs(url: &str) -> Result<Output> {
    let mut args = (ARGS).to_vec();
    args.push(url);
    let output: Output = Command::new("yt-dlp").args(args).output()?;
    // match output.status.code().unwrap() {
    //     0 => println!("Subtitles downloaded successfully!"),
    //     _ => println!("Error downloading subtitles: {}", String::from_utf8(output.stderr)?),
    // }
    // let out = find_subtitle_filename(&String::from_utf8(output.stdout)?)?;
    Ok(output)
}
fn find_subtitle_filename(output: &str) -> Result<String> {
    match output.lines().par_bridge().find_first(|line| line.contains(TERM_LOG_MESSAGE)) {
        Some(val) => {
            let mut filename = val.replace(TERM_LOG_MESSAGE, "").trim().to_owned();
            filename.shrink_to_fit(); // Optional: to reduce capacity.
            Ok(filename)
        }
        None => Err(anyhow!("No matches found")),
    }
}

//////////////////////////////////////////////////////////////////////////////////

// Avoid unnecessary allocations: In the extract_subtitles_xml function, you are currently pushing
// each subtitle to a Vec<String>. However, you could avoid allocating a new string for each
// subtitle by using a String buffer and appending the subtitles to it. You can also preallocate the
// Vec with an initial capacity to avoid reallocations as you add more elements.
// let mut subtitles: Vec<String> = vec![];
// for event in parser.into_iter() {
// match event {
// Ok(XmlEvent::Characters(s)) => {
// if !s.trim().is_empty() {
// subtitles.push(s); } }
fn extract_subtitles_xml(xml_file: &str) -> Result<Vec<String>> {
    let file = File::open(xml_file)?;
    let parser = EventReader::new(file);
    // let mut subtitles = Vec::with_capacity(100); // Preallocate with initial capacity
    let mut subtitles = Vec::new();
    let mut buffer = String::new(); // Use a buffer to avoid allocating new strings
    for event in parser.into_iter() {
        match event {
            Ok(XmlEvent::Characters(s)) => {
                if !s.trim().is_empty() {
                    buffer.push_str(&s);
                }
            }
            // NOTE: Don't need this now.
            // Ok(XmlEvent::StartElement { name, attributes, .. }) => {
            //     get_timestamps(name, attributes, &mut subtitles) }
            _ => {}
        }
        if !buffer.trim().is_empty() {
            subtitles.push(buffer.clone());
            buffer.clear();
        }
    }
    Ok(subtitles)
}

#[allow(unused)]
fn get_timestamps(
    name: xml::name::OwnedName,
    attributes: Vec<xml::attribute::OwnedAttribute>,
    subtitles: &mut Vec<String>,
) {
    if name.local_name == "p" {
        let mut text = String::new();
        attributes.iter().for_each(|attr| match attr.name.local_name.as_str() {
            "begin" => text += &format!("{} - ", attr.value),
            "end" => text += &format!("{}\n", attr.value),
            _ => {}
        });
        // NOTE: Don't need this now.
        subtitles.push(text)
    }
}

//////////////////////////////////////////////////////////////////////////////////

fn csv_write_subtitles(path: &str, subtitles_global: &[String]) -> anyhow::Result<()> {
    // Change delimiter to something other than comma.
    let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').from_path(path)?;

    // Write header row. Write subtitle rows.
    wtr.write_record(["subtitle"])?;
    // subtitles_global.par_iter().for_each(|subtitle| wtr.write_record([subtitle]).unwrap());
    for subtitle in subtitles_global.iter() {
        wtr.write_record([subtitle]).unwrap()
    }
    wtr.flush()?; // Flush the writer to ensure all data is written to the file.

    println!("Subtitles written successfully to `{}`", path);

    Ok(())
}
