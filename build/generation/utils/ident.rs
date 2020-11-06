use crate::generation::*;
use quote::{ToTokens, TokenStreamExt};
use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Ident(Vec<String>);

impl Ident {
    pub fn new(parts: &[&str]) -> Self {
        Self(parts.into_iter().map(|str| str::to_string(str)).collect())
    }

    pub fn prefix(&self, prefix: &str) -> Self {
        let mut parts = self.0.clone();
        parts.insert(0, prefix.to_string());

        Self(parts)
    }

    pub fn suffix(&self, suffix: &str) -> Self {
        let mut parts = self.0.clone();
        parts.push(suffix.to_string());

        Self(parts)
    }

    pub fn no(&self) -> Self {
        self.prefix(Lay::NO)
    }

    pub fn reset(&self) -> Self {
        self.prefix(Lay::RESET)
    }

    pub fn get(&self) -> Self {
        self.prefix(Lay::GET)
    }

    pub fn get_mut(&self) -> Self {
        self.prefix(Lay::GET).suffix(Lay::MUT)
    }

    pub fn set(&self) -> Self {
        self.clone()
    }

    pub fn set_mut(&self) -> Self {
        self.suffix(Lay::MUT)
    }

    pub fn on_set(&self) -> Self {
        self.prefix(Lay::ON)
    }

    pub fn on_set_mut(&self) -> Self {
        self.prefix(Lay::ON).suffix(Lay::MUT)
    }

    pub fn on_reset(&self) -> Self {
        self.prefix(Lay::RESET).prefix(Lay::ON)
    }

    pub fn on_reset_mut(&self) -> Self {
        self.prefix(Lay::RESET).prefix(Lay::ON).suffix(Lay::MUT)
    }

    pub fn lower(&self) -> proc_macro2::Ident {
        let i = self.0.iter().map(low).collect::<Vec<_>>().join("");
        ident!(i)
    }

    pub fn pascal(&self) -> proc_macro2::Ident {
        let i = self.0.iter().map(cap).collect::<Vec<_>>().join("");
        ident!(i)
    }

    pub fn snake(&self) -> proc_macro2::Ident {
        let i = self.0.iter().map(low).collect::<Vec<_>>().join("_");
        ident!(i)
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.pascal());
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.pascal())
    }
}

fn low(s: &String) -> String {
    s.to_lowercase()
}

fn cap(s: &String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
