use crate::*;

macro_rules! idoc {
    ($($arg:tt)+) => { $crate::utils::Doc::inner(format!($($arg)+)) };
}

macro_rules! doc {
    ($($arg:tt)+) => { $crate::utils::Doc::outer(format!($($arg)+)) };
}

macro_rules! comment {
    ($($arg:tt)+) => { $crate::utils::Doc::comment(format!($($arg)+)) };
}

macro_rules! centered_comment {
    ($col:literal, $($arg:tt)+) => {{
        let text = format!($($arg)+);
        let len = $col - 6 - text.len();
        let (left, right) = if len % 2 == 0 {
            (len / 2, len / 2)
        } else {
            (len / 2, len / 2 + 1)
        };

        comment!(
            "{sep} //
            {sep} //
            {left}{text}{right} //
            {sep} //
            {sep} //",
            sep = "=".repeat($col - 6),
            text = text,
            left = " ".repeat(left),
            right = " ".repeat(right),
        )
    }};
}

pub const COMMENT_START: &str = "__COMMENT_START__";
pub const COMMENT_END: &str = "__COMMENT_END__";

#[derive(Clone, PartialEq, Debug)]
pub enum Doc {
    Inner(Vec<String>),
    Outer(Vec<String>),
    Comment(Vec<String>),
}

impl Doc {
    pub fn inner(string: String) -> Self {
        Self::Inner(
            dedent(&string.trim())
                .map(|line| format!(" {}", line))
                .collect(),
        )
    }

    pub fn outer(string: String) -> Self {
        Self::Outer(
            dedent(&string.trim())
                .map(|line| format!(" {}", line))
                .collect(),
        )
    }

    pub fn comment(string: String) -> Self {
        Self::Comment(
            dedent(&string.trim())
                .map(|line| format!("{}{}{}", COMMENT_START, line, COMMENT_END))
                .collect(),
        )
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

        tokens.append(TokenTree::Group(Group::new(Delimiter::None, match self {
            Self::Inner(lines) => quote! { #(#![doc = #lines])* },
            Self::Outer(lines) => quote! { #(#[doc = #lines])* },
            Self::Comment(lines) => quote! { #(#lines)* },
        })));
    }
}

impl Default for Doc {
    fn default() -> Self {
        Self::Outer(vec![])
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
