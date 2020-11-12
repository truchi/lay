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

Lay!(
    Reset reset
    No    no
    Color color
    [White Black (Dark) Grey Red Green Yellow Blue Magenta Cyan] Rgb Ansi
    [white black (dark) grey red green yellow blue magenta cyan] rgb ansi
    Foreground foreground (Fg)
    Background background (Bg)
    [
      0 Weight(Wgt)    [Bold Light]
        weight         [bold light]
      1 Slant(Slt)     [Italic]
        slant          [italic]
      2 Underline(Udl) [Underlined]
        underline      [underlined]
      3 Strike(Str)    [Striked]
        strike         [striked]
      4 Overline(Ovl)  [Ovelined]
        overline       [ovelined]
      5 Invert(Inv)    [Inverted]
        invert         [inverted]
      6 Blink(Blk)     [Slow Fast]
        blink          [slow fast]
      7 Border(Brd)    [Circle Frame]
        border         [circle frame]
    ]
);

pub struct Generation;

pub fn generate() {
    if let Ok(profile) = std::env::var("PROFILE") {
        if profile == "debug" {
            // panic!("{:#?}", LAY);
            println!("cargo:rerun-if-changed=build/mod.rs");

            write_part(
                "style/mod.rs",
                "import_markers",
                Generation::import_markers(),
            );

            write(&format!("style/{}.rs", COLOR.snake), Generation::color());
            write(
                &format!("style/{}.rs", RESET.to_lowercase()),
                Generation::reset(),
            );
            write(&format!("style/{}.rs", INDEX), Generation::i());
            write(&format!("style/{}.rs", NONE), Generation::no());
            write(
                &format!("style/attributes/mod.rs"),
                Generation::mod_style_attributes(),
            );
            write("style/style.rs", Generation::style());
            write("style/styler.rs", Generation::styler());

            for ground in GROUNDS {
                write(
                    &format!("style/attributes/{}.rs", ground.snake),
                    Generation::ground(*ground),
                );
            }

            for attr in ATTRS {
                write(
                    &format!("style/attributes/{}.rs", attr.snake),
                    Generation::attr(*attr),
                );
            }
        }
    }
}
