//! Quak lexer
//!
//! token.rs      Token enum (definitions only)
//! tokenizer.rs  lex(), callbacks, string scanning, lexer errors, tests
 
mod token;
mod tokenizer;
pub use token::{Span, Spanned, Token};
pub use tokenizer::{lex, split_string, LexError, LexErrorKind, StrPart};