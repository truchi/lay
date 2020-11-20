#[macro_use]
mod utils;
mod lay;

mod examples;
mod style;

use lay::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Deref;
use utils::*;

impl StylerFn {
    const GET: &'static str = "get_";
    const MUT: &'static str = "_mut";
    const ON: &'static str = "on_";
    const SET: &'static str = "";
}

impl Styler {
    const STYLER: &'static [&'static str] = &["Styler"];
    const STYLER_INDEX: &'static [&'static str] = &["Styler", "Index"];
    const STYLER_INDEX_MUT: &'static [&'static str] = &["Styler", "Index", "Mut"];
    const STYLER_MUT: &'static [&'static str] = &["Styler", "Mut"];
}

impl Variant {
    const TYPE: &'static str = "u8";
}

impl Lay {
    const ATTRIBUTES: &'static [(&'static str, &'static [&'static str])] = &[
        ("Weight", &["Bold", "Light"]),
        ("Slant", &["Italic"]),
        ("Underline", &["Underlined"]),
        ("Strike", &["Striked"]),
        ("Overline", &["Overlined"]),
        ("Invert", &["Inverted"]),
        ("Blink", &["Slow", "Fast"]),
        ("Border", &["Circle", "Frame"]),
    ];
    const BACKGROUND: &'static str = "Background";
    const COLOR: &'static str = "Color";
    const COLORS: &'static [(&'static [&'static str], &'static [&'static str])] = &[
        (&["White"], &[]),
        (&["Black"], &[]),
        (&["Grey"], &[]),
        (&["Dark", "Grey"], &[]),
        (&["Red"], &[]),
        (&["Dark", "Red"], &[]),
        (&["Green"], &[]),
        (&["Dark", "Green"], &[]),
        (&["Yellow"], &[]),
        (&["Dark", "Yellow"], &[]),
        (&["Blue"], &[]),
        (&["Dark", "Blue"], &[]),
        (&["Magenta"], &[]),
        (&["Dark", "Magenta"], &[]),
        (&["Cyan"], &[]),
        (&["Dark", "Cyan"], &[]),
        (&["Rgb"], &["r", "g", "b"]),
        (&["Ansi"], &["ansi"]),
    ];
    const FOREGROUND: &'static str = "Foreground";
    const INDEX: &'static str = "I";
    const NONE: &'static str = "No";
    const RESET: &'static str = "Reset";
}

/// Generation
#[derive(Clone, Debug)]
struct Generation(Lay);

impl Deref for Generation {
    type Target = Lay;

    fn deref(&self) -> &Lay {
        &self.0
    }
}

fn main() {
    let cwd = std::env::current_dir().expect("Cannot get current directory");
    let dir = cwd.parent().expect("Cannot get cwd's parent");

    let gen = Generation(Lay::new());
    // panic!("{:#?}", gen);

    let mut style_dir = dir.to_path_buf();
    style_dir.push("src/style/gen/");
    gen.generate_style(style_dir.to_str().expect("Cannot convert dir to str"));

    let mut examples_dir = dir.to_path_buf();
    examples_dir.push("examples/");
    gen.generate_examples(examples_dir.to_str().expect("Cannot convert dir to str"));

    // let mut docs_dir = dir.to_path_buf();
    // docs_dir.push("src/");
    // gen.generate_docs(docs_dir.to_str().expect("Cannot convert dir to str"));
}

fn concat(streams: &[TokenStream]) -> TokenStream {
    streams.iter().fold(quote! {}, |tokens, stream| {
        if stream.is_empty() {
            tokens
        } else {
            quote! { #tokens #LINE_BREAK #stream }
        }
    })
}
