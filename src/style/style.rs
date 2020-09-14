use super::{
    Background,
    Blinking,
    Bordered,
    Foreground,
    Inverted,
    Overlined,
    Slanted,
    Striked,
    Underlined,
    Weighted,
};
use std::fmt::{Display, Error, Formatter};

/// `Option`al `Style`s.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Style {
    pub foreground: Foreground,
    pub background: Background,
    pub weight:     Weighted,
    pub slant:      Slanted,
    pub blink:      Blinking,
    pub invert:     Inverted,
    pub strike:     Striked,
    pub underline:  Underlined,
    pub overline:   Overlined,
    pub border:     Bordered,
}

impl Style {
    // /// Inherits fields from `parent`.
    // pub fn inherit(&self, parent: Self) -> Self {
    //     macro_rules! inherit {
    //         ($($field:ident)*) => {
    //             Self {
    //                 $($field: self.$field.or(parent.$field),)*
    //             }
    //         };
    //     }
    //     inherit!(foreground background weight slant blink invert strike underline
    // overline border) }
    //
    // /// Unsets fields if the value is identical to the corresponding one in
    // /// `before`.
    // pub fn simplify(&self, before: Self) -> Self {
    //     macro_rules! simplify {
    //         ($($field:ident)*) => {
    //             Self {
    //                 $($field: match (before.$field, self.$field) {
    //                     (Some(b), Some(s)) if b == s => None,
    //                     _ => self.$field,
    //                 },)*
    //             }
    //         };
    //     }
    //     simplify!(foreground background weight slant blink invert strike
    // underline overline border) }
    //
    // /// Defaults (i.e. resets) fields which are set.
    // pub fn materialize(&self) -> Self {
    //     macro_rules! materialize {
    //         ($($field:ident)*) => {
    //             Self {
    //                 $($field: self.$field.or(Some(Default::default())),)*
    //             }
    //         };
    //     }
    //     materialize!(foreground background weight slant blink invert strike
    // underline overline border) }
    //
    //   /// Defaults (i.e. resets) fields which are set.
    //   pub fn reset(&self) -> Self {
    //       macro_rules! reset {
    //           ($($field:ident)*) => {
    //               Self {
    //                   $($field: self.$field.and(Some(Default::default())),)*
    //               }
    //           };
    //       }
    //       reset!(foreground background weight slant blink invert strike underline
    // overline border)   }
}

// impl_styler!(Style);
// impl_styler_ops!(Style);

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        macro_rules! display {
            ($($field:ident)*) => {
                $(write!(f, "{}", self.$field)?;)*
            };
        }

        display!(foreground background weight slant blink invert strike underline overline border);
        Ok(())
    }
}
