//! crates/quakc/src/lexer/token.rs
//! token.rs : Contains all the token definitions.
//! github.com/hrshtmk (Harshit Mukhedkar) - 2026-10-03

use logos::Logos;
use std::ops::Range;
 
use super::tokenizer::{
    lex_block_comment, lex_single_quoted, lex_string, parse_float, parse_int, LexErrorKind,
};
 
pub type Span = Range<usize>;
pub type Spanned = (Token, Span);
 
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexErrorKind)]
#[logos(skip r"([ \t\r\n\f]+|//[^\n]*)+")]
pub enum Token {
    // ---Keywords---
    #[token("Out")]
    Out,
 
    #[token("Inp")]
    Inp,
 
    #[token("int")]
    TypeInt,
 
    #[token("float")]
    TypeFlt,
 
    #[token("double")]
    TypeDbl,
 
    #[token("str")]
    TypeStr,
 
    #[token("bool")]
    TypeBool,
 
    #[token("char")]
    TypeChar,
 
    #[token("import")]
    Import,
 
    #[token("struct")]
    Struct,
 
    #[token("kind")]
    Kind,
 
    #[token("do")]
    Do,
 
    #[token("or")]
    Or,
 
    #[token("if")]
    If,
 
    #[token("for")]
    For,
 
    #[token("switch")]
    Switch,
 
    #[token("case")]
    Case,
 
    #[token("default")]
    Default,
 
    #[token("break")]
    Break,
 
    #[token("redo")]
    Redo,
 
    #[token("match")]
    Match,
 
    #[token("throw")]
    Throw,
 
    #[token("return")]
    Return,
 
    #[token("is")]
    Is,
 
    #[token("true")]
    True,
 
    #[token("false")]
    False,
 
    #[token("Box")]
    Box,
 
    // ---Literals & identifiers---
    // User types (`kind` names) and function names are plain identifiers.
    #[regex(r"[A-Za-z][A-Za-z0-9_]*|_[A-Za-z0-9_]+", |lex| lex.slice().to_owned())]
    Ident(String),
 
    #[regex(r"[0-9][0-9_]*", parse_int)]
    #[regex(r"0[xX][0-9a-fA-F_]+", parse_int)]
    Int(i64),
 
    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9]+)?", parse_float)]
    #[regex(r"[0-9][0-9_]*[eE][+-]?[0-9]+", parse_float)]
    Float(f64),
 
    /// Raw `"..."` text including both quotes; may contain `${...}`.
    #[token("\"", lex_string)]
    Str(String),
 
    /// `'...'` with escapes decoded. One char = `char` literal.
    #[regex(r"'([^'\\\n]|\\[^\n])*'", lex_single_quoted)]
    SingleQuoted(String),
 
    // ---Symbols---
    #[token("{")]
    LBrace,
 
    #[token("}")]
    RBrace,
 
    #[token("(")]
    LParen,
 
    #[token(")")]
    RParen,
 
    #[token("[")]
    LBracket,
 
    #[token("]")]
    RBracket,
 
    #[token(",")]
    Comma,
 
    #[token(";")]
    Semi,
 
    #[token(":")]
    Colon,
 
    #[token("::")]
    ColonColon,
 
    #[token(".")]
    Dot,
 
    #[token("...")]
    Ellipsis,
 
    #[token("@")]
    At,
 
    #[token("->")]
    RightArrow,
 
    #[token("<-")]
    LeftArrow,
 
    #[token("|")]
    Bar,
 
    #[token("_")]
    Underscore,
 
    // ---Operators---
    #[token("=")]
    Equals,
 
    #[token("==")]
    Equates,
 
    #[token("!=")]
    NotEquates,
 
    #[token("<")]
    Less,
 
    #[token("<=")]
    LessEq,
 
    #[token(">")]
    Greater,
 
    #[token(">=")]
    GreaterEq,
 
    #[token("+")]
    Plus,
 
    #[token("-")]
    Minus,
 
    /// Multiply, or the command prefix in `*name { ... }`.
    #[token("*")]
    Star,
 
    #[token("**")]
    StarStar,
 
    #[token("/")]
    Slash,
 
    #[token("%")]
    Percent,
 
    #[token("+=")]
    PlusEq,
 
    #[token("-=")]
    MinusEq,
 
    #[token("*=")]
    StarEq,
 
    #[token("/=")]
    SlashEq,
 
    #[token("&&")]
    AndAnd,
 
    #[token("||")]
    OrOr,
 
    #[token("!")]
    Bang,
 
    /// Dropped by lexer, never reaches the parser.
    #[token("/*", lex_block_comment)]
    BlockComment,
}