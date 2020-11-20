use crate::*;

impl Generation {
    pub fn example_style(&self) -> Vec<Vec<(String, TokenStream)>> {
        vec![
            vec![(
                format!(
                    "Styling utilities.

                    This module contains utilities
                    to work with terminal CSIs and styled types."
                ),
                quote! {
                    #LINE_BREAK
                    println!(
                        "{on_black}{green}{blink}{weight}Styling utilities.{reset_blink}

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
                    "> Heads up!

                    This doc is generated from the `style` example.
                    Run it to print the code blocks along with the comments!

                    $ `clear; cargo run --quiet --example style`"
                ),
                quote! {},
            )],
            vec![
                (
                    format!(
                        "# Colors and attributes

                        You can use the following types to print CSIs to the terminal:
                        - Colors (tuple structs):
                          - `Foreground`: `Foreground(Color)`
                          - `Background`: `Background(Color)`
                        - Attributes (enums):
                          - `Weight`: `Bold`, `Light`, `ResetWeight`
                          - `Slant`: `Italic`, `ResetSlant`
                          - `Blink`: `Slow`, `Fast`, `ResetBlink`
                          - `Invert`: `Inverted`, `ResetInvert`
                          - `Strike`: `Striked`, `ResetStrike`
                          - `Underline`: `Underlined`, `ResetUnderline`
                          - `Overline`: `Overlined`, `ResetOverline`
                          - `Border`: `Frame`, `Circle`, `ResetBorder`

                        All those types `Default` to their reset value: colors default to the user's
                        terminal default foreground/background color, attributes default to the
                        unsetting CSI.

                        They `Display` the CSI they represent. Some basic examples:"
                    ),
                    quote! {
                        println!("{}Red.{} Not red.", Foreground(Red), Foreground(ResetColor));
                        println!("{}On Green{}. Not on green.", Background(Green), Background(ResetColor));
                        println!("{}Bold{}. Not bold.", Bold, ResetWeight);
                        #LINE_BREAK
                    },
                ),
                (
                    format!(
                        "Note that the `Color` enum does not `Display` by itself.

                        In addition, we provide the `Reset` type which represents the CSI
                        to reset all colors/attributes:"
                    ),
                    quote! {
                        println!(
                            "{}{}{}Multiple attributes, one reset.{} Not styled.",
                            Foreground(Red),
                            Background(Green),
                            Bold,
                            Reset
                        );
                    },
                ),
            ],
            vec![(format!("Easy, right?"), quote! {})],
            vec![(
                format!(
                    "# Styling

                    (We will refer to both colors and attributes as 'attributes'.)

                    We want to individually wrap styling attributes with `Option`s to
                    convey ideas such as 'undefined' (display no CSI) or
                    'inherit' (inherit from some parent attribute, if any)."
                ),
                quote! {},
            )],
            vec![
                (
                    format!(
                        "## `Styler`

                        The [`crate::Styler`] trait is at the heart of styles. It defines getters
                        and setters for types with `Option`al attributes:"
                    ),
                    quote! {
                        #LINE_BREAK
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
