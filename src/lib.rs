mod nodes;
mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::{Parse, Parser};
    use cssparser::{Parser as Lexer, ParserInput as LexerInput};

    #[test]
    fn parse_value() {
        let input = r#"
center / contain no-repeat
      url('../../media/examples/firefox-logo.svg'),
    #eee 35% url('../../media/examples/lizard.png')
        "#;
        let mut lexer_input = LexerInput::new(&input);
        let mut lexer = Lexer::new(&mut lexer_input);
        Parser::parse_value(&mut lexer).unwrap();
    }

    #[test]
    fn parse_declaration() {
        let input = "border-color: hsl(var(--b2) / var(--tw-border-opacity));";
        let mut lexer_input = LexerInput::new(&input);
        let mut lexer = Lexer::new(&mut lexer_input);
        Parser::parse_declaration(&mut lexer).unwrap();
    }

    #[test]
    fn parse_declaration_block() {
        let input = "{ border-color: red; color: red; }";
        let mut lexer_input = LexerInput::new(&input);
        let mut lexer = Lexer::new(&mut lexer_input);
        Parser::parse_declaration_block(&mut lexer).unwrap();
    }

    #[test]
    fn parse_selector() {
        let input = "input {}";
        let mut lexer_input = LexerInput::new(&input);
        let mut lexer = Lexer::new(&mut lexer_input);
        Parser::parse_selector(&mut lexer).unwrap();
    }
}
