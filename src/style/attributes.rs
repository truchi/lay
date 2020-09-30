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

            doc!("Returns `" stringify!($Name) "::" stringify!($reset) "`.",
            impl Default for $Name {
                doc!("Returns `" stringify!($Name) "::" stringify!($reset) "`.",
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
                        $($Name::$variant => f.write_str($str),)*
                        $Name::$reset => f.write_str($reset_str)
                    }?;

                    f.write_str("m")
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    use super::*;
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
