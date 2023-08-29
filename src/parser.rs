use cssparser::{BasicParseError, ParseError, Parser as Lexer, Token};

#[derive(Debug)]
pub struct Selector(String);

#[derive(Debug)]
pub struct Property(String);

#[derive(Debug)]
pub struct Value(String);

#[derive(Debug)]
pub struct Declaration {
    pub property: Property,
    pub value: Value,
}

#[derive(Debug)]
pub struct DeclarationBlock {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declaration_block: DeclarationBlock,
}

#[derive(Debug)]
pub struct AtRule {
    pub identifier: String,
    pub condition: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Rule,
    AtRule,
}

pub struct Stylesheet {
    pub statements: Vec<Statement>,
}

pub trait Parse<'i: 't, 't> {
    type ParsingError;

    fn parse_declaration_block(
        lexer: &mut Lexer<'i, 't>,
    ) -> Result<DeclarationBlock, Self::ParsingError>;

    fn parse_declaration(lexer: &mut Lexer<'i, 't>) -> Result<Declaration, Self::ParsingError>;

    fn parse_value(lexer: &mut Lexer<'i, 't>) -> Result<Value, Self::ParsingError>;

    fn eat(lexer: &mut Lexer<'i, 't>) -> Result<String, Self::ParsingError>;
}

pub struct Parser;

impl<'i: 't, 't> Parse<'i, 't> for Parser {
    type ParsingError = ParseError<'i, BasicParseError<'i>>;

    fn parse_declaration_block(
        lexer: &mut Lexer<'i, 't>,
    ) -> Result<DeclarationBlock, Self::ParsingError> {
        let declarations: Vec<Declaration> = Vec::new();

        lexer.expect_curly_bracket_block()?;
        lexer.parse_nested_block(|inner_lexer| {
            let decl = Self::parse_declaration(inner_lexer)?;

            println!("{decl:?}");

            Ok::<(), Self::ParsingError>(())
        })?;

        Ok(DeclarationBlock { declarations })
    }

    fn parse_declaration(lexer: &mut Lexer<'i, 't>) -> Result<Declaration, Self::ParsingError> {
        let property = Property(lexer.expect_ident()?.to_string());
        lexer.expect_colon()?;
        let value = Self::parse_value(lexer)?;
        if !lexer.is_exhausted() {
            lexer.expect_semicolon()?;
        }

        Ok(Declaration { property, value })
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
                Token::Ident(value)
                | Token::Hash(value)
                | Token::IDHash(value)
                | Token::UnquotedUrl(value) => text.push_str(value),
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
                    println!("'{{' found");
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
                _ => {}
            }
        }

        Ok(text)
    }
}
