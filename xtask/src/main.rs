use anyhow::{anyhow, Result};
use xtask::Task;

////////////////////////////////////////////////////////////////////////////////

/// # Dev
///
/// ```sh
/// $ cargo watch -c -x 'r --bin xtask -- fetch'
/// ```
///
/// # Errors
///
/// This function will return an error if .
fn main() -> Result<()> {
    if let Err(e) = Task::main() {
        eprintln!("{}", format_args!("{:?}", fmt_error(e)));
        std::process::exit(1);
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

fn fmt_error(e: anyhow::Error) -> anyhow::Error {
    let ctx = e
        .chain()
        .map(|err| anyhow!("try_main: {}", err.to_string()).to_string())
        .collect::<Vec<_>>()
        .join(", ");
    
    anyhow!("`{e}`").context(ctx)
}

////////////////////////////////////////////////////////////////////////////////
