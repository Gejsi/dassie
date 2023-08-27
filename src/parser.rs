use cssparser::{BasicParseError, ParseError, Parser as Lexer, Token};

#[derive(Debug)]
pub struct Selector(String);

#[derive(Debug)]
pub struct Property(String);

#[derive(Debug)]
pub struct Value(String);

#[derive(Debug)]
pub struct Declaration {
    property: Property,
    value: Value,
}

#[derive(Debug)]
pub struct DeclarationBlock {
    declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub struct Rule {
    selectors: Vec<Selector>,
    declaration_block: DeclarationBlock,
}

#[derive(Debug)]
pub struct AtRule {
    identifier: String,
    condition: String,
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Rule,
    AtRule,
}

struct Stylesheet(Vec<Statement>);

pub trait Parse<'i, 't> {
    type ParsingError;

    fn new(lexer: Lexer<'i, 't>) -> Self;

    fn parse_declaration(&mut self) -> Result<Declaration, Self::ParsingError>;

    fn parse_value(&mut self) -> Result<Value, Self::ParsingError>;

    fn eat<'a: 't, 'b>(
        lexer: &mut Lexer<'a, 'b>,
        depth: usize,
    ) -> Result<String, Self::ParsingError>;
}

pub struct Parser<'i, 't> {
    pub lexer: Lexer<'i, 't>,
}

impl<'i: 't, 't> Parse<'i, 't> for Parser<'i, 't> {
    type ParsingError = ParseError<'i, BasicParseError<'i>>;

    fn new(lexer: Lexer<'i, 't>) -> Self {
        Parser { lexer }
    }

    fn parse_declaration(&mut self) -> Result<Declaration, Self::ParsingError> {
        let property = Property(self.lexer.expect_ident()?.to_string());
        self.lexer.expect_colon()?;
        let value = self.parse_value()?;
        println!("{value:?}");
        self.lexer.expect_semicolon()?;

        Ok(Declaration { property, value })
    }

    fn parse_value(&mut self) -> Result<Value, Self::ParsingError> {
        let text = Self::eat(&mut self.lexer, 0)?.trim().to_string();
        Ok(Value(text))
    }

    fn eat<'a: 't, 'b>(
        lexer: &mut Lexer<'a, 'b>,
        mut depth: usize,
    ) -> Result<String, Self::ParsingError> {
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
                    depth += 1;
                }

                Token::ParenthesisBlock => {
                    text.push_str("(");
                    lexer
                        .parse_nested_block(|inner_lexer| {
                            let mut inner_text = Self::eat(inner_lexer, depth).unwrap();
                            inner_text.push_str(")");
                            text.push_str(&inner_text.to_string());
                            Ok::<(), ParseError<'a, BasicParseError<'a>>>(())
                        })
                        .unwrap();
                }
                Token::SquareBracketBlock => {
                    text.push_str("[");
                    lexer
                        .parse_nested_block(|inner_lexer| {
                            let mut inner_text = Self::eat(inner_lexer, depth).unwrap();
                            inner_text.push_str("]");
                            text.push_str(&inner_text.to_string());
                            Ok::<(), ParseError<'a, BasicParseError<'a>>>(())
                        })
                        .unwrap();
                }
                Token::Function(value) => {
                    text.push_str(&format!("{value}("));
                    lexer
                        .parse_nested_block(|inner_lexer| {
                            let mut inner_text = Self::eat(inner_lexer, depth).unwrap();
                            inner_text.push_str(")");
                            text.push_str(&inner_text.to_string());
                            Ok::<(), ParseError<'a, BasicParseError<'a>>>(())
                        })
                        .unwrap();
                }
                _ => {}
            }
        }

        Ok(text)
    }
}
