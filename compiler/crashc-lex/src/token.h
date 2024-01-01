#ifndef CRASHC_TOKEN_H
#define CRASHC_TOKEN_H

#include "iostream"
#include <string>

class Token {

public:
    enum class Kind {
        eof = -1,

        PUBLIC = -2, // public || pub
        PROTECTED = -3, // protected || prot

        IMPORT = -4, // import
        CLASS = -5, // class
        ENUM = -6, // enum
        INTERFACE = -7, // interface

        RETURN = -8, // return || ret
        THROW = -9, // throw
        IF = -10, // if
        SWITCH = -11, // switch
        MATCH = -12, // match
        LOOP = -13, // loop
        FOR = -14, // for
        MUTABLE = -15, // mut

        BOOLEAN = -16, // boolean || bool
        CHARACTER = -17, // char
        I8 = -18, // i8
        U8 = -19, // u8
        I16 = -20, // i16
        U16 = -21, // u16
        I32 = -22, // i32
        U32 = -23, // u32
        I64 = -24, // i64
        U64 = -25, // u64
        F32 = -26, // f32
        F64 = -27, // f64

        COPY = -28, // &
        ASSIGN = -29, // =

        AND = -30, // &&
        OR = -31, // ||

        EQUALS = -32, // ==
        NOT_EQUALS = -33, // !=
        GREATER_OR_EQUAL = -34, // >=
        LESS_OR_EQUAL = -35, // <=
        EXCLAMATION = -36, // !
        GREATER = -37, // >
        LESS = -38, // <

        ADD = -39, // +
        SUBTRACT = -40, // -
        MULTIPLY = -41, // *
        DIVIDE = -42, // /
        MODULUS = -43, // %

        OPEN_BRACE = -44, // (
        CLOSE_BRACE = -45, // )
        COMMA = -46, // ,
        EITHER = -47, // ?
        THEN = -48, // :
        SEMICOLON = -49, // ;
        OPEN_CURLY_BRACE = -50, // {
        CLOSE_CURLY_BRACE = -51, // }
        OPEN_SQUARE_BRACE = -52, // [
        CLOSE_SQUARE_BRACE = -53, // ]

        LITERAL_BOOLEAN = -54, // true || false
        LITERAL_STRING = -55, // "Some text here"
        LITERAL_CHAR = -56, // 'a'
        LITERAL_NUMBER = -57, // 345 || 34.4 || 5.6_564 || #345 || o#43 || b#000_0011
        LITERAL_IDENTIFIER = -58 // this || self || someThing
    };

    Token(Kind kind, const char* asString) {
        this->kind = kind;
        this->asString = asString;
    }

    Kind getKind() const {
        return kind;
    }

    const std::string &getAsString() const {
        return asString;
    }

private:
    Kind kind;
    std::string asString;
};

extern Token parse(const std::string& input);

#endif
