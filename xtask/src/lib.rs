use std::{
    env,
    io::{self, Write},
    process::{Command, Stdio},
};

use anyhow::{Context, Result};

////////////////////////////////////////////////////////////////////////////////

const HELP_HEADER: &str = r#"xtask 0.0.0
A cargo-xtask automation tool

USAGE:
    cargo xtask [COMMAND]...
ARGS:
"#;

////////////////////////////////////////////////////////////////////////////////

// PERF: Combine the 2 macros above so that running and generating help happens here.
// OR use `clap` :)

tasks!(
    Fetch => "run --bin ytscriptrs to fetch youtube subtitles via yt-dlp CLI";
    Flamegraph => "generate a flamegraph of the binary";
    Flamegraphserve => "generate a flamegraph and serve it via http";
    Todo => "builds rustdoc documentation";
);

fn print_help() -> Result<()> {
    let help = generate_help!(
        ("fetch", "run --bin ytscriptrs to fetch youtube subtitles via yt-dlp CLI"),
        ("flamegraph", "generate a flamegraph of the binary"),
        ("flamegraphserve", "generate a flamegraph and serve it via http"),
        ("todo", "builds rustdoc documentation")
    );
    eprintln!("{help}");

    Ok(())
}

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

#[macro_export]
macro_rules! tasks {
    ($(
        $task:ident => $description:expr;
    )*) => {
       #[derive(Debug, PartialEq)]
        pub enum Task {
            $($task),*
        }
        impl Task {
            fn from_str(input: &str) -> Option<Self> {
                match (input[0..1].to_uppercase() + &input[1..]).as_str() {
                    $(stringify!($task) => Some(Task::$task),)*
                    _ => None,
                }
            }
            fn run(self)-> Result<()>{
                match self{
                    Task::Fetch => run_fetch()?,
                    Task::Flamegraph => run_flamegraph()?,
                    Task::Flamegraphserve => run_flamegraph_serve()?,
                    Task::Todo => (),
                }
                Ok(())
            }
            pub fn main() -> Result<()> {
                let args: Vec<String> = env::args().skip(1).collect();
                if args.is_empty() {
                    print_help()?;
                } else if let Some(task) = Task::from_str(&args[0]) {
                    task.run()?;
                } else {
                    writeln!(io::stderr(), "Invalid command: {}", args[0])?;
                    print_help()?;
                }
                Ok(())
            }
        }
    };
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
