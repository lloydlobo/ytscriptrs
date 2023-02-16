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
    let subtitles_global: Vec<Subtitle> = extract_subtitles_xml(&xml_file_path)?;
    dbg!(&subtitles_global);
    let output_csv = format!("sub_{filename}.csv", filename = xml_file_path);
    csv_write_subtitles(&output_csv, &subtitles_global)
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
    let result: String = String::from_utf8(output.stdout.clone())?;
    let error = String::from_utf8(output.stderr)?;
    match output.status.code().unwrap() {
        0 => println!("Subtitles downloaded successfully!"),
        _ => println!("Error downloading subtitles: {}", error),
    }

    let subtitle_filename = find_subtitle_filename(&result)?;
    dbg!(&subtitle_filename);

    // Write output log to a file.
    let f = File::create("app_log.txt")?;
    dbg!(&f);
    let mut log_file = BufWriter::new(f);
    log_file.write_all(&output.stdout)?;
    dbg!(&log_file);

    Ok(subtitle_filename)
}

// PERF: Change to find() method.
// PERF: Use prefix methods.
// # PERF: Replace "  " to "_".
// [src/main.rs:70] &line = "[info] Writing video subtitles to: Guy Kawasaki： The Top 10 Mistakes of Entrepreneurs [HHjgK6p4nrw].en-ehkg1hFWq8A.ttml"
fn find_subtitle_filename(output: &str) -> Result<String> {
    let term_log_message = "[info] Writing video subtitles to: ";
    let mut filename = String::new();
    for line in output.lines() {
        if line.contains(&term_log_message) {
            filename = line
                .replace(&term_log_message, "")
                .trim() // .replace(" ", "_")
                .to_owned();
            break;
        }
    }
    if filename.is_empty() {
        return Err(anyhow!("No matches found"));
    }
    // let filename = format!(
    //     "subtitles_{filename}_{now}",
    //     filename = filename,
    //     now = chrono::Utc::now().to_rfc3339(),
    // );
    Ok(filename)
}

//////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Subtitle {
    id: String,
    index: i32,
    subtitle: String,
}

fn append_to_global_subtitles(out_subtitles: &[String], subtitles_global: &mut Vec<Subtitle>) {
    for (i, sub) in out_subtitles.iter().enumerate() {
        if !sub.trim().is_empty() {
            let subtitle = Subtitle {
                id: Uuid::new_v4().to_string(),
                index: i as i32,
                subtitle: sub.clone(),
            };
            subtitles_global.push(subtitle);
        }
    }
}

fn extract_subtitles_xml(xml_file: &str) -> Result<Vec<Subtitle>> {
    fn extract_subtitles(xml_file: &str) -> Result<Vec<String>> {
        let file = File::open(xml_file)?;
        let parser = EventReader::new(file);
        let mut subtitles: Vec<String> = vec![];
        for event in parser {
            match event {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.local_name == "p" {
                        let mut text = String::new();
                        for attr in attributes.into_iter() {
                            match attr.name.local_name.as_str() {
                                "begin" => {
                                    // text += &format!("{} - ", attr.value);
                                }
                                "end" => {
                                    // text += &format!("{}\n", attr.value);
                                }
                                _ => {}
                            }
                        }
                        // subtitles.push(text)
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    if !s.trim().is_empty() {
                        subtitles.push(s);
                    }
                }
                _ => {}
            }
        }
        Ok(subtitles)
    }
    let out: Vec<Subtitle> = extract_subtitles(xml_file)?
        .iter()
        .enumerate()
        .map(|(i, subtitle)| {
            let subtitle = subtitle.trim().to_string();
            Subtitle {
                id: Uuid::new_v4().to_string(),
                index: i as i32,
                subtitle,
            }
        })
        .collect();

    Ok(out)
}

fn csv_write_subtitles(path: &str, subtitles_global: &[Subtitle]) -> anyhow::Result<()> {
    let mut wtr = csv::Writer::from_writer(File::create(path)?);

    // Write header row.
    // wtr.write_record(&["id", "index", "subtitle"])?;
    wtr.write_record(&["subtitle"])?;

    // Write subtitle rows.
    for subtitle in subtitles_global {
        dbg!(subtitle);
        wtr.write_record(&[
            // &subtitle.id,
            // &subtitle.index.to_string(),
            &subtitle.subtitle,
        ])?;
    }

    // Flush the writer to ensure all data is written to the file.
    wtr.flush()?;
    println!("Subtitles written successfully to `{}`", path);

    Ok(())
}
