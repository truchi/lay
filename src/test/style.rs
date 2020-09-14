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
pub struct Styles {
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

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct OptionalStyles {
    pub foreground: Option<Foreground>,
    pub background: Option<Background>,
    pub weight:     Option<Weighted>,
    pub slant:      Option<Slanted>,
    pub blink:      Option<Blinking>,
    pub invert:     Option<Inverted>,
    pub strike:     Option<Striked>,
    pub underline:  Option<Underlined>,
    pub overline:   Option<Overlined>,
    pub border:     Option<Bordered>,
}

struct Test {
    stuff: u8,
}

trait WithStuff: WithOptionalStuff {
    fn get_stuff_mut(&mut self) -> &mut u8;
}

trait WithOptionalStuff {
    fn get_stuff_mut(&mut self) -> Option<&mut u8>;
}

impl WithStuff for Test {
    fn get_stuff_mut(&mut self) -> &mut u8 {
        &mut self.stuff
    }
}

impl WithOptionalStuff for Test {
    fn get_stuff_mut(&mut self) -> Option<&mut u8> {
        Some(<Self as WithStuff>::get_stuff_mut(self))
    }
}

pub trait Style {
    fn inherit(self, parent: impl Into<OptionalStyles>) -> Self;
    fn inherit_mut(&mut self, parent: &Self);
    fn dedup(self, before: Self) -> Self;
    fn dedup_mut(&mut self, before: &Self);
}

impl_with!(Styles: styles {
    WithForeground [*]: styles.foreground,
    WithBackground [/]: styles.background,
    WithWeight     [+]: styles.weight,
    WithSlant      [+]: styles.slant,
    WithBlink      [+]: styles.blink,
    WithInvert     [+]: styles.invert,
    WithStrike     [+]: styles.strike,
    WithUnderline  [+]: styles.underline,
    WithOverline   [+]: styles.overline,
    WithBorder     [+]: styles.border,
});

impl_with!(OptionalStyles: styles {
    WithOptionalForeground [*]: styles.foreground,
    WithOptionalBackground [/]: styles.background,
    WithOptionalWeight     [+]: styles.weight,
    WithOptionalSlant      [+]: styles.slant,
    WithOptionalBlink      [+]: styles.blink,
    WithOptionalInvert     [+]: styles.invert,
    WithOptionalStrike     [+]: styles.strike,
    WithOptionalUnderline  [+]: styles.underline,
    WithOptionalOverline   [+]: styles.overline,
    WithOptionalBorder     [+]: styles.border,
});
