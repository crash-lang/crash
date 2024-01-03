use std::str::Chars;

pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
    prev: char
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {

    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            chars: input.chars(),
            prev : EOF_CHAR
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    pub(crate) fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn second(&self) -> char {
        let mut iterator = self.chars.clone();
        iterator.next();
        iterator.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    pub(crate) fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len()
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        {
            self.prev  = c;
        }

        Some(c);
    }

    pub(crate) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}