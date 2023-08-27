mod parser;

use cssparser::{Parser as Lexer, ParserInput as LexerInput};
use std::fs;

use crate::parser::{Parse, Parser};

fn main() -> anyhow::Result<()> {
    let daisy_css = fs::read_to_string("static/mini.css")?;

    let mut lexer_input = LexerInput::new(&daisy_css);
    let lexer = Lexer::new(&mut lexer_input);

    println!("--------------------------------------------------------------------------");
    let mut parser = Parser::new(lexer);
    let decl = parser.parse_declaration().unwrap();
    println!("{:#?}", decl);
    println!("--------------------------------------------------------------------------");

    Ok(())
}
