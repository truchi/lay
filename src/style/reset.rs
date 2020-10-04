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

        __reset!([From Color] $Reset $($reset_color)*);
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
    ([From Color] $Reset:ident $reset_color:ident $($_:ident)+) => {
        $crate::doc!("Returns `Color::" stringify!($reset_color) "`.",
        impl From<$Reset> for Color {
            $crate::doc!("Returns `Color::" stringify!($reset_color) "`.",
            fn from(_: $Reset) -> Self {
                Self::$reset_color
            });
        });
    };
    ([From $Self:ident] $Reset:ident $($doc:expr)*, $body:expr) => {
        $crate::doc!("Returns `" $($doc)* "`.",
        impl From<$Reset> for $Self {
            $crate::doc!("Returns `" $($doc)* "`.",
            fn from(_: $Reset) -> Self {
                $body
            });
        });
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
        // From Reset
        assert_eq!(Color::from(Reset), ResetColor);
        assert_eq!(Foreground::from(Reset), Foreground(ResetColor));
        assert_eq!(Background::from(Reset), Background(ResetColor));
        assert_eq!(Weight::from(Reset), ResetWeight);
        assert_eq!(Slant::from(Reset), ResetSlant);
        assert_eq!(Blink::from(Reset), ResetBlink);
        assert_eq!(Invert::from(Reset), ResetInvert);
        assert_eq!(Strike::from(Reset), ResetStrike);
        assert_eq!(Underline::from(Reset), ResetUnderline);
        assert_eq!(Overline::from(Reset), ResetOverline);
        assert_eq!(Border::from(Reset), ResetBorder);

        // Option From Reset
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
