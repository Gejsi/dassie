mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::{Declaration, Parse, Parser};
    use cssparser::{Parser as Lexer, ParserInput as LexerInput};

    #[test]
    fn it_works() {
        let input = "border-color: hsl(var(--b2) / var(--tw-border-opacity));";
        let mut lexer_input = LexerInput::new(&input);
        let lexer = Lexer::new(&mut lexer_input);
        let mut parser = Parser::new(lexer);
        let _ = parser.parse_declaration().unwrap();
    }
}
