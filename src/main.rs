// #![allow(dead_code)]

use anyhow::{anyhow, Result};
use std::{
    fs::File,
    io::{BufWriter, Write},
    process::Command,
};
use uuid::Uuid;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let url = "https://youtu.be/HHjgK6p4nrw";
    let xml_file_path: String = download_youtube_subs(url)?;

    let subtitles_global: Vec<Subtitle> = parse_map_subtitles(&xml_file_path)?;
    let output_csv = format!("sub_{filename}.csv", filename = xml_file_path);
    csv_write_subtitles(&output_csv, &subtitles_global)?;

    {
        // TODO: remove filler,
        // (audience clapping)
        // I wanna thank you Guy--
        // - Well, thank you.
    }

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////

fn download_youtube_subs(url: &str) -> Result<String> {
    let output = Command::new("yt-dlp")
        .args([
            "--all-subs",
            "--skip-download",
            "--sub-format",
            "ttml/vtt/best",
            "--sub-langs",
            "en",
            url,
        ])
        .output()?;

    let stdout: String = String::from_utf8(output.stdout.clone())?;
    let stderr = String::from_utf8(output.stderr)?;

    match output.status.code().unwrap() {
        0 => println!("Subtitles downloaded successfully!"),
        _ => println!("Error downloading subtitles: {}", stderr),
    }

    let subtitle_filename = find_subtitle_filename(&stdout)?;

    // Write output log to a file.
    let f = File::create("app_log.txt")?;
    let mut log_file = BufWriter::new(f);
    log_file.write_all(&output.stdout)?;

    Ok(subtitle_filename)
}

// PERF: Change to find() method.
// PERF: Use prefix methods.
// # PERF: Replace "  " to "_".
// [src/main.rs:70] &line = "[info] Writing video subtitles to: Guy Kawasaki： The Top 10 Mistakes of Entrepreneurs [HHjgK6p4nrw].en-ehkg1hFWq8A.ttml"
fn find_subtitle_filename(output: &str) -> Result<String> {
    let term_log_message = "[info] Writing video subtitles to: ";
    for line in output.lines() {
        if line.contains(term_log_message) {
            let filename = line
                .replace(term_log_message, "")
                .trim() // .replace(" ", "_")
                .to_owned();
            // break;
            return Ok(filename);
        }
    }
    Err(anyhow!("No matches found"))
}

//////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Subtitle {
    id: String,
    index: i32,
    subtitle: String,
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
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name == "p" {
                    let mut text = String::new();
                    attributes
                        .into_iter()
                        .for_each(|attr| match attr.name.local_name.as_str() {
                            "begin" => {
                                text += &format!("{} - ", attr.value);
                            }
                            "end" => {
                                text += &format!("{}\n", attr.value);
                            }
                            _ => {}
                        });
                    // NOTE: Don't need this now.
                    // subtitles.push(text)
                }
            }
            _ => {}
        }
    }
    Ok(subtitles)
}

fn parse_map_subtitles(xml_file: &str) -> Result<Vec<Subtitle>> {
    let out: Vec<Subtitle> = extract_subtitles_xml(xml_file)?
        .iter()
        .enumerate()
        .map(|(i, subtitle)| Subtitle {
            id: Uuid::new_v4().to_string(),
            index: i as i32,
            subtitle: subtitle.trim().to_string(),
        })
        .collect();
    Ok(out)
}

// fn append_to_global_subtitles(out_subtitles: &[String], subtitles_global: &mut Vec<Subtitle>) {
//     for (i, sub) in out_subtitles.iter().enumerate() {
//         if !sub.trim().is_empty() {
//             let subtitle = Subtitle { id: Uuid::new_v4().to_string(), index: i as i32, subtitle: sub.clone(), };
//             subtitles_global.push(subtitle);
//         }
//     }
// }

//////////////////////////////////////////////////////////////////////////////////

/// NOTE: If each subtitle text is enclosed in double quotes, you may want to
/// remove the double quotes while writing to a file, you can change the delimiter
/// of the CSV writer to something other than a comma (which is the default delimiter),
/// and then manually concatenate the subtitle text with the delimiter.
///
// PERF: Add id, index..
// * wtr.write_record(&["id", "index", "subtitle"])?;
// * let mut wtr = csv::Writer::from_writer(File::create(path)?);
// * wtr.write_record(&[&subtitle.id, &subtitle.index.to_string(), &subtitle.subtitle])?;
fn csv_write_subtitles(path: &str, subtitles_global: &[Subtitle]) -> anyhow::Result<()> {
    // Change delimiter to something other than comma.
    let mut wtr = csv::WriterBuilder::new().delimiter(b'\t').from_path(path)?;

    // Write header row.
    wtr.write_record(["subtitle"])?;
    // Write subtitle rows.
    for subtitle in subtitles_global {
        wtr.write_record([&subtitle.subtitle])?;
    }

    // Flush the writer to ensure all data is written to the file.
    wtr.flush()?;
    println!("Subtitles written successfully to `{}`", path);

    Ok(())
}
