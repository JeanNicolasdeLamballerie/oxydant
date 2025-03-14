use cursor::Cursor;

mod cursor;

pub fn hello() {
    let mut cursor_reader = Cursor::new("hello");
    let tokens = cursor::Tokens::new(&mut cursor_reader);
    let scopes = tokens.scopes();
    dbg!(scopes);
}
