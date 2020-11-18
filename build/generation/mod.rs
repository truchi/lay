#[macro_use]
mod utils;
#[macro_use]
mod lay;

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
    const ATTRIBUTES: &'static [(&'static str, &'static str, &'static [&'static str])] = &[
        ("Wgt", "Weight", &["Bold", "Light"]),
        ("Slt", "Slant", &["Italic"]),
        ("Udl", "Underline", &["Underlined"]),
        ("Str", "Strike", &["Striked"]),
        ("Ovl", "Overline", &["Overlined"]),
        ("Inv", "Invert", &["Inverted"]),
        ("Blk", "Blink", &["Slow", "Fast"]),
        ("Brd", "Border", &["Circle", "Frame"]),
    ];
    const BACKGROUND: (&'static str, &'static str) = ("Bg", "Background");
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
    const FOREGROUND: (&'static str, &'static str) = ("Fg", "Foreground");
    const INDEX: &'static str = "I";
    const NONE: &'static str = "No";
    const RESET: &'static str = "Reset";
}

/// Generation
#[derive(Clone, Debug)]
pub struct Generation(Lay);

impl Deref for Generation {
    type Target = Lay;

    fn deref(&self) -> &Lay {
        &self.0
    }
}

pub fn generate() {
    if let Ok(profile) = std::env::var("PROFILE") {
        if profile == "debug" {
            let gen = Generation(Lay::new());
            // panic!("{:#?}", gen.styler);
            println!("cargo:rerun-if-changed=build/mod.rs");

            write_part("style/mod.rs", "import_markers", gen.import_markers());

            write(&format!("style/{}.rs", gen.color.snake), gen.color());
            write(&format!("style/{}.rs", gen.reset.snake), gen.reset());
            write(&format!("style/{}.rs", gen.index.snake), gen.i());
            write(&format!("style/{}.rs", gen.none.snake), gen.no());
            write("style/style.rs", gen.style());

            let (styler_index, styler_index_mut) = gen.styler_index();
            let (styler, styler_mut) = gen.styler();
            write("style/styler/mod.rs", gen.mod_style_styler());
            write("style/styler/styler_index.rs", styler_index);
            write("style/styler/styler_index_mut.rs", styler_index_mut);
            write("style/styler/styler.rs", styler);
            write("style/styler/styler_mut.rs", styler_mut);

            write(
                &format!("style/attributes/mod.rs"),
                gen.mod_style_attributes(),
            );
            for ground in &gen.grounds {
                write(
                    &format!("style/attributes/{}.rs", ground.snake),
                    gen.ground(ground),
                );
            }
            for attribute in &gen.attributes {
                write(
                    &format!("style/attributes/{}.rs", attribute.snake),
                    gen.attr(attribute),
                );
            }

            write("style/backends/mod.rs", gen.mod_style_backends());
            write("style/backends/crossterm.rs", gen.backend_crossterm());

            let styled_impls = (
                &Str::new("Styled<T>"),
                &[quote! { T: Display }][..],
                &Str::new("style"),
            );
            write(
                "style/styled_impls.rs",
                concat(&[
                    quote! {
                        use crate::*;
                        use std::fmt::Display;
                        use std::ops::{Add, Div, Mul};
                        use std::ops::{BitAnd, BitOr, BitXor};
                        use std::ops::{Rem, Not};
                        #LINE_BREAK
                    },
                    gen.impl_styler_index(styled_impls),
                    gen.impl_styler_index_mut(styled_impls),
                    gen.impl_styler(styled_impls),
                    gen.impl_styler_mut(styled_impls),
                    gen.impl_styler_ops(styled_impls),
                ]),
            );
        }
    }
}

fn concat(streams: &[TokenStream]) -> TokenStream {
    streams.iter().fold(quote! {}, |tokens, stream| {
        quote! { #tokens #LINE_BREAK #stream }
    })
}
