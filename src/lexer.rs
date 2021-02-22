#[derive(Debug)]
pub enum Token {
    Word(String),
    Number(u32),
}

pub struct Lexer {}

impl Lexer {
    pub fn lex(input: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut number = 0;
        let mut word = String::new();
        for i in input.chars() {
            if i.is_digit(10) {
                if word != "" {
                    tokens.push(Token::Word(word));
                    word = "".to_string();
                }
                number = number * 10 + i.to_digit(10).unwrap();
            }
            // Ignoring spaces and \n
            else if i == ' ' || i == '\n' {
                if word != "" {
                    tokens.push(Token::Word(word));
                    word = "".to_string();
                }
                if number != 0 {
                    tokens.push(Token::Number(number));
                    number = 0;
                }
            }
            // i is word
            else {
                if number != 0 {
                    tokens.push(Token::Number(number));
                    number = 0;
                }
                word.push(i);
            }
        }
        if word != "" {
            tokens.push(Token::Word(word));
        }
        if number != 0 {
            tokens.push(Token::Number(number));
        }
        tokens
    }
}
