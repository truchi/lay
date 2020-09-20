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
        impl_styler!(impl $mode Foreground get_foreground foreground_mut $self $foreground);
        impl_styler!(impl $mode Background get_background background_mut $self $background);
        impl_styler!(impl $mode Weighted get_weighted weighted_mut $self $weight);
        impl_styler!(impl $mode Slanted get_slanted slanted_mut $self $slant);
        impl_styler!(impl $mode Blinking get_blinking blinking_mut $self $blink);
        impl_styler!(impl $mode Inverted get_inverted inverted_mut $self $invert);
        impl_styler!(impl $mode Striked get_striked striked_mut $self $strike);
        impl_styler!(impl $mode Underlined get_underlined underlined_mut $self $underline);
        impl_styler!(impl $mode Overlined get_overlined overlined_mut $self $overline);
        impl_styler!(impl $mode Bordered get_bordered bordered_mut $self $border);
    };
    (impl field $Type:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Type> {
            let $self = self;
            $field
        }

        fn $set_mut(&mut self, arg: ::std::option::Option<$crate::$Type>) {
            let $self = self;
            $field = arg;
        }
    };
    (impl method $Type:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Type> {
            let $self = self;
            $field.$get()
        }

        fn $set_mut(&mut self, arg: ::std::option::Option<$crate::$Type>) {
            let $self = self;
            $field.$set_mut(arg);
        }
    };
}
