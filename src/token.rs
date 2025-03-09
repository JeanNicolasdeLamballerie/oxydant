use crate::cursor::Cursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'input_lifetime> {
    /// Will contain all of "function", "struct", "let"
    Keyword(Keyword),
    /// Will contain all of +, =, ==, !=
    Operator(&'input_lifetime str),
    /// Will contain all of {}, (), []
    Scope(&'input_lifetime str),
    /// Any leftover string (for now)
    Unknown(&'input_lifetime str),
}

type Transformator<'a> = fn(usize, usize) -> Token<'a>;
type TransformatorTuple<'a> = (&'static [&'static str], Transformator<'a>);
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Function(usize, usize),
    Struct(usize, usize),
    /// There is only one way to write let, so we shouldn't need to have two positions.
    Let(usize, usize),
}
/// It would be nice to be able to say
/// "this is a constant array who should live nearby in memory"
/// but it is functionally differentiated in 3 separate arrays.
/// (This is likely possible with memory arenas)
impl Keyword {
    const FUNC_STRINGS: [&'static str; 3] = ["function", "fn", "fun"];
    const STRUCT_STRINGS: [&'static str; 2] = ["struct", "st"];
    const LET_STRINGS: [&'static str; 1] = ["let"];

    const ALL_KW: [&'static TransformatorTuple<'_>; 3] =
        [&Self::FN_STR, &Self::ST_STR, &Self::LET_STR];
    const FN_STR: TransformatorTuple<'_> = (&Self::FUNC_STRINGS, Self::to_keyword_function);
    const ST_STR: TransformatorTuple<'_> = (&Self::STRUCT_STRINGS, Self::to_keyword_struct);
    const LET_STR: TransformatorTuple<'_> = (&Self::LET_STRINGS, Self::to_keyword_let);
    fn to_keyword_function<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Keyword(Keyword::Function(start, end))
    }
    fn to_keyword_struct<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Keyword(Keyword::Struct(start, end))
    }
    fn to_keyword_let<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Keyword(Keyword::Let(start, end))
    }
}
pub fn parse_token<'a>(
    cursor: &'a Cursor<'a>,
    transform: &'a [&'a TransformatorTuple<'a>],
) -> Token<'a> {
    let previous = cursor.position();
    let iterator = transform.iter();
    for (references, to) in iterator {
        if let Some(token) = parse_with_strings(references, cursor, previous, to) {
            return token;
        }
    }
    Token::Unknown("...")
}

fn parse_with_strings<'a>(
    references: &[&'static str],
    cursor: &Cursor<'a>,
    previous: usize,
    to: &Transformator<'a>,
) -> Option<Token<'a>> {
    for &id in references {
        let len = id.len();
        if id == cursor.extract_substring(previous, previous + len) {
            match cursor.extract_substring(previous + len, previous + len + 1) {
                "" | " " => {
                    return Some(to(previous, previous + len));
                }
                _ => continue,
            };
        };
    }
    None
}
#[test]
fn name() {
    let c = Cursor::new("let");
    let token1 = parse_token(&c, &Keyword::ALL_KW);
    let c = Cursor::new("fn");
    let token2 = parse_token(&c, &Keyword::ALL_KW);
    let c = Cursor::new("struct");
    let token3 = parse_token(&c, &Keyword::ALL_KW);
    dbg!(token1, token2, token3);
}
