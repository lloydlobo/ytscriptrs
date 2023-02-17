use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{Context, Result};

////////////////////////////////////////////////////////////////////////////////

struct TaskM {
    name: String,
    func: Box<dyn Fn() -> Result<()>>,
}

fn run_task(task: &TaskM) -> Result<()> {
    (task.func)()
}

macro_rules! task {
    ($name:expr, $func:expr) => {{
        TaskM { name: $name.to_string(), func: Box::new($func) }
    }};
}

pub fn try_run_task() -> Result<()> {
    let tasks = vec![
        task!("fetch", run_fetch),
        task!("flamegraph", run_flamegraph),
        task!("flamegraphserve", run_flamegraph_serve),
        task!("todo", || Ok(())),
    ];

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("list") => {
            println!("Available tasks:");
            for task in tasks.iter() {
                println!("- {}", task.name);
            }
        }
        Some(name) => {
            let task = tasks.iter().find(|t| t.name == name);
            match task {
                Some(t) => run_task(t)?,
                None => {
                    eprintln!("Task not found: {}", name);
                    print_help()?;
                    std::process::exit(1);
                }
            }
        }
        None => {
            print_help()?;
            std::process::exit(1);
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

const HELP_HEADER: &str = r#"xtask 0.0.0
A cargo-xtask automation tool

USAGE:
    cargo xtask [COMMAND]...
ARGS:
"#;

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! generate_help {
    ( $( ($name:expr, $desc:expr) ),* ) => {{
        let mut help = String::from(HELP_HEADER);
        $(
            help.push_str(&format!("    {:<16}{}\n", $name, $desc));
        )*
        help
    }};
}

fn print_help() -> Result<()> {
    let help = generate_help!(
        ("fetch", "run --bin ytscriptrs to fetch youtube subtitles via yt-dlp CLI"),
        ("flamegraph", "generate a flamegraph of the binary"),
        ("flamegraphserve", "generate a flamegraph and serve it via http"),
        ("todo", "builds rustdoc documentation"),
        ("list", "Available tasks:")
    );
    eprintln!("{help}");

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

fn write_buf_stdout(buf: &[u8]) -> Result<()> {
    std::io::stdout().lock().write_all(buf).with_context(|| "Failed to write to stdout")?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

fn run_fetch() -> Result<()> {
    let cmd = Command::new("cargo")
        .args(["r", "-r", "--bin", "ytscriptrs"])
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to spawn process")?;
    let output = cmd.wait_with_output().with_context(|| "Failed to wait for process")?;
    write_buf_stdout(&output.stdout)?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

fn run_flamegraph() -> Result<()> {
    let cmd = Command::new("cargo")
        .args(["flamegraph", "-p", "ytscriptrs", "--inverted", "--deterministic"])
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to spawn process")?;
    let cmd = Command::new("bat") // HACK: Curbs printing of flamegraph non utf-8 stream.
        .stdin(cmd.stdout.unwrap())
        .spawn()
        .with_context(|| "Failed to spawn process")?;
    let output = cmd.wait_with_output().with_context(|| "Failed to wait for process")?;
    write_buf_stdout(&output.stdout)?;

    Ok(())
}

fn run_flamegraph_serve() -> Result<()> {
    run_flamegraph()?;
    let cmd = Command::new("miniserve")
        .args(["flamegraph.svg"]) // .stdout(Stdio::piped())
        .spawn()
        .with_context(|| "Failed to spawn process")?;
    let output = cmd.wait_with_output().with_context(|| "Failed to wait for process")?;
    write_buf_stdout(&output.stdout)?;

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////
