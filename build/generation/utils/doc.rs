#![allow(unused)]

use crate::generation::*;
use quote::{ToTokens, TokenStreamExt};

macro_rules! idoc {
    ($($arg:tt)+) => { $crate::generation::utils::Doc::inner(format!($($arg)+)) };
}

macro_rules! doc {
    ($($arg:tt)+) => { $crate::generation::utils::Doc::outer(format!($($arg)+)) };
}

macro_rules! comment {
    ($($arg:tt)+) => { $crate::generation::utils::Doc::comment(format!($($arg)+)) };
}

pub const COMMENT_START: &str = "__COMMENT_START__";
pub const COMMENT_END: &str = "__COMMENT_END__";

pub struct Doc(TokenStream);

impl Doc {
    pub fn inner(string: String) -> Self {
        let lines = dedent(&string.trim()).map(|line| format!(" {}", line));

        Self(quote! { #(#![doc = #lines])* })
    }

    pub fn outer(string: String) -> Self {
        let lines = dedent(&string.trim()).map(|line| format!(" {}", line));

        Self(quote! { #(#[doc = #lines])* })
    }

    pub fn comment(string: String) -> Self {
        let lines =
            dedent(&string.trim()).map(|line| format!("{}{}{}", COMMENT_START, line, COMMENT_END));

        Self(quote! { #(#lines)* })
    }

    pub fn replace(string: String) -> String {
        let rodoc = "# [doc = r\"";
        let ridoc = "# ! [doc = r\"";
        let odoc = "# [doc = \"";
        let idoc = "# ! [doc = \"";

        let string = replace(&string, rodoc, "\"]", "///", "\n");
        let string = replace(&string, ridoc, "\"]", "//!", "\n");
        let string = replace(&string, odoc, "\"]", "///", "\n");
        let string = replace(&string, idoc, "\"]", "//!", "\n");
        string
            .replace(&format!("\"{}", COMMENT_START), "// ")
            .replace(&format!("{}\"", COMMENT_END), "\n")
    }
}

impl ToTokens for Doc {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use proc_macro2::*;

        tokens.append(TokenTree::Group(Group::new(
            Delimiter::None,
            self.0.clone(),
        )));
    }
}

// ======= //
// Helpers //
// ======= //

/// Removes indentation
fn dedent(s: &str) -> impl Iterator<Item = &str> {
    let indent = get_indent(s);

    s.lines().enumerate().map(move |(n, line)| {
        // Skip first line and small lines
        if n == 0 || line.len() < indent {
            line
        } else {
            &line[indent..]
        }
    })
}

/// Finds the smallest amout of spaces at the start of the lines,
/// except for first line, and blank lines.
fn get_indent(s: &str) -> usize {
    s.lines()
        .enumerate()
        .fold(None, |indent: Option<usize>, (n, line)| {
            // Disregard first line and black lines
            if n == 0 || line.len() == 0 {
                return None;
            }

            // Count starting spaces
            let mut i = 0;
            for c in line.chars() {
                if c == ' ' {
                    i += 1;
                } else {
                    break;
                }
            }

            // Min that with previous iteration
            if let Some(indent) = indent {
                Some(indent.min(i))
            } else {
                Some(i)
            }
        })
        .unwrap_or(0)
}

// Does what you think it does, hopefully.
fn replace(s: &str, old_start: &str, old_end: &str, new_start: &str, new_end: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut end = s;

    while let Some(new_end) = replace_buf(end, &mut buf, old_start, old_end, new_start, new_end) {
        end = new_end;
    }

    buf.push_str(end);
    buf
}

// Replaces until `new_end` into `buf` and returns the rest, if some.
fn replace_buf<'a>(
    s: &'a str,
    buf: &mut String,
    old_start: &str,
    old_end: &str,
    new_start: &str,
    new_end: &str,
) -> Option<&'a str> {
    if let Some(i) = s.find(old_start) {
        let start = &s[..i];
        let end = &s[i + old_start.len()..];

        let j = end.find(old_end).expect("cannot find doc ending");
        let middle = &end[..j];
        let end = &end[j + old_end.len()..];

        buf.push_str(start);
        buf.push_str(new_start);
        buf.push_str(middle);
        buf.push_str(new_end);

        Some(end)
    } else {
        None
    }
}
