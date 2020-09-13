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

impl_with!(Style: style {
    foreground [*]: style.foreground,
    background [/]: style.background,
    weight     [+]: style.weight,
    slant      [+]: style.slant,
    blink      [+]: style.blink,
    invert     [+]: style.invert,
    strike     [+]: style.strike,
    underline  [+]: style.underline,
    overline   [+]: style.overline,
    border     [+]: style.border,
});
