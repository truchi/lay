use super::{OptionalStyle, OptionalStyler, Style, Styler};
use std::fmt::{Display, Error, Formatter};

/// `Display`able `Style`d content.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Styled<T: Display, U = OptionalStyle> {
    pub content: T,
    pub style:   U,
}

impl<T: Display, U> Styled<T, U> {
    /// Retuns a new `Styled` with `content` and `style`.
    pub fn new(content: T, style: U) -> Self {
        Self { content, style }
    }
}

impl<T: Display, U> From<(T, U)> for Styled<T, U> {
    fn from((content, style): (T, U)) -> Self {
        Self::new(content, style)
    }
}

macro_rules! styled {
    ($(
        $([$Option:ident])? $Style:ident $Trait:ident
        $self:ident : $has_foreground:expr, $has_background:expr, $default:ident,
    )*) => {
        $(
            impl<T: Display> Styled<T, $Style> {
                /// Whether this `Styled` has a visible `Foreground`.
                pub fn has_foreground(&self) -> bool {
                    let $self = self;
                    $has_foreground
                }

                /// Whether this `Styled` has a visible `Background`.
                pub fn has_background(&self) -> bool {
                    let $self = self;
                    $has_background
                }
            }

            impl<T: Display> $Trait for Styled<T, $Style> {
                impl_styler!($([$Option])? styled => styled.style);
            }

            impl_styler_ops!($([$Option])? Styled<T: Display, $Style,>);

            impl<T: Display> From<T> for Styled<T, $Style> {
                fn from(content: T) -> Self {
                    Self::new(content, $Style::$default)
                }
            }
        )*
    };
}

styled!(
    Style Styler _styled:
        true,
        true,
        RESET,
    [Option] OptionalStyle OptionalStyler styled:
        styled.style.get_foreground().is_some(),
        styled.style.get_background().is_some(),
        EMPTY,
);

impl<T: Display> Display for Styled<T, Style> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}", self.style, self.content)
    }
}

impl<T: Display> Display for Styled<T, OptionalStyle> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}{}", self.style, self.content, !self.style)
    }
}
