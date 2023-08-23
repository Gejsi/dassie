use std::fmt::Display;

use cssparser::{ParseError, Parser as Lexer, ParserInput as LexerInput, Token};

#[derive(Debug)]
struct Rule {
    selector: String,
    declarations: String,
}

struct Parser {
    rules: Vec<Rule>,
}

impl Parser {
    fn new() -> Self {
        Self {
            rules: vec![Rule {
                selector: "".to_string(),
                declarations: "".to_string(),
            }],
        }
    }

    fn parse(&mut self, lexer: &mut Lexer, mut i: usize) -> anyhow::Result<&mut Vec<Rule>> {
        while let Ok(token) = lexer.next_including_whitespace() {
            match token {
                Token::Delim(value) => self.rules[i].selector.push_str(&value.to_string()),
                Token::WhiteSpace(value) => self.rules[i].selector.push_str(value),
                Token::Comma => self.rules[i].selector.push_str(","),

                Token::CurlyBracketBlock => {
                    println!("{:#?}", self.rules[i]);
                    self.rules.push(Rule {
                        selector: "".to_string(),
                        declarations: "".to_string(),
                    });

                    i += 1;
                }

                Token::Ident(value)
                | Token::Hash(value)
                | Token::IDHash(value)
                | Token::UnquotedUrl(value) => self.rules[i].selector.push_str(value),
                Token::AtKeyword(value) => self.rules[i].selector.push_str(&format!("@{value}")),
                Token::QuotedString(value) => {
                    self.rules[i].selector.push_str(&format!("'{value}'"))
                }

                Token::ParenthesisBlock => {
                    self.rules[i].selector.push_str("(");
                    lexer
                        .parse_nested_block(|inner_lexer| {
                            let rules = self.parse(inner_lexer, i).unwrap();
                            rules[i].selector.push_str(")");
                            Ok::<(), ParseError<anyhow::Error>>(())
                        })
                        .unwrap();
                }
                Token::SquareBracketBlock => {
                    self.rules[i].selector.push_str("[");
                    lexer
                        .parse_nested_block(|inner_lexer| {
                            let rules = self.parse(inner_lexer, i).unwrap();
                            rules[i].selector.push_str("]");
                            Ok::<(), ParseError<anyhow::Error>>(())
                        })
                        .unwrap();
                }
                Token::Function(value) => {
                    self.rules[i].selector.push_str(&format!("{value}("));
                    lexer
                        .parse_nested_block(|inner_lexer| {
                            let rules = self.parse(inner_lexer, i).unwrap();
                            rules[i].selector.push_str(")");
                            Ok::<(), ParseError<anyhow::Error>>(())
                        })
                        .unwrap();
                }

                Token::Colon => self.rules[i].selector.push_str(":"),
                Token::Semicolon => self.rules[i].selector.push_str(";"),
                Token::IncludeMatch => self.rules[i].selector.push_str("~="),
                Token::DashMatch => self.rules[i].selector.push_str("|="),
                Token::PrefixMatch => self.rules[i].selector.push_str("^="),
                Token::SuffixMatch => self.rules[i].selector.push_str("$="),
                Token::SubstringMatch => self.rules[i].selector.push_str("*="),
                _ => {}
            }
        }

        Ok(&mut self.rules)
    }
}

pub fn run(css: &str) -> anyhow::Result<()> {
    let mut lexer_input = LexerInput::new(&css);
    let mut lexer = Lexer::new(&mut lexer_input);

    println!("--------------------------------------------------------------------------");
    let mut parser = Parser::new();
    let rules = parser.parse(&mut lexer, 0)?;
    // println!("{:#?}", rules);
    println!("--------------------------------------------------------------------------");

    Ok(())
}
