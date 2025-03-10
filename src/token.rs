use crate::cursor::Cursor;

type Transformator<'a> = fn(usize, usize) -> Token<'a>;

type TransformatorTuple<'a> = (&'static [&'static str], Transformator<'a>, Marker);

pub trait Length {
    fn length(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'input_lifetime> {
    /// Will contain all of "function", "struct", "let"
    Keyword(Keyword),
    /// Will contain all of +, =, ==, !=
    Operator(Operator),
    /// Will contain all of {}, (), []
    Scope(Scope),
    /// Any leftover string (for now)
    Unknown(&'input_lifetime str),
    /// Whitespace.
    Whitespace(usize, usize),
}
impl Length for Token<'_> {
    fn length(&self) -> usize {
        match self {
            Token::Keyword(keyword) => keyword.length(),
            Token::Operator(operator) => operator.length(),
            Token::Scope(_) => 1,
            Token::Unknown(str) => str.len(),
            Token::Whitespace(start, end) => end - start,
        }
    }
}

impl Length for Keyword {
    fn length(&self) -> usize {
        match self {
            Keyword::Function(a, b) => b - a,
            Keyword::Struct(a, b) => b - a,
            Keyword::Let(a, b) => b - a,
        }
    }
}

impl Length for Operator {
    fn length(&self) -> usize {
        match self {
            Operator::Assign(a, b) => b - a,
            Operator::Equality(a, b) => b - a,
            Operator::Unequality(a, b) => b - a,
            Operator::And(a, b) => b - a,
            Operator::Or(a, b) => b - a,
            Operator::Add(a, b) => b - a,
            Operator::Substract(a, b) => b - a,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    /// (
    ParenthesisOpen(usize, usize),
    /// )
    ParenthesisClose(usize, usize),
    /// [
    SquareBracketOpen(usize, usize),
    /// ]
    SquareBracketClose(usize, usize),
    /// {
    BraceOpen(usize, usize),
    /// }
    BraceClose(usize, usize),
    /// <
    ChevronOpen(usize, usize),
    /// >
    ChevronClose(usize, usize),
}
impl Scope {
    const PAR_STRINGS: [&'static str; 1] = ["("];
    const PAR_C_STRINGS: [&'static str; 1] = [")"];
    const SQ_STRINGS: [&'static str; 1] = ["["];
    const SQ_C_STRINGS: [&'static str; 1] = ["]"];
    const BRACE_STRINGS: [&'static str; 1] = ["{"];
    const BRACE_C_STRINGS: [&'static str; 1] = ["}"];
    const CH_STRINGS: [&'static str; 1] = ["<"];
    const CH_C_STRINGS: [&'static str; 1] = [">"];
    const ALL_KW: [&'static TransformatorTuple<'_>; 8] = [
        &Self::PAR_TUPLE,
        &Self::PAR_C_TUPLE,
        &Self::SQ_TUPLE,
        &Self::SQ_C_TUPLE,
        &Self::BRACE_TUPLE,
        &Self::BRACE_C_TUPLE,
        &Self::CH_TUPLE,
        &Self::CH_C_TUPLE,
    ];
    const PAR_TUPLE: TransformatorTuple<'_> = (
        &Self::PAR_STRINGS,
        Self::to_scope_parenthesis,
        Marker::Ignore,
    );
    const PAR_C_TUPLE: TransformatorTuple<'_> = (
        &Self::PAR_C_STRINGS,
        Self::to_scope_closed_parenthesis,
        Marker::Ignore,
    );
    const SQ_TUPLE: TransformatorTuple<'_> = (
        &Self::SQ_STRINGS,
        Self::to_square_bracket_open,
        Marker::Ignore,
    );
    const SQ_C_TUPLE: TransformatorTuple<'_> = (
        &Self::SQ_C_STRINGS,
        Self::to_square_bracket_closed,
        Marker::Ignore,
    );
    const BRACE_TUPLE: TransformatorTuple<'_> =
        (&Self::BRACE_STRINGS, Self::to_brace_open, Marker::Ignore);
    const BRACE_C_TUPLE: TransformatorTuple<'_> = (
        &Self::BRACE_C_STRINGS,
        Self::to_brace_closed,
        Marker::Ignore,
    );
    const CH_TUPLE: TransformatorTuple<'_> =
        (&Self::CH_STRINGS, Self::to_chevron_open, Marker::Ignore);
    const CH_C_TUPLE: TransformatorTuple<'_> =
        (&Self::CH_C_STRINGS, Self::to_chevron_closed, Marker::Ignore);
    fn to_scope_parenthesis<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Scope(Scope::ParenthesisOpen(start, end))
    }
    fn to_scope_closed_parenthesis<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Scope(Scope::ParenthesisClose(start, end))
    }
    fn to_square_bracket_open<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Scope(Scope::SquareBracketOpen(start, end))
    }
    fn to_square_bracket_closed<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Scope(Scope::SquareBracketClose(start, end))
    }
    fn to_brace_open<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Scope(Scope::BraceOpen(start, end))
    }
    fn to_brace_closed<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Scope(Scope::BraceClose(start, end))
    }
    fn to_chevron_open<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Scope(Scope::ChevronOpen(start, end))
    }
    fn to_chevron_closed<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Scope(Scope::ChevronClose(start, end))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    /// =
    Assign(usize, usize),
    /// ==
    Equality(usize, usize),
    /// !=
    Unequality(usize, usize),
    /// &&
    And(usize, usize),
    /// ||
    Or(usize, usize),
    /// +
    Add(usize, usize),
    /// -
    Substract(usize, usize),
}
impl Operator {
    const ASSIGN_STRINGS: [&'static str; 1] = ["="];
    const EQ_STRINGS: [&'static str; 1] = ["=="];
    const UNEQ_STRINGS: [&'static str; 1] = ["!="];
    const AND_STRINGS: [&'static str; 1] = ["&&"];
    const OR_STRINGS: [&'static str; 1] = ["||"];
    const ADD_STRINGS: [&'static str; 1] = ["+"];
    const SUB_STRINGS: [&'static str; 1] = ["-"];

    const ALL_KW: [&'static TransformatorTuple<'_>; 7] = [
        &Self::ASSIGN_TUPLE,
        &Self::EQ_TUPLE,
        &Self::UNEQ_TUPLE,
        &Self::AND_TUPLE,
        &Self::OR_TUPLE,
        &Self::ADD_TUPLE,
        &Self::SUB_TUPLE,
    ];
    const ASSIGN_TUPLE: TransformatorTuple<'_> = (
        &Self::ASSIGN_STRINGS,
        Self::to_operator_assign,
        Marker::Ignore,
    );
    const EQ_TUPLE: TransformatorTuple<'_> = (
        &Self::EQ_STRINGS,
        Self::to_operator_equality,
        Marker::Ignore,
    );
    const UNEQ_TUPLE: TransformatorTuple<'_> = (
        &Self::UNEQ_STRINGS,
        Self::to_operator_unequality,
        Marker::Ignore,
    );
    const AND_TUPLE: TransformatorTuple<'_> =
        (&Self::AND_STRINGS, Self::to_operator_and, Marker::Ignore);
    const OR_TUPLE: TransformatorTuple<'_> =
        (&Self::OR_STRINGS, Self::to_operator_or, Marker::Ignore);
    const ADD_TUPLE: TransformatorTuple<'_> =
        (&Self::ADD_STRINGS, Self::to_operator_add, Marker::Ignore);
    const SUB_TUPLE: TransformatorTuple<'_> =
        (&Self::SUB_STRINGS, Self::to_operator_sub, Marker::Ignore);
    fn to_operator_assign<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Operator(Operator::Assign(start, end))
    }
    fn to_operator_equality<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Operator(Operator::Equality(start, end))
    }
    fn to_operator_unequality<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Operator(Operator::Unequality(start, end))
    }
    fn to_operator_and<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Operator(Operator::And(start, end))
    }
    fn to_operator_or<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Operator(Operator::Or(start, end))
    }
    fn to_operator_add<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Operator(Operator::Add(start, end))
    }
    fn to_operator_sub<'z>(start: usize, end: usize) -> Token<'z> {
        Token::Operator(Operator::Substract(start, end))
    }
}
// trait TokenAnalyzer {
//     fn keywords() -> &'static [&'static TransformatorTuple<'static>];
// }
// impl TokenAnalyzer for Keyword {
//     fn keywords() -> &'static [&'static TransformatorTuple<'static>] {
//         &Self::ALL_KW
//     }
// }
// impl TokenAnalyzer for Operator {
//     fn keywords() -> &'static [&'static TransformatorTuple<'static>] {
//         &Self::ALL_KW
//     }
// }
// impl TokenAnalyzer for Scope {
//     fn keywords() -> &'static [&'static TransformatorTuple<'static>] {
//         &Self::ALL_KW
//     }
// }
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
    const MARKERS: [&'static str; 2] = ["", " "];
    const ALL_KW: [&'static TransformatorTuple<'_>; 3] =
        [&Self::FN_STR, &Self::ST_STR, &Self::LET_STR];
    const FN_STR: TransformatorTuple<'_> = (
        &Self::FUNC_STRINGS,
        Self::to_keyword_function,
        Marker::Ruled(&Self::MARKERS),
    );
    const ST_STR: TransformatorTuple<'_> = (
        &Self::STRUCT_STRINGS,
        Self::to_keyword_struct,
        Marker::Ruled(&Self::MARKERS),
    );
    const LET_STR: TransformatorTuple<'_> = (
        &Self::LET_STRINGS,
        Self::to_keyword_let,
        Marker::Ruled(&Self::MARKERS),
    );
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

