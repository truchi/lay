#[macro_use]
mod utils;
mod lay;

mod doc;
mod geometry;
mod layer;
mod style;

use lay::*;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use std::{
    fmt::{self, Debug, Display, Formatter},
    fs::{create_dir_all, File},
    io::Write,
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};
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
struct Generation {
    lay:      Lay,
    root:     PathBuf,
    examples: PathBuf,
    src:      PathBuf,
    style:    PathBuf,
    layer:    PathBuf,
    geometry: PathBuf,
}

impl Deref for Generation {
    type Target = Lay;

    fn deref(&self) -> &Lay {
        &self.lay
    }
}

fn main() {
    let cwd = std::env::current_dir().expect("Cannot get current directory");
    let root = cwd.parent().expect("Cannot get cwd's parent").to_path_buf();
    let mut src = root.to_path_buf();
    let mut examples = root.to_path_buf();
    let mut style = root.to_path_buf();
    let mut layer = root.to_path_buf();
    let mut geometry = root.to_path_buf();
    src.push("src/");
    examples.push("examples/");
    style.push("src/style/gen/");
    layer.push("src/layer/gen/");
    geometry.push("src/geometry/gen/");

    let gen = Generation {
        lay: Lay::new(),
        root,
        examples,
        src,
        style,
        layer,
        geometry,
    };

    gen.generate_style();
    gen.generate_layer();
    gen.generate_geometry();
    gen.generate_docs();
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
