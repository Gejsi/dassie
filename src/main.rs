mod nodes;
mod parser;

use cssparser::{Parser as Lexer, ParserInput as LexerInput};
use std::{error::Error, fs};

use crate::parser::{Parse, Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let daisy_css = fs::read_to_string("static/single.css")?;

    let mut lexer_input = LexerInput::new(&daisy_css);
    let mut lexer = Lexer::new(&mut lexer_input);

    println!("--------------------------------------------------------------------------");
    // let mut parser = Parser::new(lexer);
    let res = Parser::parse_selector(&mut lexer).unwrap();
    println!("{:#?}", res);
    println!("--------------------------------------------------------------------------");

    //     let input = r#"
    // center / contain no-repeat
    //       url('../../media/examples/firefox-logo.svg'),
    //     #eee 35% url('../../media/examples/lizard.png')
    //         "#;
    //     let mut lexer_input = LexerInput::new(&input);
    //     let mut lexer = Lexer::new(&mut lexer_input);
    //     let a = Parser::parse_value(&mut lexer).unwrap();
    //     println!("{a:?}");

    Ok(())
}
