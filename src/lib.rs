use cssparser::{ParseError, Parser as Lexer, ParserInput as LexerInput, Token};

fn parse(lexer: &mut Lexer, selector: &mut String) -> anyhow::Result<()> {
    while let Ok(token) = lexer.next_including_whitespace() {
        match token {
            Token::Delim(value) => selector.push_str(&value.to_string()),
            Token::WhiteSpace(value) => selector.push_str(value),
            Token::Comma => selector.push_str(","),

            Token::CurlyBracketBlock => {
                println!("{selector}");
                selector.clear();
            }

            Token::Ident(value)
            | Token::Hash(value)
            | Token::IDHash(value)
            | Token::UnquotedUrl(value) => selector.push_str(value),
            Token::AtKeyword(value) => selector.push_str(&format!("@{value}")),
            Token::QuotedString(value) => selector.push_str(&format!("'{value}'")),

            Token::ParenthesisBlock => {
                selector.push_str("(");
                lexer
                    .parse_nested_block(|inner_lexer| {
                        let _ = parse(inner_lexer, selector);
                        selector.push_str(")");
                        Ok::<(), ParseError<anyhow::Error>>(())
                    })
                    .unwrap();
            }
            Token::SquareBracketBlock => {
                selector.push_str("[");
                lexer
                    .parse_nested_block(|inner_lexer| {
                        let _ = parse(inner_lexer, selector);
                        selector.push_str("]");
                        Ok::<(), ParseError<anyhow::Error>>(())
                    })
                    .unwrap();
            }
            Token::Function(value) => {
                selector.push_str(&format!("{value}("));
                lexer
                    .parse_nested_block(|inner_lexer| {
                        let _ = parse(inner_lexer, selector);
                        selector.push_str(")");
                        Ok::<(), ParseError<anyhow::Error>>(())
                    })
                    .unwrap();
            }

            Token::Colon => selector.push_str(":"),
            Token::Semicolon => selector.push_str(";"),
            Token::IncludeMatch => selector.push_str("~="),
            Token::DashMatch => selector.push_str("|="),
            Token::PrefixMatch => selector.push_str("^="),
            Token::SuffixMatch => selector.push_str("$="),
            Token::SubstringMatch => selector.push_str("*="),
            _ => {}
        }
    }

    Ok(())
}

pub fn run(css: &str) -> anyhow::Result<()> {
    let mut lexer_input = LexerInput::new(&css);
    let mut lexer = Lexer::new(&mut lexer_input);

    println!("--------------------------------------------------------------------------");
    let _ = parse(&mut lexer, &mut String::new());
    println!("--------------------------------------------------------------------------");

    Ok(())
}
