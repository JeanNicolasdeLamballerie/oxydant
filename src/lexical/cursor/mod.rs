// use std::iter::Peekable;
mod token;
// use unicode_segmentation::{Graphemes, UnicodeSegmentation};
//
//
/// Re-exports :
pub use token::{Token, Tokens};
//
//
//
//
//
//
use token::{parse_token, Length};
pub struct Cursor<'input_lifetime> {
    input: &'input_lifetime str,
    input_len: usize,
    // graphemes: Peekable<Graphemes<'input_lifetime>>,
    pos: usize,
}

impl<'input_lifetime> Cursor<'input_lifetime> {
    pub fn new(input: &'input_lifetime str) -> Self {
        Self {
            input,
            input_len: input.len(),
            // graphemes: input.graphemes(true).peekable(),
            pos: 0,
        }
    }

    pub fn read(&mut self) -> Vec<Token> {
        // let mut cursor: Cursor = Cursor::new(input);
        let cursor = self;
        let mut tokens: Vec<Token> = vec![];
        while cursor.pos < cursor.input_len {
            let token = parse_token(cursor);
            cursor.advance(token.length());
            tokens.push(token);
        }
        tokens
    }

    pub fn position(&self) -> usize {
        self.pos
    }
    // pub fn next_whitespace(&self) -> bool {
    //     " " == self.extract_substring(self.pos, self.pos + 1)
    // }
    // fn next(&mut self) -> Option<&'input_lifetime str> {
    //     let grapheme = self.graphemes.next()?;
    //     self.pos += grapheme.len(); // Advance position by grapheme length
    //     Some(grapheme)
    // }
    //
    // fn peek(&mut self) -> Option<&&'input_lifetime str> {
    //     self.graphemes.peek()
    // }
    /// Advances the cursor's position by `len`.
    pub fn advance(&mut self, len: usize) {
        self.pos += len;
    }
    // fn nth(&mut self, n :usize) -> Option<&&'input_lifetime str> {
    //     self.graphemes.nth(n)
    // }

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

// pub fn read_with_cursor(input: &str) -> Vec<Token> {
//     let mut cursor: Cursor = Cursor::new(input);
//     let mut tokens: Vec<Token> = vec![];
//     while cursor.pos < cursor.input_len {
//         let token = parse_token(&cursor);
//         cursor.advance(token.length());
//         tokens.push(token);
//     }
//     tokens
// }
mod test {

    #[test]
    fn test_cursor() {
        use super::Cursor;
        let c = Cursor::new("0123456");
        let d = Cursor::new("function FncName");
        let s = c.extract_substring(6, 7);
        let s2 = d.extract_substring(0, 1);
        let s3 = c.extract_substring(0, 7);
        let s4 = d.extract_substring(0, 8);
        assert!(s == "6");
        assert!(s2 == "f");
        assert!(s3 == "0123456");
        assert!(s4 == "function");
        // Cursor::new("struct FncName");
    }
    #[test]
    fn read_text() {
        use super::token::{Keyword, Operator, Token};
        use crate::lexical::cursor::Cursor;
        let input = "fn + function - struct";
        let mut crs = Cursor::new(input);
        let tokens = crs.read();
        let tokens_verif = vec![
            Token::Keyword(Keyword::Function(0, 2)),
            Token::Whitespace(2, 3),
            Token::Operator(Operator::Add(3, 4)),
            Token::Whitespace(4, 5),
            Token::Keyword(Keyword::Function(5, 13)),
            Token::Whitespace(13, 14),
            Token::Operator(Operator::Substract(14, 15)),
            Token::Whitespace(15, 16),
            Token::Keyword(Keyword::Struct(16, 22)),
        ];
        assert!(
            tokens == tokens_verif,
            "The tokens were not properly parsed."
        );
    }
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
