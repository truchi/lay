#![allow(unused)]

#[macro_use]
mod lay2;

#[macro_use]
mod utils;
// #[macro_use]
// mod lay;

// mod style;

use lay2::*;
// use lay::*;
use proc_macro2::TokenStream;
use quote::quote;
use utils::*;

// consts!(
// I No Reset Get On Mut
//
// Color
// [White Black (Dark) Grey Red Green Yellow Blue Magenta Cyan]
// Rgb Ansi
//
// [Foreground(Fg) Background(Bg)]
// [
// Weight(Wgt)    [Bold Light]
// Slant(Slt)     [Italic]
// Underline(Udl) [Underlined]
// Strike(Str)    [Striked]
// Overline(Ovl)  [Ovelined]
// Invert(Inv)    [Inverted]
// Blink(Blk)     [Slow Fast]
// Border(Brd)    [Circle Frame]
// ]
// );

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

pub struct Generation<'a>(&'a Lay);

pub fn generate() {
    if let Ok(profile) = std::env::var("PROFILE") {
        if profile == "debug" {
            panic!("{:#?}", LAY);
            // let lay = Lay::new();
            // let gen = Generation(&lay);

            // panic!("{:#?}", lay);

            // This script does an aweful lot of heap allocations...
            println!("cargo:rerun-if-changed=build/mod.rs");

            // write_part("style/mod.rs", "import_markers",
            // gen.import_markers());
            //
            // write(&format!("style/{}.rs", lay.color.lower()), gen.color());
            // write(&format!("style/{}.rs", lay.reset.lower()), gen.reset());
            // write(&format!("style/{}.rs", lay.i.lower()), gen.i());
            // write(&format!("style/{}.rs", lay.no.lower()), gen.no());
            // write(
            // &format!("style/attributes/mod.rs"),
            // gen.mod_style_attributes(),
            // );
            // write(
            // &format!("style/attributes/{}.rs", lay.foreground.lower()),
            // gen.foreground(),
            // );
            // write(
            // &format!("style/attributes/{}.rs", lay.background.lower()),
            // gen.background(),
            // );
            // write("style/style.rs", gen.style());
            // write("style/styler.rs", gen.styler());
            //
            // for (name, content) in gen.attributes() {
            // write(&format!("style/attributes/{}.rs", name), content);
            // }

            // panic!(); // To show println calls...
        }
    }
}
