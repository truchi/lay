/// # Styling utilities.
///
/// This module contains utilities to work with terminal CSIs and styled types.
///
/// ```
/// # use lay::*;
/// println!(
///     "{on_black}{green}{blink}{weight}# Styling utilities.{reset_blink}
///
///     {blue}{slant}This module contains utilities to work with terminal CSIs and styled types.{reset}",
///     on_black = Background(Rgb(0, 0, 0)),
///     green = Foreground(Green),
///     blue = Foreground(Blue),
///     blink = Fast,
///     reset_blink = ResetBlink,
///     weight = Bold,
///     slant = Italic,
///     reset = Reset,
/// );
/// ```
/// $ `cargo run --quiet --example style01`
///
/// - [Color](crate::style#color)
/// - [Attributes](crate::style#attributes)
/// - [Reset](crate::style#reset)
/// - [Styling](crate::style#styling)
///   - [Style](crate::style#style)
///   - [Styler](crate::style#styler)
///
/// ## Color
///
/// The [`Color`](crate::Color) enum is surely no surprises for you!
///
/// It lists all the available colors of the terminal, with dark variants,
/// and with `Rgb`, `Ansi` and `ResetColor` variants.
///
/// As all enums in [`lay`](crate), its variants are re-exported:
///
/// ```
/// # use lay::*;
/// assert_eq!(White, Color::White);
/// ```
///
/// It does not `Display`s by itself though. Read on!
///
/// ## Attributes
///
/// You can use the following types to print CSIs to the terminal:
/// - [`Foreground`](crate::Foreground) tuple struct: `Foreground(Color)`
/// - [`Background`](crate::Background) tuple struct: `Background(Color)`
/// - [`Weight`](crate::Weight) enum: `Bold`, `Light`, `ResetWeight`
/// - [`Slant`](crate::Slant) enum: `Italic`, `ResetSlant`
/// - [`Blink`](crate::Blink) enum: `Slow`, `Fast`, `ResetBlink`
/// - [`Invert`](crate::Invert) enum: `Inverted`, `ResetInvert`
/// - [`Strike`](crate::Strike) enum: `Striked`, `ResetStrike`
/// - [`Underline`](crate::Underline) enum: `Underlined`, `ResetUnderline`
/// - [`Overline`](crate::Overline) enum: `Overlined`, `ResetOverline`
/// - [`Border`](crate::Border) enum: `Frame`, `Circle`, `ResetBorder`
///
/// They `Display` the CSI they represent. Some basic examples:
///
/// ```
/// # use lay::*;
/// println!("{}Red.{} Not red.", Foreground(Red), Foreground(ResetColor));
/// println!(
///     "{}On Green{}. Not on green.",
///     Background(Green),
///     Background(ResetColor)
/// );
/// println!("{}Bold{}. Not bold.", Bold, ResetWeight);
/// ```
/// $ `cargo run --quiet --example style02`
///
/// ## Reset
///
/// The [`Reset`](crate::Reset) unit struct will reset all attributes
/// sent to the screen.
///
/// Note that "reseting" ([`Reset`](crate::Reset) and `Reset*` variants) means
/// going back to the user's default attribute(s).
///
/// ```
/// # use lay::*;
/// println!(
///     "{}{}{}Multiple attributes, one reset.{} Not styled.",
///     Foreground(Red),
///     Background(Green),
///     Bold,
///     Reset
/// );
/// ```
/// $ `cargo run --quiet --example style03`
///
/// Easy, right?
///
/// ## Styling
///
/// We want to individually wrap styling attributes with `Option`s
/// to convey ideas such as 'undefined' (display no CSI)
/// or 'inherit' (inherit from some parent attribute, if any).
///
/// ### `Style`
///
/// Enter the [`Style`](crate::Style) type. A [`Style`](crate::Style) holds an
/// `Option` for each attribute. It also comes with
/// a [`NONE`](crate::Style::NONE) (aka `Default`) and
/// a [`RESET`](crate::Style::RESET) consts.
///
/// ```
/// # use lay::*;
/// let style = Style {
///     foreground: Some(Foreground(White)),
///     background: Some(Background(Black)),
///     weight: Some(Bold),
///     slant: Some(Italic),
///     ..Style::NONE
/// };
///
/// println!("{}Printing with style", style);
/// ```
/// $ `cargo run --quiet --example style04`
///
/// The `Styler*` trait and `std::ops::*` make it easier to declare and use
/// [`Style`](crate::Style)s. Here's a foretaste:
///
/// ```
/// # use lay::*;
/// // FIXME
/// let style = White / Black * Bold * Italic;
///
/// println!("{}Declaring with style", style);
/// ```
/// $ `cargo run --quiet --example style05`
///
/// ### `Styler`
///
/// The [`Styler`](crate::Styler) trait is at the heart of styles. It defines
/// getters and setters for types with `Option`al attributes:
///
/// ```
/// # use lay::*;
/// let style = Style::default().red().on_green().bold().reset_blink();
/// assert_eq!(style.get_foreground(), Some(Foreground(Red)));
/// assert_eq!(style.get_background(), Some(Background(Green)));
/// assert_eq!(style.get_weight(), Some(Bold));
/// assert_eq!(style.get_blink(), Some(ResetBlink));
/// assert_eq!(style.get_slant(), None);
/// ```
///
/// It also provides convenients methods for styles manipulation:
/// `and` (`Option::and` fields), `or` (`Option::or` fields), `xor`
/// (`Option::xor` fields), `dedup` (`None`s when identical
/// fields), `reset` (reset `Some` fields).
pub mod style;
pub use style::*;
