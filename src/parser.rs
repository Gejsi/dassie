use cssparser::{BasicParseError, Delimiter, ParseError, Parser as Lexer, Token};

use crate::nodes::{Declaration, Property, Rule, Selector, Value};

pub trait Parse<'i: 't, 't> {
    type ParsingError;

    fn parse_rule(lexer: &mut Lexer<'i, 't>) -> Result<Rule, Self::ParsingError>;

    fn parse_selectors(lexer: &mut Lexer<'i, 't>) -> Result<Vec<Selector>, Self::ParsingError>;

    fn parse_declaration_block(
        lexer: &mut Lexer<'i, 't>,
    ) -> Result<Vec<Declaration>, Self::ParsingError>;

    fn parse_declaration(lexer: &mut Lexer<'i, 't>) -> Result<Declaration, Self::ParsingError>;

    fn parse_value(lexer: &mut Lexer<'i, 't>) -> Result<Value, Self::ParsingError>;

    fn eat(lexer: &mut Lexer<'i, 't>) -> Result<String, Self::ParsingError>;
}

pub struct Parser;

impl<'i: 't, 't> Parse<'i, 't> for Parser {
    type ParsingError = ParseError<'i, BasicParseError<'i>>;

    fn parse_rule(lexer: &mut Lexer<'i, 't>) -> Result<Rule, Self::ParsingError> {
        let selectors = Self::parse_selectors(lexer)?;
        let declaration_block = Self::parse_declaration_block(lexer)?;

        Ok(Rule {
            selectors,
            declaration_block,
        })
    }

    fn parse_selectors(lexer: &mut Lexer<'i, 't>) -> Result<Vec<Selector>, Self::ParsingError> {
        lexer.parse_until_before(Delimiter::CurlyBracketBlock, |lexer| {
            lexer.parse_comma_separated(|lexer| {
                let text = Self::eat(lexer)?.trim().to_string();
                Ok(Selector(text))
            })
        })
    }

    fn parse_declaration_block(
        lexer: &mut Lexer<'i, 't>,
    ) -> Result<Vec<Declaration>, Self::ParsingError> {
        lexer.expect_curly_bracket_block()?;
        lexer.parse_nested_block(|lexer| {
            let mut declarations: Vec<Declaration> = Vec::new();

            while !lexer.is_exhausted() {
                let decl = Self::parse_declaration(lexer)?;
                declarations.push(decl);
            }

            Ok(declarations)
        })
    }

    fn parse_declaration(lexer: &mut Lexer<'i, 't>) -> Result<Declaration, Self::ParsingError> {
        lexer.parse_until_after(Delimiter::Semicolon, |lexer| {
            let property = Property(lexer.expect_ident()?.to_string());
            lexer.expect_colon()?;
            let value = Self::parse_value(lexer)?;

            Ok(Declaration(property, value))
        })
    }

    fn parse_value(lexer: &mut Lexer<'i, 't>) -> Result<Value, Self::ParsingError> {
        let text = Self::eat(lexer)?.trim().to_string();
        Ok(Value(text))
    }

    fn eat(lexer: &mut Lexer<'i, 't>) -> Result<String, Self::ParsingError> {
        let mut text = String::new();

        while let Ok(token) = lexer.next_including_whitespace() {
            match token {
                Token::Delim(value) => text.push_str(&value.to_string()),
                Token::WhiteSpace(value) => text.push_str(value),
                Token::Comma => text.push_str(","),
                Token::Ident(value) | Token::UnquotedUrl(value) => text.push_str(value),
                Token::Hash(value) | Token::IDHash(value) => text.push_str(&format!("#{value}")),
                Token::AtKeyword(value) => text.push_str(&format!("@{value}")),
                Token::QuotedString(value) => text.push_str(&format!("'{value}'")),
                Token::Colon => text.push_str(":"),
                Token::Semicolon => return Ok(text),
                Token::IncludeMatch => text.push_str("~="),
                Token::DashMatch => text.push_str("|="),
                Token::PrefixMatch => text.push_str("^="),
                Token::SuffixMatch => text.push_str("$="),
                Token::SubstringMatch => text.push_str("*="),

                Token::CurlyBracketBlock => {
                    return Ok(text);
                }

                Token::ParenthesisBlock => {
                    text.push_str("(");
                    lexer.parse_nested_block(|inner_lexer| {
                        let mut inner_text = Self::eat(inner_lexer)?;
                        inner_text.push_str(")");
                        text.push_str(&inner_text.to_string());
                        Ok::<(), Self::ParsingError>(())
                    })?;
                }
                Token::SquareBracketBlock => {
                    text.push_str("[");
                    lexer.parse_nested_block(|inner_lexer| {
                        let mut inner_text = Self::eat(inner_lexer)?;
                        inner_text.push_str("]");
                        text.push_str(&inner_text.to_string());
                        Ok::<(), Self::ParsingError>(())
                    })?;
                }
                Token::Function(value) => {
                    text.push_str(&format!("{value}("));
                    lexer.parse_nested_block(|inner_lexer| {
                        let mut inner_text = Self::eat(inner_lexer)?;
                        inner_text.push_str(")");
                        text.push_str(&inner_text.to_string());
                        Ok::<(), Self::ParsingError>(())
                    })?;
                }

                Token::Number {
                    value, int_value, ..
                } => {
                    if let Some(int_value) = int_value {
                        text.push_str(&format!("{int_value}"));
                    } else {
                        text.push_str(&format!("{value}"));
                    }
                }

                Token::Percentage {
                    unit_value,
                    int_value,
                    ..
                } => {
                    if let Some(int_value) = int_value {
                        text.push_str(&format!("{int_value}%"));
                    } else {
                        text.push_str(&format!("{}%", unit_value * 100.0));
                    }
                }

                Token::Dimension {
                    value,
                    int_value,
                    unit,
                    ..
                } => {
                    if let Some(int_value) = int_value {
                        text.push_str(&format!("{int_value}{unit}"));
                    } else {
                        text.push_str(&format!("{value}{unit}"));
                    }
                }

                _ => {}
            }
        }

        Ok(text)
    }
}
