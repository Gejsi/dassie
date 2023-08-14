use std::{convert::Infallible, fs};

use lightningcss::{
    rules::CssRuleList,
    stylesheet::{ParserOptions, StyleSheet},
    visit_types,
    visitor::{Visit, VisitTypes, Visitor},
};

struct Explorer;

impl<'i, T> Visitor<'i, T> for Explorer
where
    T: Visit<'i, T, Self>,
{
    type Error = Infallible;

    const TYPES: VisitTypes = visit_types!(RULES);

    fn visit_rule_list(&mut self, rules: &mut CssRuleList<'i, T>) -> Result<(), Self::Error> {
        // rules.visit_children(self)
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
    let stylesheet = StyleSheet::parse(&daisy_css, ParserOptions::default()).unwrap();
    let _target_class = "tabs";
    stylesheet.visit(&mut Explorer)?;

    // for rule in stylesheet.rules.0 {
    //     match rule {
    //         CssRule::Style(style) => {
    //             for selector in style.selectors.0 {
    //                 println!("{:?}", selector);
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    Ok(())
}

// impl<'i, T, V> Visit<'i, T, V> for Explorer
// where
//     T: Visit<'i, T, V>,
//     V: Visitor<'i, T>,
// {
//     const CHILD_TYPES: VisitTypes = visit_types!(RULES);

//     // Required method
//     fn visit_children(&mut self, visitor: &mut V) -> Result<(), V::Error> {
//         Ok(())
//     }
// }
