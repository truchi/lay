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
/// - [Style](crate::style#style)
/// - [StylerIndex](crate::style#stylerindex)
/// - [Styler](crate::style#styler)
/// - [Styled](crate::style#styled)
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
/// ## Style
///
/// Enter the [`Style`](crate::Style) type. A [`Style`](crate::Style) holds an
/// `Option` for each attribute.
///
/// Why are we wrapping with `Option`? To convey ideas such as 'undefined'
/// (display no CSI) or 'inherit' (inherit from some parent attribute, if any).
///
/// It also comes with a [`NONE`](crate::Style::NONE) (aka `Default`) and
/// a [`RESET`](crate::Style::RESET) consts.
///
/// ```
/// # use lay::*;
/// println!("{}Printing with style.", Style {
///     foreground: Some(Foreground(White)),
///     background: Some(Background(Black)),
///     weight: Some(Bold),
///     slant: Some(Italic),
///     ..Style::NONE
/// });
/// ```
/// $ `cargo run --quiet --example style04`
///
/// The `Styler*` traits make it easier to declare and use
/// [`Style`](crate::Style)s. Here's a foretaste:
///
/// ```
/// # use lay::*;
/// println!(
///     "{}Declaring with style",
///     Style::NONE.white().on_black().bold().italic() // Same as above
/// );
/// ```
/// $ `cargo run --quiet --example style05`
///
/// More on that below!
///
/// ## StylerIndex
///
/// The [`StylerIndex`](crate::StylerIndex) trait defines getters
/// for types with `Option`al attributes.
///
/// It will get you clones of `Option`s of attribute from shared references:
///
/// ```
/// # use lay::*;
/// let style = Style {
///     foreground: Some(Foreground(Red)),
///     weight: Some(Bold),
///     ..Style::NONE
/// };
///
/// assert_eq!(style.get_foreground(), Some(Foreground(Red)));
/// assert_eq!(style.get_weight(), Some(Bold));
/// assert_eq!(style.get_slant(), None);
/// ```
///
/// Its little brother [`StylerIndexMut`](`crate::StylerIndexMut`) will get you
/// unique references to `Option`s of attribute from unique references:
///
/// ```
/// # use lay::*;
/// let style = &mut Style {
///     foreground: Some(Foreground(Red)),
///     weight: Some(Bold),
///     ..Style::NONE
/// };
///
/// assert_eq!(style.get_foreground_mut(), &mut Some(Foreground(Red)));
/// assert_eq!(style.get_weight_mut(), &mut Some(Bold));
/// assert_eq!(style.get_slant_mut(), &mut None);
/// ```
///
/// ## Styler
///
/// There are traits to set `Option`al attributes as well:
/// [`Styler`](crate::Styler) and [`StylerMut`](crate::StylerMut).
///
/// Even better, you can set variants or `None` an attribute straight as
/// provided methods!
///
/// ```
/// # use lay::*;
/// // Styler
/// println!(
///     "{}Sweet!",
///     Style::NONE
///         .weight(Bold)  // Sets an attribute
///         .no_slant()    // Nones an attribute
///         .underlined()  // Sets a variant
///         .reset_blink() // That's a variant as well
///         .white()       // Sets a foreground color
///         .on_dark_red() // Sets a background color
/// );
///
/// // StylerMut
/// # #[allow(const_item_mutation)]
/// let style = &mut Style::NONE;
/// style.weight_mut(Bold); //  Sets an attribute
/// style.no_slant_mut(); //    Nones an attribute
/// style.underlined_mut(); //  Sets a variant
/// style.reset_blink_mut(); // That's a variant as well
/// style.white_mut(); //       Sets a foreground color
/// style.on_dark_red_mut(); // Sets a background color
///
/// println!("{}Sick!", style);
/// ```
/// $ `cargo run --quiet --example style06`
///
/// Find out some more provided methods on their own documentation!
///
/// [`Styler`](crate::Styler)'s setters return the
/// [`Output`](crate::Styler::Output) associated type so that this trait is not
/// limited to the [`Style`](crate::Style) type. Actually, [`lay`](crate)'s
/// types implement the `Styler*` traits anywhere applicable.
///
/// ## Styled
///
/// We are going back to the type world to close this tour of
/// [`style`](crate::style) with [`Styled`](crate::Styled).
/// [`Styled`](crate::Styled) associates `Display`able content to
/// [`Style`](crate::Style)s, so it can be printed on its own.
///
/// ```
/// # use lay::*;
/// println!("{}", Styled::from("Wow!").red().on_blue().bold());
/// ```
/// $ `cargo run --quiet --example style07`
///
/// What is the point you ask? It only resets the attributes that were used:
///
/// ```
/// # use lay::*;
/// println!(
///     "{}{}, still underlined.",
///     Underlined,
///     Styled::from("Red").red(),
/// );
/// ```
/// $ `cargo run --quiet --example style08`
///
/// Here is a more contrived example:
///
/// ```
/// # use lay::*;
/// /// Makes a list
/// fn list(title: &str, items: &[&str], selected: usize) -> String {
///     let mut list = String::from(title) + ":\n";
///
///     for (i, item) in items.iter().enumerate() {
///         if i == selected {
///             // Red is flashy, right?
///             list = format!("{}‣ {}\n", list, Styled::from(item).red());
///         } else {
///             list = format!("{}• {}\n", list, Styled::from(item));
///         }
///     }
///
///     list
/// }
///
/// // Print list
/// println!(
///     "{}",
///     Styled::from(list("List", &vec!["Item", "Item", "Item"], 1))
///         .red() // Red can be fancy too!
///         .on_dark_yellow()
///         .bold(),
/// );
///
/// // Wait, what?
/// ```
/// $ `cargo run --quiet --example style09`
///
/// Oh, no! It does not work as expected... It is time to dive into
/// [`layer`](crate::layer)s!
pub mod style;
pub use style::*;
