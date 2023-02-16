use std::{
    fs::File,
    process::{Command, Output},
};

use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use xml::reader::{EventReader, XmlEvent};

//////////////////////////////////////////////////////////////////////////////////

pub static URL: Lazy<&'static str> = Lazy::new(|| "https://youtu.be/HHjgK6p4nrw");
pub static ARGS: [&str; 6] =
    ["--all-subs", "--skip-download", "--sub-format", "ttml/vtt/best", "--sub-langs", "en"];
pub static TERM_LOG_MESSAGE: &str = "[info] Writing video subtitles to: ";

//////////////////////////////////////////////////////////////////////////////////

pub fn download_youtube_subs(url: &str) -> Result<Output> {
    let mut args = (ARGS).to_vec();
    args.push(url);
    let output: Output = Command::new("yt-dlp").args(args).output()?;
    // println!("{:#?}", output);
    // match output.status.code().unwrap() {
    //     0 => println!("Subtitles downloaded successfully!"),
    //     _ => println!("Error downloading subtitles: {}", String::from_utf8(output.stderr)?),
    // }
    Ok(output)
}

//////////////////////////////////////////////////////////////////////////////////

pub fn find_subtitle_filename_no_shrink(output: &str) -> Result<String> {
    output
        .lines()
        .par_bridge()
        .find_map_last(|line| match line.contains(TERM_LOG_MESSAGE) {
            true => Some(line.replace(TERM_LOG_MESSAGE, "").trim().to_owned()),
            false => None,
        })
        .ok_or_else(|| anyhow!("No matches found"))
}

#[allow(unused)]
pub fn find_subtitle_filename_shrink(output: &str) -> Result<String> {
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

// Avoid unnecessary allocations: In the extract_subtitles_xml function, you are currently
// pushing each subtitle to a Vec<String>. However, you could avoid allocating a new string
// for each subtitle by using a String buffer and appending the subtitles to it. You can
// also preallocate the Vec with an initial capacity to avoid reallocations as you add more
// elements.
pub fn extract_subtitles_xml(xml_file: &str) -> Result<Vec<String>> {
    let file = File::open(xml_file)?;
    let parser = EventReader::new(file);

    // let mut subtitles = Vec::with_capacity(100); // Preallocate with initial capacity
    let mut subtitles = Vec::new();
    let mut buffer = String::new(); // Use a buffer to avoid allocating new strings
                                    //
    parser.into_iter().for_each(|event| {
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
    });

    Ok(subtitles)
}

#[allow(unused)]
pub fn get_timestamps(
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

pub fn csv_write_subtitles(path: &str, subtitles_global: &[String]) -> anyhow::Result<()> {
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

//////////////////////////////////////////////////////////////////////////////////

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

// static ARGS: Lazy<[&'static str; 6]> = Lazy::new(|| {
//     ["--all-subs", "--skip-download", "--sub-format", "ttml/vtt/best", "--sub-langs", "en"]
// });
// static TERM_LOG_MESSAGE: Lazy<&'static str> = Lazy::new(||
//     "[info] Writing video subtitles to: "
// );
