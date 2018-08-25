use std::cell::RefCell;

use proc_macro2::{Delimiter, Span};

use buffer::Cursor;
use error::{self, Error};
use span::IntoSpans;
use token::Token;

/// Support for checking the next token in a stream to decide how to parse.
///
/// Use [`ParseStream::lookahead1`] to construct this object.
///
/// [`ParseStream::lookahead1`]: struct.ParseBuffer.html#method.lookahead1
pub struct Lookahead1<'a> {
    scope: Span,
    cursor: Cursor<'a>,
    comparisons: RefCell<Vec<String>>,
}

impl<'a> Lookahead1<'a> {
    // Not public API.
    #[doc(hidden)]
    pub fn new(scope: Span, cursor: Cursor<'a>) -> Self {
        Lookahead1 {
            scope: scope,
            cursor: cursor,
            comparisons: RefCell::new(Vec::new()),
        }
    }

    pub fn peek<T: Peek>(&self, token: T) -> bool {
        let _ = token;
        if T::Token::peek(self) {
            return true;
        }
        self.comparisons.borrow_mut().push(T::Token::display());
        false
    }

    pub fn error(self) -> Error {
        let comparisons = self.comparisons.borrow();
        match comparisons.len() {
            0 => if self.cursor.eof() {
                Error::new(self.scope, "unexpected end of input")
            } else {
                Error::new(self.cursor.span(), "unexpected token")
            },
            1 => {
                let message = format!("expected {}", comparisons[0]);
                error::new_at(self.scope, self.cursor, message)
            }
            _ => {
                let join = comparisons.join(", ");
                let message = format!("expected one of: {}", join);
                error::new_at(self.scope, self.cursor, message)
            }
        }
    }

    // Not public API.
    #[doc(hidden)]
    pub fn cursor(&self) -> Cursor<'a> {
        self.cursor
    }
}

/// Types that can be parsed by looking at just one token.
///
/// This trait is sealed and cannot be implemented for types outside of Syn.
pub trait Peek: private::Sealed {
    // Not public API.
    #[doc(hidden)]
    type Token: Token;
}

impl<F: FnOnce(TokenMarker) -> T, T: Token> Peek for F {
    type Token = T;
}

pub enum TokenMarker {}

impl<S> IntoSpans<S> for TokenMarker {
    fn into_spans(self) -> S {
        match self {}
    }
}

// Not public API.
#[doc(hidden)]
pub fn is_delimiter(lookahead: &Lookahead1, delimiter: Delimiter) -> bool {
    lookahead.cursor.group(delimiter).is_some()
}

mod private {
    use super::{Token, TokenMarker};
    pub trait Sealed {}
    impl<F: FnOnce(TokenMarker) -> T, T: Token> Sealed for F {}
}
