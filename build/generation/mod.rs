#![allow(unused)]

#[macro_use]
mod utils;
#[macro_use]
mod lay;

mod style;

use lay::*;
use proc_macro2::TokenStream;
use quote::quote;
use utils::*;

consts!(
    I No Reset Get On Mut

    Color
    [White Black (Dark) Grey Red Green Yellow Blue Magenta Cyan]
    Rgb Ansi

    [Foreground(Fg) Background(Bg)]
    [
        Weight(Wgt)    [Bold Light]
        Slant(Slt)     [Italic]
        Underline(Udl) [Underlined]
        Strike(Str)    [Striked]
        Overline(Ovl)  [Ovelined]
        Invert(Inv)    [Inverted]
        Blink(Blk)     [Slow Fast]
        Border(Brd)    [Circle Frame]
    ]
);

pub struct Generation<'a>(&'a Lay);

pub fn generate() {
    if let Ok(profile) = std::env::var("PROFILE") {
        if profile == "debug" {
            let lay = Lay::new();
            let gen = Generation(&lay);

            // panic!("{:#?}", lay);
            println!("cargo:rerun-if-changed=build/mod.rs");

            write_part("style/mod.rs", "import_markers", gen.import_markers());

            write(&format!("style/{}.rs", lay.color.lower()), gen.color());
            write(&format!("style/{}.rs", lay.reset.lower()), gen.reset());
            write(&format!("style/{}.rs", lay.i.lower()), gen.i());
            write(&format!("style/{}.rs", lay.no.lower()), gen.no());
            write(
                &format!("style/attributes/mod.rs"),
                gen.mod_style_attributes(),
            );
            write(
                &format!("style/attributes/{}.rs", lay.foreground.lower()),
                gen.foreground(),
            );
            write(
                &format!("style/attributes/{}.rs", lay.background.lower()),
                gen.background(),
            );
            write("style/style.rs", gen.style());
            write("style/styler.rs", gen.styler());

            for (name, content) in gen.attributes() {
                write(&format!("style/attributes/{}.rs", name), content);
            }

            // panic!(); // To show println calls...
        }
    }
}
