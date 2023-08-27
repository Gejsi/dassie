mod parser;

use cssparser::{Parser as Lexer, ParserInput as LexerInput};

use crate::parser::{Parse, Parser};

pub fn run(css: &str) -> anyhow::Result<()> {
    let mut lexer_input = LexerInput::new(&css);
    let lexer = Lexer::new(&mut lexer_input);

    println!("--------------------------------------------------------------------------");
    let mut parser = Parser::new(lexer);
    let decl = parser.parse_declaration().unwrap();
    println!("{:#?}", decl);
    println!("--------------------------------------------------------------------------");

    Ok(())
}
