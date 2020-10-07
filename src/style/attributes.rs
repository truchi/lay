use crate::*;

impl_styler_index!(
    (weight: Weight) {
        None, None, Some(*weight), None, None, None, None, None, None, None,
    }
    (slant: Slant) {
        None, None, None, Some(*slant), None, None, None, None, None, None,
    }
    (blink: Blink) {
        None, None, None, None, Some(*blink), None, None, None, None, None,
    }
    (invert: Invert) {
        None, None, None, None, None, Some(*invert), None, None, None, None,
    }
    (strike: Strike) {
        None, None, None, None, None, None, Some(*strike), None, None, None,
    }
    (underline: Underline) {
        None, None, None, None, None, None, None, Some(*underline), None, None,
    }
    (overline: Overline) {
        None, None, None, None, None, None, None, None, Some(*overline), None,
    }
    (border: Border) {
        None, None, None, None, None, None, None, None, None, Some(*border),
    }
);

macro_rules! attributes {
    ($(
        $(#[$meta:meta])*
        $Name:ident:
            $($variant:ident($str:literal))* + $reset:ident($reset_str:literal)
    )*) => {
        $(
            pub use $Name::*;

            $(#[$meta])*
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum $Name {
                $($variant,)*
                $reset
            }

            $crate::doc!("Returns `" stringify!($Name) "::" stringify!($reset) "`.",
            impl Default for $Name {
                $crate::doc!("Returns `" stringify!($Name) "::" stringify!($reset) "`.",
                fn default() -> Self {
                    Self::$reset
                });
            });

            /// Prints the corresponding CSI to the terminal.
            impl Display for $Name {
                /// Prints the corresponding CSI to the terminal.
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    f.write_str("\x1B[")?;

                    match self {
                        $Name::$reset => f.write_str($reset_str),
                        $($Name::$variant => f.write_str($str),)*
                    }?;

                    f.write_str("m")
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        assert_eq!(Weight::default(), ResetWeight);
        assert_eq!(Slant::default(), ResetSlant);
        assert_eq!(Blink::default(), ResetBlink);
        assert_eq!(Invert::default(), ResetInvert);
        assert_eq!(Strike::default(), ResetStrike);
        assert_eq!(Underline::default(), ResetUnderline);
        assert_eq!(Overline::default(), ResetOverline);
        assert_eq!(Border::default(), ResetBorder);
    }
}
