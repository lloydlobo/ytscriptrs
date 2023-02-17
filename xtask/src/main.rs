use anyhow::{anyhow, Result};
use xtask::try_run_task;

////////////////////////////////////////////////////////////////////////////////

/// # Dev
///
/// ```sh
/// $ cargo watch -c -x 'r --bin xtask -- flamegraphserve'
/// ```
///
/// # Errors
///
/// This function will return an error if .
fn main() -> Result<()> {
    // if let Err(e) = Task::main() {
    if let Err(e) = try_run_task() {
        eprintln!("{}", fmt_error(e));
        std::process::exit(1);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

fn fmt_error(e: anyhow::Error) -> String {
    let error = anyhow!("`{e}`").context(
        e.chain()
            .map(|err| anyhow!("try_main: {}", err.to_string()).to_string())
            .collect::<Vec<String>>()
            .join(";"),
    );

    format_args!("{error:?}", error = error).to_string()
}

////////////////////////////////////////////////////////////////////////////////
