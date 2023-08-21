use cssparser::{CowRcStr, Parser as Lexer, ParserInput as LexerInput, Token};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let daisy_css = reqwest::get("https://cdn.jsdelivr.net/npm/daisyui@3.3.1/dist/full.css")
    //     .await?
    //     .text()
    //     .await?;

    let daisy_css = fs::read_to_string("src/static/style.css")?;

    let mut lexer_input = LexerInput::new(&daisy_css);
    let mut lexer = Lexer::new(&mut lexer_input);

    let mut selector = String::new();

    while let Ok(token) = lexer.next() {
        match token {
            Token::Delim(value) => selector.push_str(&value.to_string()),

            Token::WhiteSpace(value) => selector.push_str(value),

            Token::Comma => selector.push_str(", "),

            Token::CurlyBracketBlock => {
                dbg!(&selector);
                selector.clear();
            }

            Token::Ident(value)
            | Token::AtKeyword(value)
            | Token::Hash(value)
            | Token::IDHash(value)
            | Token::QuotedString(value)
            | Token::UnquotedUrl(value)
            | Token::Function(value) => selector.push_str(value),

            Token::ParenthesisBlock => selector.push_str("("),

            Token::SquareBracketBlock => selector.push_str("["),

            Token::CloseParenthesis => selector.push_str(")"),

            Token::CloseSquareBracket => selector.push_str("]"),

            _ => {}
        }
    }

    Ok(())
}
