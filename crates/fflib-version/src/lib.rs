//! This library allows conditional compilation of portions of code based on the
//! version of varioud ffmpeg libav* libraries.
//!
//! # Supported ffmpeg libraries
//! The following libraries can be queried for their linked version
//!    * `libavcodec`
//!    * `libavformat`
//!    * `libavfilter`
//!    * `libavutil`
//!    * `libswresample`
//!
//! # Available macros
//!
//! This library provides two different methods for applying attributes through
//! a procedural macro.
//! The first is a procedural attribute macro, one per library, in the form
//!
//! ```no_run
//! #[fflib_version::libavformat(...)]
//! ```
//! Where "..." is a valid selector
//!
//! For cases where a procedureal attribute macro is not allowed (eg enum variants)
//! an alternative macro `fflib_version::ffversion!{}` can be wrapped around the
//! context you wish to configure.
//! Within this context psuedo-attributes can be added in the form:
//!
//! ```no_run
//! #[libavformat(...)] { }
//! ```
//! Where "..." is a selector, and the `{}` encompas the items which you wish to
//! be conditionally pasted into the output.
//!
//! # Selectors
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>since(58.29.100)</code></b>
//!   —<br>
//!   True when the specified library is a version greater than or equal to
//!   the value specified. Minor and Patch numbers can be omitted.
//!   </p>
//!
//!
//! - <p style="margin-left:50px;text-indent:-50px">
//!   <b><code>before(58.2)</code></b>
//!   —<br>
//!   True when the specified library is a version strictly less than
//!   the value specified. Minor and Patch numbers can be omitted.
//!   </p>
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
    Codec,
    Filter,
    Util,
    Resample,
}

#[proc_macro]
pub fn ffcfg(item: TokenStream) -> TokenStream {
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
        Some(TokenTree::Ident(ident)) if ident.to_string() == "libavcodec" => {
            let args = parse_paren(&ident, attr_iter)?;
            Ok(libavcodec(args.stream(), find_attributes(body.stream())))
        }
        Some(TokenTree::Ident(ident)) if ident.to_string() == "libavutil" => {
            let args = parse_paren(&ident, attr_iter)?;
            Ok(libavutil(args.stream(), find_attributes(body.stream())))
        }
        Some(TokenTree::Ident(ident)) if ident.to_string() == "libavfilter" => {
            let args = parse_paren(&ident, attr_iter)?;
            Ok(libavfilter(args.stream(), find_attributes(body.stream())))
        }
        Some(TokenTree::Ident(ident)) if ident.to_string() == "libswresample" => {
            let args = parse_paren(&ident, attr_iter)?;
            Ok(libswresample(args.stream(), find_attributes(body.stream())))
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

#[proc_macro_attribute]
pub fn libavcodec(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg(Library::Codec, args, input)
}

#[proc_macro_attribute]
pub fn libavutil(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg(Library::Util, args, input)
}

#[proc_macro_attribute]
pub fn libavfilter(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg(Library::Filter, args, input)
}

#[proc_macro_attribute]
pub fn libswresample(args: TokenStream, input: TokenStream) -> TokenStream {
    cfg(Library::Resample, args, input)
}

fn cfg(library: Library, args: TokenStream, input: TokenStream) -> TokenStream {
    try_cfg(library, args, input).unwrap_or_else(Error::into_compile_error)
}

fn try_cfg(library: Library, args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let ref mut args = iter::new(args);

    let satisfied = match library {
        Library::Format => check_satisfaction(args, LIBAVFORMAT)?,
        Library::Util => check_satisfaction(args, LIBAVUTIL)?,
        Library::Filter => check_satisfaction(args, LIBAVFILTER)?,
        Library::Codec => check_satisfaction(args, LIBAVCODEC)?,
        Library::Resample => check_satisfaction(args, LIBSWRESAMPLE)?,
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
