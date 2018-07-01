// Copyright 2014-2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::iter::Peekable;
use types::CoordType;
use std::marker::PhantomData;

#[derive(PartialEq, Debug)]
pub enum Token<T>
where
    T: CoordType,
{
    Comma,
    Number(T),
    ParenClose,
    ParenOpen,
    Word(String),
}

fn is_whitespace(c: char) -> bool {
    match c {
        '\n' | '\r' | '\t' | ' ' => true,
        _ => false,
    }
}

fn is_numberlike(c: char) -> bool {
    match c {
        c if c.is_numeric() => true,
        '.' | '-' | '+' => true,
        _ => false,
    }
}

pub type PeekableTokens<T> = Peekable<Tokens<T>>;

pub struct Tokens<T>
where T: CoordType,
{
    _m: PhantomData<T>,
    text: String,
}

impl<T: CoordType> Tokens<T> {
    pub fn from_str(input: &str) -> Self {
        Tokens {
            _m: PhantomData,
            text: input.to_string(),
        }
    }
}

impl<T: CoordType> Iterator for Tokens<T> {
    type Item = Token<T>;

    fn next(&mut self) -> Option<Token<T>> {
        // TODO: should this return Result?
        let next_char = match self.pop_front() {
            Some(c) => c,
            None => return None,
        };

        match next_char {
            '\0' => None,
            '(' => Some(Token::ParenOpen),
            ')' => Some(Token::ParenClose),
            ',' => Some(Token::Comma),
            c if is_whitespace(c) => self.next(),
            c if is_numberlike(c) => {
                let mut number = c.to_string() + &self.read_until_whitespace();
                number = number.trim_left_matches('+').to_string();
                match number.parse::<T>() {
                    Ok(parsed_num) => Some(Token::Number(parsed_num)),
                    Err(_) => panic!("Could not parse number: {}", c),
                }
            }
            c => {
                let word = c.to_string() + &self.read_until_whitespace();
                Some(Token::Word(word))
            }
        }
    }
}

impl<T: CoordType> Tokens<T> {
    fn pop_front(&mut self) -> Option<char> {
        match self.text.is_empty() {
            true => None,
            false => Some(self.text.remove(0)),
        }
    }

    fn read_until_whitespace(&mut self) -> String {
        let next_char = match self.pop_front() {
            Some(c) => c,
            None => return "".to_string(),
        };

        match next_char {
            '\0' | '(' | ')' | ',' => {
                self.text.insert(0, next_char);
                "".to_string()
            }
            c if is_whitespace(c) => "".to_string(),
            _ => next_char.to_string() + &self.read_until_whitespace(),
        }
    }
}

#[test]
fn test_tokenizer_empty() {
    let test_str = "";
    let tokens: Vec<Token<f64>> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![]);
}

#[test]
fn test_tokenizer_1word() {
    let test_str = "hello";
    let tokens: Vec<Token<f64>> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![Token::Word("hello".to_string())]);
}

#[test]
fn test_tokenizer_2words() {
    let test_str = "hello world";
    let tokens: Vec<Token<f64>> = Tokens::from_str(test_str).collect();
    assert_eq!(
        tokens,
        vec![
            Token::Word("hello".to_string()),
            Token::Word("world".to_string()),
        ]
    );
}

#[test]
fn test_tokenizer_1number() {
    let test_str = "4.2";
    let tokens: Vec<Token<f64>> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![Token::Number(4.2)]);
}

#[test]
fn test_tokenizer_1number_plus() {
    let test_str = "+4.2";
    let tokens: Vec<Token<f64>> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![Token::Number(4.2)]);
}

#[test]
fn test_tokenizer_2numbers() {
    let test_str = ".4 -2";
    let tokens: Vec<Token<f64>> = Tokens::from_str(test_str).collect();
    assert_eq!(tokens, vec![Token::Number(0.4), Token::Number(-2.0)]);
}

#[test]
fn test_tokenizer_point() {
    let test_str = "POINT (10 -20)";
    let tokens: Vec<Token<f64>> = Tokens::from_str(test_str).collect();
    assert_eq!(
        tokens,
        vec![
            Token::Word("POINT".to_string()),
            Token::ParenOpen,
            Token::Number(10.0),
            Token::Number(-20.0),
            Token::ParenClose,
        ]
    );
}
