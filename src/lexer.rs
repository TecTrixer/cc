// pub trait Lexer<T> {
// fn get_token(bytes: &mut Vec<u8>, idx: &mut usize, len: usize) -> T;
// }

const SEMICOLON: u8 = b';';
const EQUALS: u8 = b'=';
const SPACE: u8 = b' ';
const NEWLINE: u8 = b'\n';
const TAB: u8 = b'\t';
const MINUS: u8 = b'-';
const COMMA: u8 = b',';
const STRING_BRACKET: u8 = b'"';
const ESCAPE: u8 = b'\\';
const UNDERSCORE: u8 = b'_';
const OPENING_PARENTHESIS: u8 = b'(';
const CLOSING_PARENTHESIS: u8 = b')';
const OPENING_SCOPE: u8 = b'{';
const CLOSING_SCOPE: u8 = b'}';
const KEYWORD_INT: [u8; 3] = [b'i', b'n', b't'];
const KEYWORD_LONG: [u8; 4] = [b'l', b'o', b'n', b'g'];
const KEYWORD_VOID: [u8; 4] = [b'v', b'o', b'i', b'd'];

fn is_valid_identifier_character(c: u8) -> bool {
    match c {
        _ if c >= b'a' && c <= b'z' => true,
        _ if c >= b'A' && c <= b'Z' => true,
        UNDERSCORE => true,
        _ => false,
    }
}

#[derive(Clone, Debug, Hash)]
pub enum Token {
    StringLiteral(String),
    UIntLiteral(u64),
    Semicolon,
    Equals,
    Whitespace(Whitespace),
    Minus,
    Identifier(String),
    KeywordInt,
    KeywordLong,
    KeywordVoid,
    OpeningScope,
    ClosingScope,
    Comma,
    OpeningParenthesis,
    ClosingParenthesis,
}

#[derive(Clone, Debug, Hash)]
pub enum Whitespace {
    Space,
    Tab,
    Newline,
}

#[derive(Clone, Debug)]
pub struct Lexer {
    cursor: Cursor,
}

impl Lexer {
    pub fn new(bytes: Vec<u8>, idx: usize, len: usize) -> Lexer {
        Self {
            cursor: Cursor::new(bytes, idx, len),
        }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        if let Some(t) = self.semicolon() {
            Some(t)
        } else if let Some(t) = self.equals() {
            Some(t)
        } else if let Some(t) = self.minus() {
            Some(t)
        } else if let Some(t) = self.comma() {
            Some(t)
        } else if let Some(t) = self.opening_parenthesis() {
            Some(t)
        } else if let Some(t) = self.closing_parenthesis() {
            Some(t)
        } else if let Some(t) = self.opening_scope() {
            Some(t)
        } else if let Some(t) = self.closing_scope() {
            Some(t)
        } else if let Some(t) = self.whitespace() {
            Some(t)
        } else if let Some(t) = self.uint_literal() {
            Some(t)
        } else if let Some(t) = self.string_literal() {
            Some(t)
        } else if let Some(t) = self.keyword_int() {
            Some(t)
        } else if let Some(t) = self.keyword_long() {
            Some(t)
        } else if let Some(t) = self.keyword_void() {
            Some(t)
        } else if let Some(t) = self.identifier() {
            Some(t)
        } else {
            None
        }
    }

