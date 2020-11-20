use crate::*;

impl Generation {
    pub fn example_style(&self) -> Vec<Vec<(String, TokenStream)>> {
        vec![
            vec![(
                format!(
                    "# Styling utilities.

                    This module contains utilities
                    to work with terminal CSIs and styled types."
                ),
                quote! {
                    #LINE_BREAK
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
                },
            )],
            vec![(
                format!(
                    "> **Heads up!**

                    This doc is generated from the `style` example.
                    Run it to print the code blocks along with the comments!

                    $ `clear; cargo run --quiet --example style`"
                ),
                quote! {},
            )],
            vec![(
                format!(
                    "## Color

                        The [`crate::Color`] enum is surely no surprises for you!

                        It lists all the available colors of the terminal, with dark variants,
                        and with `Rgb`, `Ansi` and `ResetColor` variants.

                        It does not `Display`s by itself though. Read on!"
                ),
                quote! {},
            )],
            vec![(
                format!(
                    "## Attributes

                        You can use the following types to print CSIs to the terminal:
                        - [`crate::Foreground`] tuple struct: `Foreground(Color)`
                        - [`crate::Background`] tuple struct: `Background(Color)`
                        - [`crate::Weight`] enum: `Bold`, `Light`, `ResetWeight`
                        - [`crate::Slant`] enum: `Italic`, `ResetSlant`
                        - [`crate::Blink`] enum: `Slow`, `Fast`, `ResetBlink`
                        - [`crate::Invert`] enum: `Inverted`, `ResetInvert`
                        - [`crate::Strike`] enum: `Striked`, `ResetStrike`
                        - [`crate::Underline`] enum: `Underlined`, `ResetUnderline`
                        - [`crate::Overline`] enum: `Overlined`, `ResetOverline`
                        - [`crate::Border`] enum: `Frame`, `Circle`, `ResetBorder`
                        - [`crate::Reset`] unit struct

                        They `Display` the CSI they represent. Some basic examples:"
                ),
                quote! {
                    println!("{}Red.{} Not red.", Foreground(Red), Foreground(ResetColor));
                    println!("{}On Green{}. Not on green.", Background(Green), Background(ResetColor));
                    println!("{}Bold{}. Not bold.", Bold, ResetWeight);
                    println!(
                        "{}{}{}Multiple attributes, one reset.{} Not styled.",
                        Foreground(Red),
                        Background(Green),
                        Bold,
                        Reset
                    );
                    #LINE_BREAK
                },
            )],
            vec![(format!("Easy, right?"), quote! {})],
            vec![(
                format!(
                    "## Styling

                    We want to individually wrap styling attributes with `Option`s
                    to convey ideas such as 'undefined' (display no CSI)
                    or 'inherit' (inherit from some parent attribute, if any)."
                ),
                quote! {},
            )],
            vec![
                (
                    format!(
                        "### `Styler`

                        The [`crate::Styler`] trait is at the heart of styles. It defines getters
                        and setters for types with `Option`al attributes:"
                    ),
                    quote! {
                        #LINE_BREAK
                        // TODO comments in comment!()
                        // NOTE: Style implements Styler, see below
                        let style = Style::default() // All fields are None
                        .red() // Red foreground
                        .on_green() // Green background
                        .bold() // Bold text
                        .reset_blink(); // Reset blink

                        assert_eq!(style.get_foreground(), Some(Foreground(Red)));
                        assert_eq!(style.get_background(), Some(Background(Green)));
                        assert_eq!(style.get_weight(), Some(Bold));
                        assert_eq!(style.get_blink(), Some(ResetBlink));
                        assert_eq!(style.get_slant(), None); // etc..
                        #LINE_BREAK
                    },
                ),
                (
                    format!(
                        "It also provides convenients methods for styles manipulation:
                        `and` (`Option::and` fields), `or` (`Option::or` fields), `xor`
                        (`Option::xor` fields), `dedup` (`None`s when identical
                        fields), `reset` (reset `Some` fields).",
                    ),
                    quote! {},
                ),
            ],
        ]
    }
}
