macro_rules! impl_styler {
    ($Type:ident $(<$($G:ident $(: $B:tt)?,)+>)? $self:ident {
        $foreground:expr,
        $background:expr,
        $weight:expr,
        $slant:expr,
        $blink:expr,
        $invert:expr,
        $strike:expr,
        $underline:expr,
        $overline:expr,
        $border:expr,
    }) => {
        impl $(<$($G $(: $B)?,)+>)? $crate::Styler for $Type $(<$($G,)+>)? {
            impl_styler!(impl $crate::Foreground, $self $foreground, get_foreground foreground_mut);
            impl_styler!(impl $crate::Background, $self $background, get_background background_mut);
            impl_styler!(impl $crate::Weighted, $self $weight, get_weighted weighted_mut);
            impl_styler!(impl $crate::Slanted, $self $slant, get_slanted slanted_mut);
            impl_styler!(impl $crate::Blinking, $self $blink, get_blinking blinking_mut);
            impl_styler!(impl $crate::Inverted, $self $invert, get_inverted inverted_mut);
            impl_styler!(impl $crate::Striked, $self $strike, get_striked striked_mut);
            impl_styler!(impl $crate::Underlined, $self $underline, get_underlined underlined_mut);
            impl_styler!(impl $crate::Overlined, $self $overline, get_overlined overlined_mut);
            impl_styler!(impl $crate::Bordered, $self $border, get_bordered bordered_mut);
        }
    };
    (impl $Type:ty, $self:ident $field:expr, $get:ident $set_mut:ident) => {
        fn $get(&self) -> $Type {
            let $self = self;
            $field
        }

        fn $set_mut(&mut self, field: $Type) {
            let $self = self;
            $field = field;
        }
    };
}
