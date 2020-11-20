////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

include!("style.data.rs");
use lay::*;

pub fn main() {
    println!("{}", PART_0); // ;)

    // # Styling utilities.
    //
    // This module contains utilities
    // to work with terminal CSIs and styled types.

    println!(
        "{on_black}{green}{blink}{weight}# Styling utilities.{reset_blink}

{blue}{slant}This module contains utilities
to work with terminal CSIs and styled types.{reset}",
        on_black = Background(Rgb(0, 0, 0)),
        green = Foreground(Green),
        blue = Foreground(Blue),
        blink = Fast,
        reset_blink = ResetBlink,
        weight = Bold,
        slant = Italic,
        reset = Reset,
    );

    println!("{}", PART_1); // ;)

    // > **Heads up!**
    //
    // This doc is generated from the `style` example.
    // Run it to print the code blocks along with the comments!
    //
    // $ `clear; cargo run --quiet --example style`

    println!("{}", PART_2); // ;)

    // ## Color
    //
    // The [`crate::Color`] enum is surely no surprises for you!
    //
    // It lists all the available colors of the terminal, with dark variants,
    // and with `Rgb`, `Ansi` and `ResetColor` variants.
    //
    // It does not `Display`s by itself though. Read on!

    println!("{}", PART_3); // ;)

    // ## Attributes
    //
    // You can use the following types to print CSIs to the terminal:
    // - [`crate::Foreground`] tuple struct: `Foreground(Color)`
    // - [`crate::Background`] tuple struct: `Background(Color)`
    // - [`crate::Weight`] enum: `Bold`, `Light`, `ResetWeight`
    // - [`crate::Slant`] enum: `Italic`, `ResetSlant`
    // - [`crate::Blink`] enum: `Slow`, `Fast`, `ResetBlink`
    // - [`crate::Invert`] enum: `Inverted`, `ResetInvert`
    // - [`crate::Strike`] enum: `Striked`, `ResetStrike`
    // - [`crate::Underline`] enum: `Underlined`, `ResetUnderline`
    // - [`crate::Overline`] enum: `Overlined`, `ResetOverline`
    // - [`crate::Border`] enum: `Frame`, `Circle`, `ResetBorder`
    // - [`crate::Reset`] unit struct
    //
    // They `Display` the CSI they represent. Some basic examples:
    println!("{}Red.{} Not red.", Foreground(Red), Foreground(ResetColor));
    println!(
        "{}On Green{}. Not on green.",
        Background(Green),
        Background(ResetColor)
    );
    println!("{}Bold{}. Not bold.", Bold, ResetWeight);
    println!(
        "{}{}{}Multiple attributes, one reset.{} Not styled.",
        Foreground(Red),
        Background(Green),
        Bold,
        Reset
    );

    println!("{}", PART_4); // ;)

    // Easy, right?

    println!("{}", PART_5); // ;)

    // ## Styling
    //
    // We want to individually wrap styling attributes with `Option`s
    // to convey ideas such as 'undefined' (display no CSI)
    // or 'inherit' (inherit from some parent attribute, if any).

    println!("{}", PART_6); // ;)

    // ### `Styler`
    //
    // The [`crate::Styler`] trait is at the heart of styles. It defines getters
    // and setters for types with `Option`al attributes:

    let style = Style::default().red().on_green().bold().reset_blink();
    assert_eq!(style.get_foreground(), Some(Foreground(Red)));
    assert_eq!(style.get_background(), Some(Background(Green)));
    assert_eq!(style.get_weight(), Some(Bold));
    assert_eq!(style.get_blink(), Some(ResetBlink));
    assert_eq!(style.get_slant(), None);

    // It also provides convenients methods for styles manipulation:
    // `and` (`Option::and` fields), `or` (`Option::or` fields), `xor`
    // (`Option::xor` fields), `dedup` (`None`s when identical
    // fields), `reset` (reset `Some` fields).
}
