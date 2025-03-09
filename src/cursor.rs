use std::iter::Peekable;

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

use crate::token::{parse_token, Keyword, Token};

pub struct Cursor<'input_lifetime> {
    input: &'input_lifetime str,
    input_len: usize,
    graphemes: Peekable<Graphemes<'input_lifetime>>,
    pos: usize,
}

impl<'input_lifetime> Cursor<'input_lifetime> {
    pub fn new(input: &'input_lifetime str) -> Self {
        Self {
            input,
            input_len: input.len(),
            graphemes: input.graphemes(true).peekable(),
            pos: 0,
        }
    }
    pub fn position(&self) -> usize {
        self.pos
    }
    fn next(&mut self) -> Option<&'input_lifetime str> {
        let grapheme = self.graphemes.next()?;
        self.pos += grapheme.len(); // Advance position by grapheme length
        Some(grapheme)
    }

    fn peek(&mut self) -> Option<&&'input_lifetime str> {
        self.graphemes.peek()
    }

    /// This function is INCLUSIVE of start and NON-INCLUSIVE of end. E.g, `start = x, end = x`
    /// will always return an empty string.
    /// On the other hand, `start=x, end = x+1` will always return the char at position x.
    pub fn extract_substring(&self, start: usize, end: usize) -> &'input_lifetime str {
        //NOTE we could return an error here instead.
        if start > self.input_len || end > self.input_len {
            return "";
        }
        &self.input[start..end]
    }
}

pub fn some(input: &str) -> &str {
    let mut cursor = Cursor::new(input);
    cursor.peek();
    cursor.next();
    let extracted = cursor.extract_substring(cursor.pos, cursor.pos + 1);
    println!("extracted : {}", extracted);
    extracted
}

#[test]
fn test_cursor() {
    Cursor::new("Some Input Here");
    let c = Cursor::new("0123456");
    let d = Cursor::new("function FncName");
    let s = c.extract_substring(6, 7);
    let s2 = d.extract_substring(0, 1);
    println!("First string : {}, Second string {}", s, s2);
    dbg!(s.len(), s2.len());
    Cursor::new("struct FncName");
}
// #[test]
// fn test_tokens_functions() {
//     let one = Cursor::new("fn FncName");
//     let two = Cursor::new("function FncName");
//     let three = Cursor::new("function");
//     let four = Cursor::new("FncName");
//     let result_1 = parse_token(&one);
//     let result_2 = parse_token(&two);
//     let result_3 = parse_token(&three);
//     let result_4 = parse_token(&four);
//     assert!(result_1 == Token::Keyword(Keyword::Function(0, 2)));
//     assert!(result_2 == Token::Keyword(Keyword::Function(0, 8)));
//     assert!(result_3 == Token::Keyword(Keyword::Function(0, 8)));
//     assert!(result_4 == Token::Unknown("..."));
// }
// #[test]
// fn test_tokens_functions() {
//     let one = Cursor::new("st strName");
//     let two = Cursor::new("struct FncName");
//     let three = Cursor::new("struct");
//     let four = Cursor::new("structuration");
//     let result_1 = parse_token(&one);
//     let result_2 = parse_token(&two);
//     let result_3 = parse_token(&three);
//     let result_4 = parse_token(&four);
//     assert!(result_1 == Token::Keyword(Keyword::Function(0, 2)));
//     assert!(result_2 == Token::Keyword(Keyword::Function(0, 8)));
//     assert!(result_3 == Token::Keyword(Keyword::Function(0, 8)));
//     assert!(result_4 == Token::Unknown("..."));
// }
