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
            impl_styler!(impl Foreground $self $foreground, get_foreground foreground_mut);
            impl_styler!(impl Background $self $background, get_background background_mut);
            impl_styler!(impl Weighted $self $weight, get_weight weighted_mut);
            impl_styler!(impl Slanted $self $slant, get_slant slanted_mut);
            impl_styler!(impl Blinking $self $blink, get_blink blinking_mut);
            impl_styler!(impl Inverted $self $invert, get_invert inverted_mut);
            impl_styler!(impl Striked $self $strike, get_strike striked_mut);
            impl_styler!(impl Underlined $self $underline, get_underline underlined_mut);
            impl_styler!(impl Overlined $self $overline, get_overline overlined_mut);
            impl_styler!(impl Bordered $self $border, get_border bordered_mut);
        }
    };
    (impl $Type:ident $self:ident $field:expr, $get:ident $set_mut:ident) => {
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
