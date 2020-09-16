macro_rules! impl_styler {
    ($self:ident {
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
        impl_styler!(impl field $self {
            $foreground,
            $background,
            $weight,
            $slant,
            $blink,
            $invert,
            $strike,
            $underline,
            $overline,
            $border,
        });
    };
    ($self:ident => $style:expr) => {
        impl_styler!(impl method $self {
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
            $style,
        });
    };
    (impl $mode:ident $self:ident {
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
        impl_styler!(impl $mode Foreground $self $foreground, get_foreground foreground_mut);
        impl_styler!(impl $mode Background $self $background, get_background background_mut);
        impl_styler!(impl $mode Weighted $self $weight, get_weighted weighted_mut);
        impl_styler!(impl $mode Slanted $self $slant, get_slanted slanted_mut);
        impl_styler!(impl $mode Blinking $self $blink, get_blinking blinking_mut);
        impl_styler!(impl $mode Inverted $self $invert, get_inverted inverted_mut);
        impl_styler!(impl $mode Striked $self $strike, get_striked striked_mut);
        impl_styler!(impl $mode Underlined $self $underline, get_underlined underlined_mut);
        impl_styler!(impl $mode Overlined $self $overline, get_overlined overlined_mut);
        impl_styler!(impl $mode Bordered $self $border, get_bordered bordered_mut);
    };
    (impl field $Type:ident $self:ident $field:expr, $get:ident $set_mut:ident) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Type> {
            let $self = self;
            $field
        }

        fn $set_mut(&mut self, field: ::std::option::Option<$crate::$Type>) {
            let $self = self;
            $field = field;
        }
    };
    (impl method $Type:ident $self:ident $field:expr, $get:ident $set_mut:ident) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Type> {
            let $self = self;
            $field.$get()
        }

        fn $set_mut(&mut self, field: ::std::option::Option<$crate::$Type>) {
            let $self = self;
            $field.$set_mut(field);
        }
    };
}
