macro_rules! reset {
    ($(#[$meta_reset:meta])* $Reset:ident
        Colors { $($Color:ident $reset_color:ident)* }
        Attributes { $($Attr:ident $reset_attr:ident)* }
    ) => {
        $(#[$meta_reset])*
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
        pub struct $Reset;

        /// Prints the "Reset"/"Normal" csi to the terminal.
        impl Display for $Reset {
            /// Prints the "Reset"/"Normal" csi to the terminal.
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                f.write_str("\x1B[0m")
            }
        }

        $(__reset!([From $Color] $Reset
            stringify!($Color) "(Color::" stringify!($reset_color) ")",
            $Color(Color::$reset_color)
        );)*
        $(__reset!([From $Attr] $Reset
            stringify!($Attr) "::" stringify!($reset_attr),
            $Attr::$reset_attr
        );)*
    };
}

macro_rules! __reset {
    ([From $Self:ident] $Reset:ident $($doc:expr)*, $body:expr) => {
        $crate::doc!("Returns `Some(" $($doc)* ")`.",
        impl From<$Reset> for Option<$Self> {
            $crate::doc!("Returns `Some(" $($doc)* ")`.",
            fn from(_: $Reset) -> Self {
                Some($body)
            });
        });
    };

}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn conversion() {
        assert_eq!(
            Option::<Foreground>::from(Reset),
            Some(Foreground(ResetColor))
        );
        assert_eq!(
            Option::<Background>::from(Reset),
            Some(Background(ResetColor))
        );
        assert_eq!(Option::<Weight>::from(Reset), Some(ResetWeight));
        assert_eq!(Option::<Slant>::from(Reset), Some(ResetSlant));
        assert_eq!(Option::<Blink>::from(Reset), Some(ResetBlink));
        assert_eq!(Option::<Invert>::from(Reset), Some(ResetInvert));
        assert_eq!(Option::<Strike>::from(Reset), Some(ResetStrike));
        assert_eq!(Option::<Underline>::from(Reset), Some(ResetUnderline));
        assert_eq!(Option::<Overline>::from(Reset), Some(ResetOverline));
        assert_eq!(Option::<Border>::from(Reset), Some(ResetBorder));
    }
}
