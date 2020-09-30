use super::{Style, Styler};
use std::fmt::{Display, Error, Formatter};

#[macro_use]
macro_rules! style {
    ($(($attr:ident: $Attr:ty, $Reset:expr))*) => {
        /// `Style`s.
        ///
        /// A straightforward implementation of `Styler`.
        ///
        /// `Display`s the corresponding CSIs to the terminal.
        ///
        /// `Default`s as an empty `Style` (all fields set to `None`).
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct Style {
            $(pub $attr: Option<$Attr>,)*
        }

        impl Style {
            /// A `Style` with fields set to their reset value.
            pub const RESET: Self = Self {
                $($attr: Some($Reset),)*
            };
        }

        $(impl From<$Attr> for Style {
            fn from($attr: $Attr) -> Self {
                Self::default() + $attr
            }
        })*

        impl_styler!((style: Style) {
            $(style.$attr,)*
        });
    };
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Styler::fmt(self, f)
    }
}
