//! This library behaves a lot like the `rustversion` library written by Dtolnay.
//!

#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::from_iter_instead_of_collect,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::needless_pass_by_value,
    clippy::redundant_else,
    clippy::toplevel_ref_arg,
    clippy::unreadable_literal
)]
mod error;
mod expr;
mod iter;
mod release;
mod token;
mod version;

use error::{Error, Result};
use iter::IterImpl;
use proc_macro::{Delimiter, Group, Punct, TokenStream, TokenTree};

use crate::{token::parse_paren, version::Version};

include!(concat!(env!("OUT_DIR"), "/libversions.rs"));

#[derive(Clone, Copy)]
enum Library {
    Format,
}

#[proc_macro]
pub fn ffversion(item: TokenStream) -> TokenStream {
    find_attributes(item)
}

fn find_attributes(item: TokenStream) -> TokenStream {
    let mut item = iter::new(item);
    let mut out = TokenStream::new();

    loop {
        // Check for '#' to start an attribute
        match item.next() {
            Some(TokenTree::Punct(ref punct)) if punct.as_char() == '#' => {
                // Check that the next two elements are Group([]) and Group({})

                let attr = match item.peek() {
                    Some(TokenTree::Group(ref group))
                        if group.delimiter() == Delimiter::Bracket =>
                    {
                        match item.next() {
                            Some(TokenTree::Group(group)) => group,
                            _ => unreachable!(),
                        }
                    }
                    // It was not an attribute type group so push the punct and continue
                    _ => {
                        out.extend([TokenTree::Punct(punct.clone())]);
                        continue;
                    }
                };

                let body = match item.peek() {
                    Some(TokenTree::Group(ref group)) if group.delimiter() == Delimiter::Brace => {
                        match item.next() {
                            Some(TokenTree::Group(group)) => group,
                            _ => unreachable!(),
                        }
                    }
                    // It was not an attribute type group so push the punct and continue
                    _ => {
                        out.extend([TokenTree::Punct(punct.clone()), TokenTree::Group(attr)]);
                        continue;
                    }
                };

                out.extend(
                    handle_attribute(punct.clone(), attr, body).map_err(Error::into_compile_error),
                );
            }
            Some(TokenTree::Group(group)) => out.extend([TokenTree::Group(Group::new(
                group.delimiter(),
                find_attributes(group.stream()),
            ))]),
            Some(x) => out.extend([x]),
            None => break,
        }
    }

    out
}

fn handle_attribute(_punct: Punct, attr: Group, body: Group) -> Result<TokenStream> {
    let ref mut attr_iter = iter::new(attr.stream());

    match attr_iter.next() {
        Some(TokenTree::Ident(ident)) if ident.to_string() == "libavformat" => {
            let args = parse_paren(&ident, attr_iter)?;
            Ok(libavformat(args.stream(), find_attributes(body.stream())))
        }
        _ => {
            panic!("Unimplemented");
        }
    }
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

    let satisfied = match library {
        Library::Format => check_satisfaction(args, LIBAVFORMAT)?,
        _ => unreachable!(),
    };

    if satisfied {
        Ok(input)
    } else {
        Ok(TokenStream::new())
    }
}

fn check_satisfaction(args: &mut IterImpl, version: Version) -> Result<bool> {
    let expr = expr::parse(args)?;
    token::parse_end(args)?;

    Ok(expr.eval(version))
}
