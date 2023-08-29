mod parser;

use cssparser::{Parser as Lexer, ParserInput as LexerInput};
use std::{error::Error, fs};

use crate::parser::{Parse, Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let daisy_css = fs::read_to_string("static/mini.css")?;

    let mut lexer_input = LexerInput::new(&daisy_css);
    let mut lexer = Lexer::new(&mut lexer_input);

    println!("--------------------------------------------------------------------------");
    // let mut parser = Parser::new(lexer);
    let decls = Parser::parse_declaration_block(&mut lexer).unwrap();
    println!("{:#?}", decls);
    println!("--------------------------------------------------------------------------");

    Ok(())
}
