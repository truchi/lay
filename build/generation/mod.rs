#![allow(unused)]

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

impl Ident {
    const GET: &'static str = "get_";
    const MUT: &'static str = "_mut";
    const ON: &'static str = "on_";
    const SET: &'static str = "";
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
    const COLORS: &'static [(
        Option<&'static str>,
        &'static [(&'static str, &'static [&'static str])],
    )] = &[
        (None, &[("White", &[]), ("Black", &[])]),
        (Some("Dark"), &[
            ("Grey", &[]),
            ("Red", &[]),
            ("Green", &[]),
            ("Yellow", &[]),
            ("Blue", &[]),
            ("Magenta", &[]),
            ("Cyan", &[]),
        ]),
        (None, &[("Rgb", &["r", "g", "b"]), ("Ansi", &["ansi"])]),
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
            // panic!("{:#?}", gen);
            println!("cargo:rerun-if-changed=build/mod.rs");

            // write_part(
            // "style/mod.rs",
            // "import_markers",
            // Generation::import_markers(),
            // );

            write(&format!("style/{}.rs", gen.color.snake), gen.color());
            // write(
            // &format!("style/{}.rs", RESET.to_lowercase()),
            // Generation::reset(),
            // );
            // write(&format!("style/{}.rs", INDEX), Generation::i());
            // write(&format!("style/{}.rs", NONE), Generation::no());
            write(
                &format!("style/attributes/mod.rs"),
                gen.mod_style_attributes(),
            );
            // write("style/style.rs", Generation::style());
            // write("style/styler.rs", Generation::styler());

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
        }
    }
}
