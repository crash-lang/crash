
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub len: u32
}

impl Token {
    pub(crate) fn new (kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {

    Invalid,

    /// "// comment"
    LineComment,

    /// "/* block comment */"
    BlockComment,

    Whitespace,

    Identifier,

    Literal { kind: LiteralKind },


    // General tokens

    /// ";"
    Semicolon,

    /// ","
    Comma,

    /// "("
    OpenBrace,

    /// ")"
    CloseBrace,

    /// "{"
    OpenCurlyBrace,

    /// "}"
    CloseCurlyBrace,

    /// "["
    OpenSquaredBrace,

    /// "]"
    CloseSquaredBrace,

    /// "?"
    Question,

    /// ":"
    Colon,


    // Math tokens

    /// "+"
    Add,

    /// "-"
    Subtract,

    /// "*"
    Multiply,

    /// "/"
    Divide,

    /// "%"
    Modulus,


    // Logic

    /// "=="
    Equals,

    /// "!="
    NotEquals,

    /// ">="
    GreaterOrEqual,

    /// "<="
    LessOrEqual,

    /// "!"
    Exclamation,

    /// ">"
    Greater,

    /// "<"
    Less,

    /// "&&"
    And,

    /// "||"
    Or,


    // Variable

    /// "&"
    Copy,

    /// "="
    Assign,

    /// "mut"
    Mutable,

    Primitive { kind: PrimitiveKind },


    // Statements

    /// "if"
    If,

    /// "switch"
    Switch,

    /// "match"
    Match,

    /// "loop"
    Loop,

    /// "for"
    For,

    /// "return" or "ret"
    Return,


    // File

    /// "class"
    Class,

    /// "interface"
    Interface,

    /// "enum"
    Enum,

    /// "import"
    Import,

    AccessModifier { kind: ModifierKind },


    /// End of input
    Eof
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModifierKind {
    /// "public" or "pub"
    Public,

    /// "protected" or "prot"
    Protected,

    /// "intern"
    Internal
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrimitiveKind {
    /// "i8"
    I8,

    /// "u8"
    U8,

    /// "i16"
    I16,

    /// "u16"
    U16,

    /// "i32"
    I32,

    /// "u32
    U32,

    /// "i64"
    I64,

    /// "u64"
    U64,

    /// "i128"
    I128,

    /// "u128"
    U128,

    /// "f32"
    F32,

    /// "f64"
    F64,

    /// "boolean" or "bool"
    Bool,

    /// "char"
    Char
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    /**
    Any _ gets ignored and is just a visual splitter

    Binary: b#0000_1011
    Octal: o#124_5
    Decimal: 345_7324
    Hexadecimal: #fff345
     */
    Integer {
        base: Base,
        empty: bool
    },

    /**
    Any _ gets ignored and is just a visual splitter

    Float literals shouldn't have a base,
    because it would be to complicated anyway.
     */
    Float {
        empty_exponent: bool
    },

    /// "'a'" | "'b'"
    Character { terminated: bool },

    /// ""Hello world""
    String { terminated: bool },

    /// "true" or "false"
    Boolean {
        val: bool
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Suffix: b#
    Binary = 2,

    /// Suffix: o#
    Octal = 8,

    Decimal = 10,

    /// Suffix: #
    Hexadecimal = 16
}