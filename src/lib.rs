mod nodes;
mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::{Parse, Parser};
    use cssparser::{Parser as Lexer, ParserInput as LexerInput};

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
}
