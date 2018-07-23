#[derive(Debug, PartialEq)]
pub enum TkType {
    Ident,
    Num,
}

#[derive(Debug, PartialEq)]
pub struct Token((u32, u32), TkType, String);

enum State {
    Fn(fn(&mut Lexer) -> State),
    EOF,
}

struct Lexer {
    code: String,
    tokens: Vec<Token>,
    state_fn: State,
    start: usize,
    offset: usize,
    // (line, pos) represent the position for user
    pos: u32,
    line: u32,
}

impl Lexer {
    fn new(code: String) -> Lexer {
        Lexer {
            code: code,
            tokens: vec![],
            state_fn: State::Fn(whitespace),
            start: 0,
            offset: 0,
            pos: 0,
            line: 1,
        }
    }

    fn ignore(&mut self) {
        self.pos += (self.offset - self.start) as u32;
        self.start = self.offset;
    }
    fn peek(&self) -> Option<char> {
        self.code.chars().nth(self.offset)
    }
    fn next(&mut self) -> Option<char> {
        self.offset += 1;
        let c = self.code.chars().nth(self.offset);
        match c {
            Some('\n') => {
                self.pos = 0;
                self.line += 1;
                c
            }
            _ => c,
        }
    }
    fn emit(&mut self, token_type: TkType) {
        unsafe {
            let s = self.code.get_unchecked(self.start..self.offset);
            self.tokens
                .push(Token((self.line, self.pos), token_type, s.to_string()));
        }
        self.ignore();
    }
}

fn whitespace(lexer: &mut Lexer) -> State {
    while let Some(c) = lexer.peek() {
        if c == ' ' || c == '\n' {
            lexer.next();
        } else {
            break;
        }
    }
    lexer.ignore();

    match lexer.peek() {
        Some(_c @ '0'...'9') => State::Fn(number),
        Some(_c @ 'a'...'z') => State::Fn(ident),
        Some(_c @ 'A'...'Z') => State::Fn(ident),
        None => State::EOF,
        _ => State::Fn(whitespace),
    }
}

fn ident(lexer: &mut Lexer) -> State {
    while let Some(c) = lexer.next() {
        if !c.is_alphanumeric() {
            break;
        }
    }
    lexer.emit(TkType::Ident);
    State::Fn(whitespace)
}

fn number(lexer: &mut Lexer) -> State {
    while let Some(c) = lexer.next() {
        if !c.is_digit(10) {
            break;
        }
    }
    lexer.emit(TkType::Num);
    State::Fn(whitespace)
}

fn lex<'a>(source: &'a str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.to_string());
    while let State::Fn(f) = lexer.state_fn {
        lexer.state_fn = f(&mut lexer);
    }
    lexer.tokens
}

#[cfg(test)]
mod tests {
    use self::TkType::*;
    use super::*;

    #[test]
    fn get_number_tokens() {
        let ts = lex("10 30");
        assert_eq!(
            ts,
            vec![
                Token((1, 0), Num, "10".to_string()),
                Token((1, 3), Num, "30".to_string()),
            ]
        );
    }

    #[test]
    fn get_ident_tokens() {
        let ts = lex(" abc6");
        assert_eq!(ts, vec![Token((1, 1), Ident, "abc6".to_string())])
    }
}
