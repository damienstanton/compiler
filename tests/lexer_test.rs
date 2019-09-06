use compiler::{
    lexer::Lexer,
    token::*
};

struct Test {
    e: TokenType,
    t: char,
}

const INPUT: &str = "=+(){},;";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delimiters() {
        let data: Vec<Test> = vec![
            Test { e: ASSIGN, t: '=' },
            Test { e: PLUS, t: '+' },
            Test { e: LPAREN, t: '(' },
            Test { e: RPAREN, t: ')' },
            Test { e: LBRACE, t: '{' },
            Test { e: RBRACE, t: '}' },
            Test { e: COMMA, t: ',' },
            Test {
                e: SEMICOLON,
                t: ';',
            },
            Test { e: EOF, t: '\0' },
        ];

        let mut lex = Lexer::new(INPUT);
        for d in data {
            let tok = lex.next_token();
            assert_eq!(tok.token_type, d.e);
            assert_eq!(tok.literal, d.t);
        }
    }
}
