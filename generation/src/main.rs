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
struct Generation(Lay);

impl Deref for Generation {
    type Target = Lay;

    fn deref(&self) -> &Lay {
        &self.0
    }
}

fn main() {
    let cwd = std::env::current_dir().expect("Cannot get current directory");
    let mut dir = cwd.parent().expect("Cannot get cwd's parent").to_path_buf();
    dir.push("src/");
    let dir = dir.to_str().expect("Cannot convert dir to str");
    let gen = Generation(Lay::new());
    // panic!("{:#?}", gen);

    // style/gen/attributes/
    for ground in &gen.grounds {
        write(
            dir,
            &format!("style/gen/attributes/{}.rs", ground.snake),
            gen.ground(ground),
        );
    }
    for attribute in &gen.attributes {
        write(
            dir,
            &format!("style/gen/attributes/{}.rs", attribute.snake),
            gen.attr(attribute),
        );
    }
    write(
        dir,
        &format!("style/gen/attributes/mod.rs"),
        gen.mod_style_attributes(),
    );

    // style/gen/backends/
    write(
        dir,
        "style/gen/backends/crossterm.rs",
        gen.backend_crossterm(),
    );
    write(dir, "style/gen/backends/mod.rs", quote! {
        #[cfg(feature = "backend-crossterm")]
        mod crossterm;
    });

    // style/gen/styler/
    let (styler_index, styler_index_mut) = gen.styler_index();
    let (styler, styler_mut) = gen.styler();
    write(dir, "style/gen/styler/styler_index.rs", styler_index);
    write(
        dir,
        "style/gen/styler/styler_index_mut.rs",
        styler_index_mut,
    );
    write(dir, "style/gen/styler/styler.rs", styler);
    write(dir, "style/gen/styler/styler_mut.rs", styler_mut);
    write(dir, "style/gen/styler/mod.rs", gen.mod_style_styler());

    // style/gen/
    write(
        dir,
        &format!("style/gen/{}.rs", gen.color.snake),
        gen.color(),
    );
    write(
        dir,
        &format!("style/gen/{}.rs", gen.reset.snake),
        gen.reset(),
    );
    write(dir, &format!("style/gen/{}.rs", gen.index.snake), gen.i());
    write(dir, &format!("style/gen/{}.rs", gen.none.snake), gen.no());
    write(dir, "style/gen/style.rs", gen.style());
    let styled_impls = (
        &Str::new("Styled<T>"),
        &[quote! { T: Display }][..],
        &Str::new("style"),
    );
    write(
        dir,
        "style/gen/styled_impls.rs",
        concat(&[
            quote! {
                use crate::*;
                use std::fmt::Display;
                // FIXME
                use std::ops::Not;
                #LINE_BREAK
            },
            gen.impl_styler_index(styled_impls),
            gen.impl_styler_index_mut(styled_impls),
            gen.impl_styler(styled_impls),
            gen.impl_styler_mut(styled_impls),
            gen.impl_styler_ops(styled_impls, false),
            gen.impl_styler_ops(styled_impls, true),
        ]),
    );
    write(
        dir,
        "style/gen/mod.rs",
        concat(&[
            quote! {
                pub mod attributes;
                mod backends;
                mod color;
                mod reset;
                mod style;
                mod styled_impls;
                mod styler;
                #LINE_BREAK

                pub use attributes::*;
                pub use color::*;
                pub use reset::*;
                pub use style::*;
                pub use styler::*;
                #LINE_BREAK
            },
            gen.import_markers(),
        ]),
    );
}

fn concat(streams: &[TokenStream]) -> TokenStream {
    streams.iter().fold(quote! {}, |tokens, stream| {
        quote! { #tokens #LINE_BREAK #stream }
    })
}
