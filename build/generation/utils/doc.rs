use crate::generation::*;
use quote::{ToTokens, TokenStreamExt};

macro_rules! doc {
    ($($arg:tt)+) => { $crate::generation::utils::Doc::new(format!($($arg)+), false) };
}

macro_rules! idoc {
    ($($arg:tt)+) => { $crate::generation::utils::Doc::new(format!($($arg)+), true) };
}

pub struct Doc(Vec<String>, bool);

impl Doc {
    pub fn new(string: String, inner: bool) -> Self {
        Self(
            string.lines().map(str::to_string).collect::<Vec<_>>(),
            inner,
        )
    }

    pub fn replace(string: String) -> String {
        fn replace(mut s: String, old: &str, new: &str) -> String {
            let mut index = 0;

            while let Some(i) = &s[index..].find(old) {
                let i = index + i;
                let mut j = i + old.len();
                if &s[j..j + 1] == "r" {
                    j += 1;
                }
                let before = &s[..i];
                let after = &s[j + 1..].trim_start();

                let k = after
                    .find("\"]")
                    .expect("Cannot find \"] after #[doc = r?\"");
                let doc = after[..k].to_string();
                let after = after[k + 2..].to_string();

                s = before.to_string() + new + &doc + "\n";
                index = s.len();
                s = s + &after;
            }

            s
        }

        string
            .lines()
            .map(|line| {
                let line = line.to_string();
                let line = replace(line, "# [doc = ", "/// ");
                let line = replace(line, "# ! [doc = ", "//! ");

                line + "\n"
            })
            .collect::<String>()
    }
}

impl ToTokens for Doc {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(doc, inner) = self;

        if *inner {
            tokens.append_all(quote! { #(#![doc = #doc])* });
        } else {
            tokens.append_all(quote! { #(#[doc = #doc])* });
        }
    }
}
