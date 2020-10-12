use super::*;
use std::fmt::{Display, Error, Formatter};

macro_rules! colors {
    ($(
        $(#[$meta_ground:meta])*
        $Ground:ident($ground:ident)($csi:literal $reset:literal)
    )*) => {
        $(
            doc!(
            "A `" stringify!($Ground) "` `Color`.\n\n"
            "Prints the corresponding CSI to the terminal when `Display`ed.\n\n"
            "`Default`s to `" stringify!($Ground) "(Color::ResetColor)`, user's default terminal's " stringify!($ground) " color.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct $Ground(pub Color););

            doc!("Returns `" stringify!($Ground) "(Color::ResetColor)`.",
            impl Default for $Ground {
                doc!("Returns `" stringify!($Ground) "(Color::ResetColor)`.",
                fn default() -> Self {
                    Self(Color::ResetColor)
                });
            });

            doc!("Returns `" stringify!($Ground) "(color)`.",
            impl From<Color> for $Ground {
                doc!("Returns `" stringify!($Ground) "(color)`.",
                fn from(color: Color) -> Self {
                    Self(color)
                });
            });

            /// Returns the inner `Color`.
            impl From<$Ground> for Color {
                /// Returns the inner `Color`.
                fn from($Ground(color): $Ground) -> Self {
                    color
                }
            }

            doc!("Returns `Some(" stringify!($Ground) "(color))`.",
            impl From<Color> for Option<$Ground> {
                doc!("Returns `Some(" stringify!($Ground) "(color))`.",
                fn from(color: Color) -> Self {
                    Some($Ground(color))
                });
            });

            impl_styler!((g: $Ground) -> Style {
                (foreground) Style::NONE.$ground(Some(g)).foreground(foreground),
                (background) Style::NONE.$ground(Some(g)).background(background),
                (weight)     Style::NONE.$ground(Some(g)).weight(weight),
                (slant)      Style::NONE.$ground(Some(g)).slant(slant),
                (blink)      Style::NONE.$ground(Some(g)).blink(blink),
                (invert)     Style::NONE.$ground(Some(g)).invert(invert),
                (strike)     Style::NONE.$ground(Some(g)).strike(strike),
                (underline)  Style::NONE.$ground(Some(g)).underline(underline),
                (overline)   Style::NONE.$ground(Some(g)).overline(overline),
                (border)     Style::NONE.$ground(Some(g)).border(border),
            });

            #[cfg(feature = "styler-ops")]
            impl_styler_ops!(($Ground) -> Style);

            /// Prints the corresponding CSI to the terminal.
            impl Display for $Ground {
                /// Prints the corresponding CSI to the terminal.
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    let color = self.0;

                    f.write_str("\x1B[")?;

                    if color == Color::ResetColor {
                        f.write_str($reset)?;
                    } else {
                        f.write_str($csi)?;

                        match color {
                            Color::White => f.write_str("5;15"),
                            Color::Black => f.write_str("5;0"),
                            Color::Grey => f.write_str("5;7"),
                            Color::DarkGrey => f.write_str("5;8"),
                            Color::Red => f.write_str("5;9"),
                            Color::DarkRed => f.write_str("5;1"),
                            Color::Green => f.write_str("5;10"),
                            Color::DarkGreen => f.write_str("5;2"),
                            Color::Yellow => f.write_str("5;11"),
                            Color::DarkYellow => f.write_str("5;3"),
                            Color::Blue => f.write_str("5;12"),
                            Color::DarkBlue => f.write_str("5;4"),
                            Color::Magenta => f.write_str("5;13"),
                            Color::DarkMagenta => f.write_str("5;5"),
                            Color::Cyan => f.write_str("5;14"),
                            Color::DarkCyan => f.write_str("5;6"),
                            Color::Rgb(r, g, b) => write!(f, "2;{};{};{}", r, g, b),
                            Color::Ansi(ansi) => write!(f, "5;{}", ansi),
                            _ => Ok(()) // NOTE: unreachable
                        }?;
                    }

                    f.write_str("m")
                }
            }
        )*
    };
}

colors!(
    Foreground(foreground)("38;" "39")
    Background(background)("48;" "49")
);

/// Returns `Foreground(color)`.
impl From<Background> for Foreground {
    /// Returns `Foreground(color)`.
    fn from(Background(color): Background) -> Self {
        Self(color)
    }
}

/// Returns `Background(color)`.
impl From<Foreground> for Background {
    /// Returns `Background(color)`.
    fn from(Foreground(color): Foreground) -> Self {
        Self(color)
    }
}

impl_styler_index!(
    (foreground: Foreground) {
        Some(*foreground), None, None, None, None, None, None, None, None, None,
    }
    (background: Background) {
        None, Some(*background), None, None, None, None, None, None, None, None,
    }
);

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        // Foreground default to Foreground(ResetColor)
        assert_eq!(Foreground::default(), Foreground(ResetColor));
        // Background default to Background(ResetColor)
        assert_eq!(Background::default(), Background(ResetColor));
    }

    #[test]
    fn conversion() {
        // Foreground/Background <-> Color
        assert_eq!(Color::from(Foreground(Blue)), Blue);
        assert_eq!(Foreground::from(Green), Foreground(Green));
        assert_eq!(Color::from(Background(Red)), Red);
        assert_eq!(Background::from(Magenta), Background(Magenta));

        // Color -> Option<Foreground/Background>
        let foreground: Option<Foreground> = Magenta.into();
        assert_eq!(foreground, Some(Foreground(Magenta)));
        let background: Option<Background> = Magenta.into();
        assert_eq!(background, Some(Background(Magenta)));

        // Foreground <-> Background
        assert_eq!(Foreground::from(Background(Black)), Foreground(Black));
        assert_eq!(Background::from(Foreground(White)), Background(White));
    }

    #[test]
    fn styler_index() {
        let foreground = Foreground(Grey);
        let background = Background(Red);

        macro_rules! styler_index {
            ($($var:ident $get_some:ident $($get_none:ident)*,)*) => {
                $(
                    assert_eq!($var.$get_some(), Some($var));
                    $(assert_eq!($var.$get_none(), None);)*
                )*
            };
        }

        styler_index!(
            foreground get_foreground get_background get_weight get_slant get_blink
            get_invert get_strike get_underline get_overline get_border,
            background get_background get_foreground get_weight get_slant get_blink
            get_invert get_strike get_underline get_overline get_border,
        );
    }

    #[test]
    fn styler() {
        let foreground = Foreground(Grey);
        assert_eq!(foreground.foreground(Blue), Style {
            foreground: Some(Foreground(Blue)),
            ..Style::NONE
        });
        assert_eq!(foreground.background(Blue), Style {
            foreground: Some(foreground),
            background: Some(Background(Blue)),
            ..Style::NONE
        });

        let background = Background(Yellow);
        assert_eq!(background.foreground(Red), Style {
            foreground: Some(Foreground(Red)),
            background: Some(background),
            ..Style::NONE
        });
        assert_eq!(background.background(Green), Style {
            background: Some(Background(Green)),
            ..Style::NONE
        });

        macro_rules! styler {
            ($($var:ident)*) => {
                $(styler!($var
                    .weight(Bold)
                    .slant(Italic)
                    .blink(Slow)
                    .invert(Inverted)
                    .strike(Striked)
                    .underline(Underlined)
                    .overline(Overlined)
                    .border(Frame)
                );)*
            };
            ($var:ident $(.$attr:ident($Attr:ident))*) => {
                $(assert_eq!($var.$attr($Attr), Style {
                    $var: Some($var),
                    $attr: Some($Attr),
                    ..Style::NONE
                });)*
            };
        }

        styler!(foreground background);
    }

    // TODO test styler ops
}
