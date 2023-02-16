use std::process::Output;

use ytscriptrs::*;

//////////////////////////////////////////////////////////////////////////////////

fn main() {
    try_main().unwrap();
}

fn try_main() -> anyhow::Result<()> {
    let output: Output = download_youtube_subs(*URL)?;
    let file_api_video_title: &str = &String::from_utf8(output.stdout)?;
    let xml_file_path = find_subtitle_filename_no_shrink(file_api_video_title)?;
    let subtitles_global = extract_subtitles_xml(&xml_file_path)?;
    let output_csv = format!("sub_{filename}.csv", filename = xml_file_path);
    csv_write_subtitles(&output_csv, &subtitles_global)?;

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////
