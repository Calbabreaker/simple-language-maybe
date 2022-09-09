use crate::{Error, ErrorKind, Position};
use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str),
    Integer(i64),
    Float(f64),
    String(&'a str),
    True,
    False,
    Function,
    Equal,
    ConstantEqual,
    Ignore,
    LineBreak,
    OpenBracket,
    CloseBracket,
    EOF,
}

pub struct Lexer<'a> {
    pub tokens: Vec<(Token<'a>, Position)>,
    chars: Peekable<Chars<'a>>,
    char_i: usize,
    line: usize,
    column: usize,
    token_length: usize,
}

impl<'a> Lexer<'a> {
    pub fn parse(code: &'a str) -> crate::Result<Self> {
        let mut lexer = Self {
            tokens: Vec::new(),
            chars: code.chars().peekable(),
            line: 0,
            column: 0,
            token_length: 0,
            char_i: 0,
        };

        loop {
            match lexer.next_token(code) {
                Err(err) => Err(Error::new(err, lexer.position()))?,
                Ok(Token::Ignore) => (),
                Ok(token) => {
                    lexer.tokens.push((token, lexer.position()));
                    match token {
                        Token::EOF => break,
                        Token::LineBreak => {
                            lexer.line += 1;
                            lexer.column = 0;
                            continue;
                        }
                        _ => (),
                    }
                }
            };

            lexer.column += lexer.token_length
        }

        Ok(lexer)
    }

    fn next_token<'b>(&mut self, code: &'b str) -> Result<Token<'b>, ErrorKind> {
        self.token_length = 0;
        let char = self.next_char();
        let token = match char {
            '=' => Token::Equal,
            _ if char == ':' && self.next_char() == '=' => Token::ConstantEqual,
            '\n' => Token::LineBreak,
            '(' => Token::OpenBracket,
            ')' => Token::CloseBracket,
            '#' => {
                if self.peek_char() == '*' {
                    // Is multiline comment
                    self.next_char();
                    while self.next_check(|c| c != '*') || self.next_check(|c| c != '#') {}
                } else {
                    while self.peek_check(|c| c != '#') {
                        self.next_char();
                    }
                }
                Token::Ignore
            }
            '"' => {
                loop {
                    let char = self.next_char();
                    if char == '"' {
                        break;
                    } else if char == '\n' || char == '\0' {
                        self.token_length -= 1;
                        Err(ErrorKind::Unmatched("quotes"))?
                    };
                }

                Token::String(self.get_substr(code, 1))
            }
            _ if char.is_alphabetic() => {
                while self.peek_char().is_alphabetic() {
                    self.next_char();
                }

                match self.get_substr(code, 0) {
                    "true" => Token::True,
                    "false" => Token::False,
                    "func" => Token::Function,
                    sub => Token::Identifier(sub),
                }
            }
            _ if char.is_digit(10) => {
                let mut is_float = false;
                loop {
                    let char = self.peek_char();
                    if char == '.' && !is_float {
                        is_float = true;
                    } else if !char.is_digit(10) {
                        break;
                    }
                    self.next_char();
                }

                let sub = self.get_substr(code, 0);
                if is_float {
                    Token::Float(sub.parse().unwrap())
                } else {
                    Token::Integer(sub.parse().unwrap())
                }
            }
            _ if is_space(char) => Token::Ignore,
            '\0' => Token::EOF,
            _ => Err(ErrorKind::InvalidToken)?,
        };

        Ok(token)
    }

    fn next_char(&mut self) -> char {
        self.token_length += 1;
        self.char_i += 1;
        self.chars.next().unwrap_or('\0')
    }

    fn position(&self) -> Position {
        Position {
            line: self.line,
            columns: (self.column, self.column + self.token_length - 1),
        }
    }

    fn next_check(&mut self, check_func: fn(char) -> bool) -> bool {
        let char = self.next_char();
        char != '\0' && check_func(char)
    }

    fn peek_char(&mut self) -> char {
        *self.chars.peek().unwrap_or(&'\0')
    }

    fn peek_check(&mut self, check_func: fn(char) -> bool) -> bool {
        self.chars.peek().copied().map_or(false, check_func)
    }

    fn get_substr<'b>(&self, code: &'b str, bounds_length: usize) -> &'b str {
        &code[(self.char_i - self.token_length + bounds_length)..(self.char_i - bounds_length)]
    }
}

fn is_space(char: char) -> bool {
    char.is_whitespace() && char != '\n'
}
