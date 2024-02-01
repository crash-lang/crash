/*
 * Copyright 2024 Julian Siebert
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use crate::lexer::Lexer;
use crate::rule::LexingRule;
use crate::token::Token;
use crate::token::TokenType::*;

pub mod token;
pub mod position;

mod rule;
mod lexer;
mod macros;

pub fn tokenize(content: &str) -> Vec<Token> {
    Lexer::new(build_rules()).tokenize(content, true)
}

fn build_rules() -> Vec<LexingRule> {
    let mut rules: Vec<LexingRule> = Vec::new();

    // General whitespace
    add_regex_rule!(rules, Whitespace, "[ \t\r\n]+");

    // Comments
    add_regex_rule!(rules, Whitespace, "//[^\\r\\n]*");
    add_multi_line_rule!(rules, Whitespace, "/*", "*/");

    add_rule!(rules, Semicolon, ";");
    add_rule!(rules, Comma, ",");
    add_rule!(rules, OpenBrace, "(");
    add_rule!(rules, CloseBrace, ")");
    add_rule!(rules, OpenCurlyBrace, "{");
    add_rule!(rules, CloseCurlyBrace, "}");
    add_rule!(rules, OpenSquareBrace, "[");
    add_rule!(rules, CloseSquareBrace, "]");
    add_rule!(rules, Question, "?");
    add_rule!(rules, Colon, ":");
    add_rule!(rules, At, "@");
    add_rule!(rules, Dots, "...");


    add_rule!(rules, Add, "+");
    add_rule!(rules, Subtract, "-");
    add_rule!(rules, Multiply, "*");
    add_rule!(rules, Divide, "/");
    add_rule!(rules, Modulus, "mod");
    add_rule!(rules, Modulus, "%");


    add_rule!(rules, Equals, "==");
    add_rule!(rules, NotEquals, "!=");
    add_rule!(rules, Greater, ">");
    add_rule!(rules, GreaterOrEqual, ">=");
    add_rule!(rules, Less, "<");
    add_rule!(rules, LessOrEqual, "<=");
    add_rule!(rules, Exclamation, "!");
    add_rule!(rules, And, "&&");
    add_rule!(rules, Or, "||");


    add_rule!(rules, BitwiseOr, "|");
    add_rule!(rules, BitwiseAnd, "&");
    add_rule!(rules, BitwiseXor, "^");
    add_rule!(rules, BitwiseComplement, "~");
    add_rule!(rules, LeftShift, "<<");
    add_rule!(rules, RightShift, ">>");
    add_rule!(rules, UnsignedRightShift, ">>>");


    add_rule!(rules, Assign, "=");
    add_rule!(rules, InstanceOf, "instanceof");


    add_rule!(rules, I8, "i8");
    add_rule!(rules, I8, "byte");

    add_rule!(rules, U8, "u8");

    add_rule!(rules, I16, "i16");
    add_rule!(rules, I16, "short");

    add_rule!(rules, U16, "u16");

    add_rule!(rules, I32, "i32");
    add_rule!(rules, I32, "int");

    add_rule!(rules, U32, "u32");

    add_rule!(rules, I64, "i64");
    add_rule!(rules, I64, "long");

    add_rule!(rules, U64, "u64");

    add_rule!(rules, I128, "i128");

    add_rule!(rules, U128, "u128");

    add_rule!(rules, F32, "f32");
    add_rule!(rules, F32, "float");

    add_rule!(rules, F64, "f64");
    add_rule!(rules, F64, "double");

    add_rule!(rules, Boolean, "boolean");
    add_rule!(rules, Boolean, "bool");

    add_rule!(rules, Char, "char");


    add_rule!(rules, Function, "function");


    add_rule!(rules, If, "if");
    add_rule!(rules, Switch, "switch");
    add_rule!(rules, Match, "match");
    add_rule!(rules, Loop, "loop");
    add_rule!(rules, For, "for");
    add_rule!(rules, Return, "return");
    add_rule!(rules, Break, "break");
    add_rule!(rules, Continue, "continue");


    add_rule!(rules, Class, "class");
    add_rule!(rules, Interface, "interface");
    add_rule!(rules, Enum, "enum");
    add_rule!(rules, Annotation, "annotation");
    add_rule!(rules, Import, "import");
    add_rule!(rules, Implements, "implements");
    add_rule!(rules, Extends, "extends");

    add_rule!(rules, Public, "public");
    add_rule!(rules, Public, "pub");
    add_rule!(rules, Protected, "protected");
    add_rule!(rules, Protected, "prot");
    add_rule!(rules, Override, "override");
    add_rule!(rules, Mutable, "mut");

    /*
        Literals must start here.
        We don't want that any other keyword gets tokenized as an identifier literal,
        so we check for identifier literals at last
     */

    add_regex_rule!(rules, FloatLiteral, "[0-9_]*\\.[0-9_]*");
    add_regex_rule!(rules, BinaryLiteral, "b[01_]*");
    add_regex_rule!(rules, OctalLiteral, "o[0-7_]*");
    add_regex_rule!(rules, DecimalLiteral, "[0-9_]*");
    add_regex_rule!(rules, HexadecimalLiteral, "#[0-9a-fA-F_]*");
    add_regex_rule!(rules, IdentifierLiteral, "[a-zA-Z.:][a-zA-Z.:0-9]*");

    add_rule!(rules, BooleanLiteral, "true");
    add_rule!(rules, BooleanLiteral, "false");

    add_multi_line_rule!(rules, StringLiteral, "\"", "\"");
    add_multi_line_rule!(rules, CharLiteral, "'", "'");

    rules
}
