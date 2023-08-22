use std::{error::Error, fs, process};

use dassie::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let daisy_css = reqwest::get("https://cdn.jsdelivr.net/npm/daisyui@3.3.1/dist/full.css")
    //     .await?
    //     .text()
    //     .await?;

    let daisy_css = fs::read_to_string("static/style.css")?;

    if let Err(e) = run(&daisy_css) {
        eprintln!("Something went wrong: {e}");
        process::exit(1);
    }

    Ok(())
}
