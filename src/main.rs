use std::{convert::Infallible, fs};

use lightningcss::{
    selector::SelectorList,
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    traits::ToCss,
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};

struct Explorer;

impl<'i> Visitor<'i> for Explorer {
    type Error = Infallible;

    const TYPES: VisitTypes = visit_types!(SELECTORS);

    fn visit_selector_list(&mut self, selectors: &mut SelectorList<'i>) -> Result<(), Self::Error> {
        let found: Vec<_> = selectors
            .0
            .iter()
            .filter(|selector| {
                let s = selector
                    .to_css_string(PrinterOptions::default())
                    .unwrap_or_else(|_| "".to_string());

                s.contains(".btn")
            })
            .collect::<_>();

        println!("{:?}", found);

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let daisy_css = reqwest::get("https://cdn.jsdelivr.net/npm/daisyui@3.3.1/dist/full.css")
    //     .await?
    //     .text()
    //     .await?;

    let daisy_css = fs::read_to_string("static/style.css")?;
    let mut stylesheet = StyleSheet::parse(&daisy_css, ParserOptions::default()).unwrap();
    let _target_class = "tabs";
    stylesheet.visit(&mut Explorer)?;

    Ok(())
}
