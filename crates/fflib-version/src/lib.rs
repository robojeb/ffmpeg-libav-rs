//! This library behaves a lot like the `rustversion` library written by Dtolnay.
//!

mod error;
mod expr;
mod iter;
mod release;
mod token;
mod version;

use error::{Error, Result};
use proc_macro::TokenStream;

use crate::version::Version;

include!(concat!(env!("OUT_DIR"), "/libversions.rs"));

#[derive(Clone, Copy)]
enum Library {
    Format,
}

#[proc_macro]
pub fn ffversion(_item: TokenStream) -> TokenStream {
    _item
}

#[proc_macro_attribute]
pub fn libavformat(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg(Library::Format, args, input)
}

fn cfg(library: Library, args: TokenStream, input: TokenStream) -> TokenStream {
    try_cfg(library, args, input).unwrap_or_else(Error::into_compile_error)
}

fn try_cfg(library: Library, args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let ref mut args = iter::new(args);
    let expr = expr::parse(args)?;
    token::parse_end(args)?;

    let satisfied = match library {
        Library::Format => expr.eval(LIBAVFORMAT),
        _ => unreachable!(),
    };

    if satisfied {
        Ok(input)
    } else {
        Ok(TokenStream::new())
    }
}
