use std::{error::Error, fs, process};

use dassie::run;

fn main() -> anyhow::Result<()> {
    let daisy_css = fs::read_to_string("static/mini.css")?;

    if let Err(e) = run(&daisy_css) {
        eprintln!("Something went wrong: {e}");
        process::exit(1);
    }

    Ok(())
}
