use super::*;
use std::fmt::{Display, Error, Formatter};

/// `Reset`s all terminal attributes.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Reset;

/// Prints the "Reset"/"Normal" csi to the terminal.
impl Display for Reset {
    /// Prints the "Reset"/"Normal" csi to the terminal.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("\x1B[0m")
    }
}

macro_rules! reset {
    (
        Colors { $($Color:ident,)* }
        Attributes { $($Attr:ident $reset_attr:ident,)* }
    ) => {
        $(reset!([From $Color]
            stringify!($Color) "(Color::ResetColor)",
            $Color(Color::ResetColor)
        );)*

        $(reset!([From $Attr]
            stringify!($Attr) "::" stringify!($reset_attr),
            $Attr::$reset_attr
        );)*
    };
    ([From $Self:ident] $($doc:expr)*, $body:expr) => {
        doc!("Returns `Some(" $($doc)* ")`.",
        impl From<Reset> for Option<$Self> {
            doc!("Returns `Some(" $($doc)* ")`.",
            fn from(_: Reset) -> Self {
                Some($body)
            });
        });
    };
}

reset!(
    Colors {
        Foreground,
        Background,
    }
    Attributes {
        Weight ResetWeight,
        Slant ResetSlant,
        Blink ResetBlink,
        Invert ResetInvert,
        Strike ResetStrike,
        Underline ResetUnderline,
        Overline ResetOverline,
        Border ResetBorder,
    }
);

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
