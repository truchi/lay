use crate::*;

#[derive(Clone)]
pub struct Str {
    string: String,
    tree:   TokenTree,
}

derefs!(self Str {
    deref str { &self.string }
    tokens { tokens.append(self.tree.clone()) }
    f { <str as Display>::fmt(self, f) }
});

impl Str {
    pub fn new(string: &str) -> Self {
        let string = string.to_string();
        let tokens = TokenStream::from_str(&string).expect("Cannot convert to TokenStream");
        let tree = TokenTree::from(Group::new(Delimiter::None, tokens));
        Self { string, tree }
    }
}

impl PartialEq for Str {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Default for Str {
    fn default() -> Self {
        Self {
            string: Default::default(),
            tree:   TokenTree::from(Group::new(Delimiter::None, Default::default())),
        }
    }
}

impl Debug for Str {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.string)
    }
}