    fn semicolon(&mut self) -> Option<Token> {
        if self.cursor.peek()? == SEMICOLON {
            self.cursor.fwd();
            Some(Token::Semicolon)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn equals(&mut self) -> Option<Token> {
        if self.cursor.peek()? == EQUALS {
            self.cursor.fwd();
            Some(Token::Equals)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn minus(&mut self) -> Option<Token> {
        if self.cursor.peek()? == MINUS {
            self.cursor.fwd();
            Some(Token::Minus)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn comma(&mut self) -> Option<Token> {
        if self.cursor.peek()? == COMMA {
            self.cursor.fwd();
            Some(Token::Comma)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn opening_parenthesis(&mut self) -> Option<Token> {
        if self.cursor.peek()? == OPENING_PARENTHESIS {
            self.cursor.fwd();
            Some(Token::OpeningParenthesis)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn closing_parenthesis(&mut self) -> Option<Token> {
        if self.cursor.peek()? == CLOSING_PARENTHESIS {
            self.cursor.fwd();
            Some(Token::ClosingParenthesis)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn opening_scope(&mut self) -> Option<Token> {
        if self.cursor.peek()? == OPENING_SCOPE {
            self.cursor.fwd();
            Some(Token::OpeningScope)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn closing_scope(&mut self) -> Option<Token> {
        if self.cursor.peek()? == CLOSING_SCOPE {
            self.cursor.fwd();
            Some(Token::ClosingScope)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn whitespace(&mut self) -> Option<Token> {
        if let Some(token) = match self.cursor.peek()? {
            SPACE => Some(Token::Whitespace(Whitespace::Space)),
            TAB => Some(Token::Whitespace(Whitespace::Tab)),
            NEWLINE => Some(Token::Whitespace(Whitespace::Newline)),
            _ => None,
        } {
            self.cursor.fwd();
            Some(token)
        } else {
            self.cursor.reset();
            None
        }
    }
    fn uint_literal(&mut self) -> Option<Token> {
        let mut val = match self.cursor.peek()? {
            c if c >= b'0' && c <= b'9' => {
                self.cursor.fwd();
                (c - b'0') as u64
            }
            _ => {
                self.cursor.reset();
                return None;
            }
        };
        while let Some(c) = self.cursor.peek() {
            match c {
                UNDERSCORE => self.cursor.fwd(),
                c if c >= b'0' && c <= b'9' => {
                    val *= 10;
                    val += (c - b'0') as u64;
                    self.cursor.fwd();
                }
                _ => break,
            }
        }
        self.cursor.reset();
        Some(Token::UIntLiteral(val))
    }
    fn string_literal(&mut self) -> Option<Token> {
        if let Some(c) = self.cursor.peek() {
            if c != STRING_BRACKET {
                self.cursor.reset();
                return None;
            }
        }
        let mut res = String::new();
        let mut last_was_escape = false;
        while let Some(c) = self.cursor.peek() {
            match c {
                ESCAPE => {
                    if last_was_escape {
                        last_was_escape = false;
                        res.push(ESCAPE as char);
                    } else {
                        last_was_escape = true;
                    }
                }
                STRING_BRACKET => {
                    if last_was_escape {
                        last_was_escape = false;
                        res.push(STRING_BRACKET as char);
                    } else {
                        self.cursor.fwd();
                        return Some(Token::StringLiteral(res));
                    }
                }
                b'n' if last_was_escape => {
                    res.push(NEWLINE as char);
                    last_was_escape = false;
                }
                b't' if last_was_escape => {
                    res.push(TAB as char);
                    last_was_escape = false;
                }
                c => {
                    last_was_escape = false;
                    res.push(c as char);
                }
            }
        }
        self.cursor.reset();
        None
    }
    fn keyword_int(&mut self) -> Option<Token> {
        for d in KEYWORD_INT {
            let c = self.cursor.peek()?;
            if c != d {
                self.cursor.reset();
                return None;
            }
        }
        self.cursor.fwd();
        Some(Token::KeywordInt)
    }
    fn keyword_long(&mut self) -> Option<Token> {
        for d in KEYWORD_LONG {
            if let Some(c) = self.cursor.peek() {
                if c != d {
                    self.cursor.reset();
                    return None;
                }
            } else {
                self.cursor.reset();
                return None;
            }
        }
        self.cursor.fwd();
        Some(Token::KeywordLong)
    }
    fn keyword_void(&mut self) -> Option<Token> {
        for d in KEYWORD_VOID {
            if let Some(c) = self.cursor.peek() {
                if c != d {
                    self.cursor.reset();
                    return None;
                }
            } else {
                self.cursor.reset();
                return None;
            }
        }
        self.cursor.fwd();
        Some(Token::KeywordVoid)
    }
    fn identifier(&mut self) -> Option<Token> {
        let mut res = match self.cursor.peek()? {
            c if is_valid_identifier_character(c) => {
                self.cursor.fwd();
                String::from(c as char)
            }
            _ => {
                self.cursor.reset();
                return None;
            }
        };
        while let Some(c) = self.cursor.peek() {
            match c {
                c if is_valid_identifier_character(c) => {
                    res.push(c as char);
                    self.cursor.fwd();
                }
                _ => break,
            }
        }
        self.cursor.reset();
        Some(Token::Identifier(res))
    }
}

#[derive(Clone, Debug)]
struct Cursor {
    bytes: Vec<u8>,
    idx: usize,
    peek_idx: usize,
    len: usize,
}

impl Cursor {
    fn new(bytes: Vec<u8>, idx: usize, len: usize) -> Self {
        Self {
            bytes,
            idx,
            peek_idx: idx,
            len,
        }
    }
    // Reset peek cursor back to normal cursor
    fn reset(&mut self) {
        self.peek_idx = self.idx;
    }
    // Forward normal cursor to peek cursor
    fn fwd(&mut self) {
        self.idx = self.peek_idx;
    }
    // Reset the peek cursor, read the next byte and consume it
    fn _next(&mut self) -> Option<u8> {
        self.reset();
        if self.idx < self.len {
            let val = Some(self.bytes[self.idx]);
            self.idx += 1;
            val
        } else {
            None
        }
    }
    // Read the next byte, increase the peek cursor but do not consume the byte
    fn peek(&mut self) -> Option<u8> {
        if self.peek_idx < self.len {
            let val = Some(self.bytes[self.peek_idx]);
            self.peek_idx += 1;
            val
        } else {
            None
        }
    }
    // Consume n bytes
    fn _consume(&mut self, n: usize) {
        self.idx += n;
    }
}
