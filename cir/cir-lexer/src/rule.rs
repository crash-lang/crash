use regex::{Regex, RegexBuilder};
use crate::token::TokenType;

#[derive(Clone)]
pub struct LexingRule {
    patterns: Vec<Regex>,
    typ: TokenType
}

fn regex_escape(input: String) -> String {
    let mut builder = String::new();
    for char in input.chars() {
        match char {
            '\0' | '\n' | '\r' | '\t' | '\\' | '^' | '$' | '?' | '|' | '*' | '/' | '+' | '.' | '(' | ')' | '[' | ']' | '{' | '}' => {
                builder.push('\\');
                builder.push(char);
                continue;
            }
            _ => {}
        }

        let digit = char as u32;

        if digit > 0xff {
            builder.push_str("\\u");
            builder.push_str(format!("{}04x", char).as_str());
        } else if is_iso_control(digit) {
            builder.push_str("\\x");
            builder.push_str(format!("{}2x", char).as_str());
        } else {
            builder.push(char);
        }
    }

    builder
}

fn is_iso_control(digit: u32) -> bool {
    return digit <= 0x9F && (digit >= 0x7F || (0 == (digit >> 5)));
}

impl LexingRule {
    pub fn new(patterns: Vec<Regex>, typ: TokenType) -> Self {
        Self { patterns, typ }
    }

    pub fn add(&mut self, value: String) {
        self.patterns.push(Regex::new(regex_escape(value).as_str()).unwrap());
    }

    pub fn add_regex(&mut self, regex: String) {
        self.patterns.push(Regex::new(regex.as_str()).unwrap());
    }

    pub fn add_multi_line(&mut self, open: String, close: String) {
        self.add_delimiter(open, String::new(), close);
    }

    pub fn add_delimiter(&mut self, open: String, escape: String, closing: String) {
        let starting_symbol = regex_escape(open.clone());
        let escape_symbol = regex_escape(escape.clone());
        let closing_symbol = regex_escape(closing.clone());

        let regex_string;

        if escape.is_empty() {
            regex_string = format!("{}.*?{}", starting_symbol, closing_symbol);
        } else {
            regex_string = format!("{}(?:{}(?:{}|{}|(?!{}).)|(?!{}|{}).)*{}",
                                   starting_symbol, escape_symbol, escape_symbol, closing_symbol,
                                   closing_symbol, escape_symbol, closing_symbol, closing_symbol
            );
        }

        let regex = RegexBuilder::new(regex_string.as_str())
            .dot_matches_new_line(true)
            .build()
            .unwrap();

        self.patterns.push(regex);
    }

    pub fn patterns(&self) -> &Vec<Regex> {
        &self.patterns
    }

    pub fn typ(&self) -> TokenType {
        self.typ.clone()
    }
}