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
            impl_styler!(impl Weighted $self $weight, get_weighted weighted_mut);
            impl_styler!(impl Slanted $self $slant, get_slanted slanted_mut);
            impl_styler!(impl Blinking $self $blink, get_blinking blinking_mut);
            impl_styler!(impl Inverted $self $invert, get_inverted inverted_mut);
            impl_styler!(impl Striked $self $strike, get_striked striked_mut);
            impl_styler!(impl Underlined $self $underline, get_underlined underlined_mut);
            impl_styler!(impl Overlined $self $overline, get_overlined overlined_mut);
            impl_styler!(impl Bordered $self $border, get_bordered bordered_mut);
        }
    };
    (impl $Type:ident $self:ident $field:expr, $get:ident $set_mut:ident) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Type> {
            let $self = self;
            $field
        }

        fn $set_mut(&mut self, field: ::std::option::Option<$crate::$Type>) {
            let $self = self;
            $field = field;
        }
    };
}
