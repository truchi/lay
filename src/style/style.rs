use super::{
    Background,
    Blinking,
    Bordered,
    Color,
    Foreground,
    Inverted,
    OptionalStyler,
    Overlined,
    Slanted,
    Striked,
    Styler,
    Underlined,
    Weighted,
};
use std::fmt::{Display, Error, Formatter};

macro_rules! style {
    ($($field:ident $Field:ident: $reset:expr,)*) => {
        style!(impl struct "Styles.", Style $($field $Field )*);
        style!(impl struct "Optional Styles.", OptionalStyle $($field Option<$Field>)*);

        style!(impl reset "A `Style` with fields set to their reset variant.",
            Style $($field $reset )*);
        style!(impl reset "An `OptionalStyle` with fields set to their reset variant.",
            OptionalStyle $($field Some($reset))*);

        style!(impl trait Style Styler $($field)*);
        style!(impl trait [Option] OptionalStyle OptionalStyler $($field)*);
    };
    (impl struct $($doc:expr)*, $Type:ident $($field:ident $Field:ty)*) => {
        doc!($($doc)*,
            #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
            pub struct $Type { $(pub $field: $Field,)* }
        );
    };
    (impl reset $($doc:expr)*, $Type:ident $($field:ident $reset:expr)*) => {
        impl $Type {
            doc!($($doc)*,
                pub const RESET: Self = Self { $($field: $reset,)* };
            );
        }
    };
    (impl trait $([$Option:ident])? $Type:ident $Trait:ident $($field:ident)*) => {
        impl $Trait for $Type {
            impl_styler!($([$Option])? style { $(style.$field,)* });
        }

        impl_styler_ops!($([$Option])? $Type);

        impl Display for $Type {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                <$Type as $Trait>::fmt(self, f)
            }
        }
    };
}

style!(
    foreground Foreground: Foreground(Color::Reset),
    background Background: Background(Color::Reset),
    weighted Weighted: Weighted::ResetWeight,
    slanted Slanted: Slanted::ResetSlant,
    blinking Blinking: Blinking::ResetBlink,
    inverted Inverted: Inverted::ResetInvert,
    striked Striked: Striked::ResetStrike,
    underlined Underlined: Underlined::ResetUnderline,
    overlined Overlined: Overlined::ResetOverline,
    bordered Bordered: Bordered::ResetBorder,
);

impl OptionalStyle {
    /// An `OptionalStyle` with fields set to `None`.
    pub const EMPTY: Self = Self {
        foreground: None,
        background: None,
        weighted:   None,
        slanted:    None,
        blinking:   None,
        inverted:   None,
        striked:    None,
        underlined: None,
        overlined:  None,
        bordered:   None,
    };
}
