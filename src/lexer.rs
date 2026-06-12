use crate::error::PunjabiError;
use crate::token::{Token, TokenKind};

pub struct Lexer {
    chars: Vec<char>,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, PunjabiError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let line = self.line;
            let column = self.column;
            let character = self.advance();

            match character {
                '(' => tokens.push(Token::new(TokenKind::LeftParen, line, column)),
                ')' => tokens.push(Token::new(TokenKind::RightParen, line, column)),
                '{' => tokens.push(Token::new(TokenKind::LeftBrace, line, column)),
                '}' => tokens.push(Token::new(TokenKind::RightBrace, line, column)),
                ',' => tokens.push(Token::new(TokenKind::Comma, line, column)),
                ';' => tokens.push(Token::new(TokenKind::Semicolon, line, column)),
                '+' => tokens.push(Token::new(TokenKind::Plus, line, column)),
                '-' => tokens.push(Token::new(TokenKind::Minus, line, column)),
                '*' => tokens.push(Token::new(TokenKind::Star, line, column)),
                '%' => tokens.push(Token::new(TokenKind::Percent, line, column)),
                '=' => {
                    let kind = if self.match_char('=') {
                        TokenKind::EqualEqual
                    } else {
                        TokenKind::Equal
                    };
                    tokens.push(Token::new(kind, line, column));
                }
                '!' => {
                    if self.match_char('=') {
                        tokens.push(Token::new(TokenKind::BangEqual, line, column));
                    } else {
                        return Err(PunjabiError::lex(
                            line,
                            column,
                            "Sirf '!=' supported ae. Logical not layi 'nai' use karo.",
                        ));
                    }
                }
                '>' => {
                    let kind = if self.match_char('=') {
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    };
                    tokens.push(Token::new(kind, line, column));
                }
                '<' => {
                    let kind = if self.match_char('=') {
                        TokenKind::LessEqual
                    } else {
                        TokenKind::Less
                    };
                    tokens.push(Token::new(kind, line, column));
                }
                '/' => {
                    if self.match_char('/') {
                        self.skip_comment();
                    } else {
                        tokens.push(Token::new(TokenKind::Slash, line, column));
                    }
                }
                '#' => self.skip_comment(),
                '"' => tokens.push(Token::new(self.string(line, column)?, line, column)),
                '\n' => tokens.push(Token::new(TokenKind::Newline, line, column)),
                ' ' | '\r' | '\t' => {}
                character if character.is_ascii_digit() => {
                    tokens.push(Token::new(self.number(character, line, column)?, line, column));
                }
                character if is_identifier_start(character) => {
                    tokens.push(Token::new(self.identifier(character), line, column));
                }
                _ => {
                    return Err(PunjabiError::lex(
                        line,
                        column,
                        format!("Unknown character '{character}'."),
                    ));
                }
            }
        }

        tokens.push(Token::new(TokenKind::Eof, self.line, self.column));
        Ok(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn advance(&mut self) -> char {
        let character = self.chars[self.current];
        self.current += 1;
        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        character
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.current + 1).copied()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() != Some(expected) {
            return false;
        }
        self.advance();
        true
    }

    fn skip_comment(&mut self) {
        while let Some(character) = self.peek() {
            if character == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn string(&mut self, line: usize, column: usize) -> Result<TokenKind, PunjabiError> {
        let mut value = String::new();

        while let Some(character) = self.peek() {
            if character == '"' {
                self.advance();
                return Ok(TokenKind::String(value));
            }
            value.push(self.advance());
        }

        Err(PunjabiError::lex(
            line,
            column,
            "String close nai hoi. Double quote (\") lagao.",
        ))
    }

    fn number(
        &mut self,
        first: char,
        line: usize,
        column: usize,
    ) -> Result<TokenKind, PunjabiError> {
        let mut value = String::from(first);

        while let Some(character) = self.peek() {
            if character.is_ascii_digit() {
                value.push(self.advance());
            } else {
                break;
            }
        }

        if self.peek() == Some('.') && matches!(self.peek_next(), Some(c) if c.is_ascii_digit()) {
            value.push(self.advance());
            while let Some(character) = self.peek() {
                if character.is_ascii_digit() {
                    value.push(self.advance());
                } else {
                    break;
                }
            }
        }

        value.parse::<f64>().map(TokenKind::Number).map_err(|_| {
            PunjabiError::lex(line, column, format!("Number '{value}' parse nai hoya."))
        })
    }

    fn identifier(&mut self, first: char) -> TokenKind {
        let mut value = String::from(first);

        while let Some(character) = self.peek() {
            if is_identifier_part(character) {
                value.push(self.advance());
            } else {
                break;
            }
        }

        match value.as_str() {
            "rakho" => TokenKind::Rakho,
            "likho" => TokenKind::Likho,
            "je" => TokenKind::Je,
            "nahi_ta" | "nai_ta" => TokenKind::NahiTa,
            "jad_tak" | "jadd_tak" => TokenKind::JadTak,
            "sach" => TokenKind::Sach,
            "jhooth" => TokenKind::Jhooth,
            "te" => TokenKind::Te,
            "ya" => TokenKind::Ya,
            "nahi" | "nai" => TokenKind::Nahi,
            _ => TokenKind::Identifier(value),
        }
    }
}

fn is_identifier_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn is_identifier_part(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_keywords_and_literals() {
        let tokens = Lexer::new("rakho x = 5\njadd_tak x < 6 { likho \"sat sri akal\" }")
            .scan_tokens()
            .unwrap();

        assert!(tokens.iter().any(|token| token.kind == TokenKind::Rakho));
        assert!(tokens.iter().any(|token| token.kind == TokenKind::Likho));
        assert!(tokens.iter().any(|token| token.kind == TokenKind::JadTak));
        assert!(tokens
            .iter()
            .any(|token| token.kind == TokenKind::String("sat sri akal".to_string())));
    }
}