fn to_whitespace<'z>(start: usize, end: usize) -> Token<'z> {
    Token::Whitespace(start, end)
}
const IS_WHITESPACE: [&str; 1] = [" "];
const WHITESPACE: [&TransformatorTuple<'_>; 1] = [&(&IS_WHITESPACE, to_whitespace, Marker::Ignore)];

const KEYS: [&[&TransformatorTuple<'static>]; 4] = [
    &WHITESPACE,
    &Operator::ALL_KW,
    &Keyword::ALL_KW,
    &Scope::ALL_KW,
];

pub fn parse_token<'a>(cursor: &Cursor<'a>) -> Token<'a> {
    let previous = cursor.position();
    for transform in KEYS.iter() {
        for (references, to, marker) in transform.iter() {
            if let Some(token) = parse_with_strings(references, cursor, previous, to, marker) {
                return token;
            }
        }
    }
    let mut prev = previous;
    let mut next = previous + 1;
    loop {
        let view = cursor.extract_substring(prev, next);
        if view.is_empty() {
            return Token::Unknown(cursor.extract_substring(previous, prev));
        } else if view == " " {
            return Token::Unknown(cursor.extract_substring(previous, next));
        } else {
            prev += 1;
            next += 1;
        }
    }
}

enum Marker {
    Ruled(&'static [&'static str]),
    Ignore,
}

fn parse_with_strings<'a>(
    references: &[&'static str],
    cursor: &Cursor<'a>,
    previous: usize,
    to: &Transformator<'a>,
    marker: &Marker,
) -> Option<Token<'a>> {
    for &id in references {
        let len = id.len();
        if id == cursor.extract_substring(previous, previous + len) {
            match marker {
                Marker::Ruled(rules) => {
                    let next = cursor.extract_substring(previous + len, previous + len + 1);
                    if rules.contains(&next) {
                        return Some(to(previous, previous + len));
                    } else {
                        continue;
                    }
                }
                Marker::Ignore => return Some(to(previous, previous + len)),
            };
        };
    }
    None
}
#[test]
fn name() {
    let c = Cursor::new("let");
    let token1 = parse_token(&c);
    let c = Cursor::new("fna");
    let token2 = parse_token(&c);
    let c = Cursor::new("struct");
    let token3 = parse_token(&c);
    let c = Cursor::new("+");
    let token4 = parse_token(&c);
    dbg!(token1, token2, token3, token4);
}
