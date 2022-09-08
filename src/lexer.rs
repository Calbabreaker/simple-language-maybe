use crate::{Error, ErrorKind, Position};
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str),
    Integer(i64),
    Float(f64),
    String(&'a str),
    True,
    False,
    Func,
    Equal,
    Space,
    LineBreak,
    OpenBracket,
    CloseBracket,
    EOF,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub tokens: Vec<(Token<'a>, Position)>,
}

impl<'a> Lexer<'a> {
    pub fn parse(code: &'a str) -> crate::Result<Self> {
        let mut tokens = Vec::new();
        let mut line = 0;
        let mut column = 0;

        let mut chars = code.chars().enumerate().peekable();
        while let Some((i, char)) = chars.next() {
            let mut token_length = 1;
            let mut position = Position::new(line, (column, column));

            macro_rules! skip_char {
                () => {
                    chars.next();
                    token_length += 1;
                    position.columns.1 += 1;
                };
            }

            let token = match char {
                '=' => Token::Equal,
                '\n' => {
                    line += 1;
                    column = 0;
                    Token::LineBreak
                }
                '(' => Token::OpenBracket,
                ')' => Token::CloseBracket,
                '#' => {
                    if peek_char(&mut chars) == '*' {
                        skip_char!();
                        while peek_test(&mut chars, |char| char != '*')
                            && next_char(&mut chars) != '#'
                        {
                            skip_char!();
                        }
                    } else {
                        while peek_test(&mut chars, |char| char != '\n') {
                            skip_char!();
                        }
                    }

                    continue;
                }
                char if is_space(char) => {
                    while is_space(peek_char(&mut chars)) {
                        skip_char!();
                    }

                    Token::Space
                }
                char if char.is_alphabetic() => {
                    while peek_char(&mut chars).is_alphabetic() {
                        skip_char!();
                    }

                    match &code[i..i + token_length] {
                        "true" => Token::True,
                        "false" => Token::False,
                        "func" => Token::Func,
                        sub => Token::Identifier(sub),
                    }
                }
                char if char.is_digit(10) => {
                    let mut is_float = false;
                    loop {
                        let char = peek_char(&mut chars);
                        if char == '.' && !is_float {
                            is_float = true;
                        } else if !char.is_digit(10) {
                            break;
                        }
                        skip_char!();
                    }

                    let sub = &code[i..i + token_length];
                    if is_float {
                        Token::Float(sub.parse().unwrap())
                    } else {
                        Token::Integer(sub.parse().unwrap())
                    }
                }
                _ => Err(Error::new(ErrorKind::InvalidToken, position))?,
            };

            tokens.push((token, position));
            column += token_length;
        }

        Ok(Self { tokens })
    }
}

type CharsIter<'a> = Peekable<Enumerate<Chars<'a>>>;

fn peek_char(chars: &mut CharsIter) -> char {
    chars.peek().unwrap_or(&(0, '\0')).1
}

fn peek_test(chars: &mut CharsIter, test_func: fn(char) -> bool) -> bool {
    chars.peek().map_or(false, |(_, char)| test_func(*char))
}

fn next_char(chars: &mut CharsIter) -> char {
    chars.next().unwrap_or((0, '\0')).1
}

fn is_space(char: char) -> bool {
    char.is_whitespace() && char != '\n'
}
