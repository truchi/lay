#![allow(non_snake_case)]

#[macro_use]
mod utils;
#[macro_use]
mod lay;

mod style;
mod write;

use lay::*;
use proc_macro2::TokenStream;
use quote::quote;
use utils::*;
use write::*;

const LAY: Lay = lay!(
    ("Reset" "No")
    "Color": ["White" "Black"] ("Rgb" "Ansi")
    "Dark": ["Grey" "Red" "Green" "Yellow" "Blue" "Magenta" "Cyan"]
    ["Fg"/"Foreground" "Bg"/"Background"]
    "Wgt"/"Weight":    ["Bold" "Light"]
    "Slt"/"Slant":     ["Italic"]
    "Udl"/"Underline": ["Underlined"]
    "Str"/"Strike":    ["Striked"]
    "Ovl"/"Overline":  ["Ovelined"]
    "Inv"/"Invert":    ["Inverted"]
    "Blk"/"Blink":     ["Slow" "Fast"]
    "Brd"/"Border":    ["Circle" "Frame"]
);

pub fn generate() {
    if let Ok(profile) = std::env::var("PROFILE") {
        if profile == "debug" {
            // panic!("{:#?}", LAY);
            println!("cargo:rerun-if-changed=build/mod.rs");

            write_part("style/mod.rs", "import_markers", LAY.import_markers());
            write("style/color.rs", LAY.color());
            write("style/reset.rs", LAY.reset());
            write("style/i.rs", LAY.i());
            write("style/no.rs", LAY.no());
            write("style/attributes/mod.rs", LAY.mod_style_attributes());
            write("style/attributes/foreground.rs", LAY.foreground());
            write("style/attributes/background.rs", LAY.background());
            write("style/styler.rs", LAY.styler());
            write("style/style.rs", LAY.style());

            for (name, content) in LAY.attributes() {
                write(&format!("style/attributes/{}.rs", name), content);
            }

            // panic!(); // To show println calls...
        }
    }
}
