use std::iter::{Enumerate, Peekable};
use unicode_segmentation::{Graphemes, UnicodeSegmentation as us};
fn main() {
    println!("Hello, world!");

    let keywords = [(
        TokenKind::Keyword,
        vec![vec!["function", "fn", "fun"], vec!["struct"]],
    )];
    // let tokens: Vec<Token> = keywords
    //     .iter()
    //     .flat_map(|kind| {
    //         kind.1.iter().map(|x| Token {
    //             magic: x.clone(),
    //             token_kind: kind.0,
    //         })
    //     })
    //     .collect();
    lexical_analysis("");
}
/// The kind of our token. This will decide what kind of operation the token is requiring.
#[derive(Clone, Copy, Debug)]
enum TokenKind {
    Operator,
    Keyword,
    Identifier,
}
#[derive(Clone, Debug)]
enum Magic<'a> {
    Operator(&'a str),
    Keyword(Vec<&'a str>),
    Identifier(&'a str),
}
/// Our representation of a lexical token. It can be divided in different token kinds :
/// `TokenKind::Operator, Keyword, Identifier`.
#[derive(Clone, Debug)]
struct Token<'a> {
    magic: Magic<'a>,
    token_kind: TokenKind,
}
//
// struct Function;
// struct Loops;
// struct Struct;
const FUNCTION_IDS: [&str; 3] = ["function", "fun", "fn"];
type Tokens<'a> = Vec<Token<'a>>;
/// The question is, would it be inherently worse or better to operate on a streamed buffer ?
/// The string property allows us to look ahead and behind, which is nice.
fn lexical_analysis(content: &str) -> Tokens {
    for (row, line) in content.lines().enumerate() {
        let max_index = line.len();
        let mut iterator = us::graphemes(line, true).enumerate().peekable();
        loop {
            let next = iterator.next();
            if let Some((col, grapheme)) = next {
                match_grapheme(grapheme, col, row, max_index, &mut iterator);
            } else {
                break;
            }
        }
        // for (col, grapheme) in iterator {
        //     iterator.next();
        //     match_grapheme(grapheme, col, row, max_index);
        // }
    }
    vec![]
}

fn match_grapheme(
    grapheme: &str,
    col: usize,
    _row: usize,
    max: usize,
    iterator: &mut Peekable<Enumerate<Graphemes>>,
) {
    match grapheme {
        "f" => {
            if let Some(char) = iterator.peek() {
                match char.1 {
                    "n" => {
                        if let Some(char) = iterator.peek();

                    }
                    "u" => {}
                    _ => {}
                }
            }
        }
        "s" => {}
        _ => {}
    };
}

// Steps :

// Lexical Analysis: The first phase, where the source code is broken down into tokens
// such as keywords, operators, and identifiers for easier processing.

// Syntax Analysis or Parsing: This phase checks if the source code follows the correct syntax rules,
// building a parse tree or abstract syntax tree (AST).

// Semantic Analysis: It ensures the program’s logic makes sense, checking for errors like type mismatches or undeclared variables.

// Intermediate Code Generation: In this phase, the compiler converts the source code into an intermediate,
// machine-independent representation, simplifying optimization and translation.

// Code Optimization: This phase improves the intermediate code to make it run more efficiently,
// reducing resource usage or increasing speed.

// Target Code Generation: The final phase where the optimized code is translated into the target machine code
// or assembly language that can be executed on the computer.
