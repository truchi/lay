macro_rules! impl_styler {
    ($([$Option:ident])? $self:ident {
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
        impl_styler!(impl $([$Option])? field $self {
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
    ($([$Option:ident])? $self:ident => $style:expr) => {
        impl_styler!(impl $([$Option])? method $self {
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
    (impl $([$Option:ident])? $mode:ident $self:ident {
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
        impl_styler!(impl [$($Option)? $mode] Foreground get_foreground foreground_mut $self $foreground);
        impl_styler!(impl [$($Option)? $mode] Background get_background background_mut $self $background);
        impl_styler!(impl [$($Option)? $mode] Weighted get_weighted weighted_mut $self $weight);
        impl_styler!(impl [$($Option)? $mode] Slanted get_slanted slanted_mut $self $slant);
        impl_styler!(impl [$($Option)? $mode] Blinking get_blinking blinking_mut $self $blink);
        impl_styler!(impl [$($Option)? $mode] Inverted get_inverted inverted_mut $self $invert);
        impl_styler!(impl [$($Option)? $mode] Striked get_striked striked_mut $self $strike);
        impl_styler!(impl [$($Option)? $mode] Underlined get_underlined underlined_mut $self $underline);
        impl_styler!(impl [$($Option)? $mode] Overlined get_overlined overlined_mut $self $overline);
        impl_styler!(impl [$($Option)? $mode] Bordered get_bordered bordered_mut $self $border);
    };
    (impl [Option field] $Type:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        impl_styler!(impl ::std::option::Option<$crate::$Type>, $get $set_mut
            $self arg { $field }, { $field = arg; });
    };
    (impl [Option method] $Type:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        impl_styler!(impl ::std::option::Option<$crate::$Type>, $get $set_mut
            $self arg { $field.$get() }, { $field.$set_mut(arg); });
    };
    (impl [field] $Type:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        impl_styler!(impl $crate::$Type, $get $set_mut
            $self arg { $field }, { $field = arg; });
    };
    (impl [method] $Type:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        impl_styler!(impl $crate::$Type, $get $set_mut
            $self arg { $field.$get() }, { $field.$set_mut(arg); });
    };
    (impl $Type:ty, $get:ident $set_mut:ident $self:ident $arg:ident $get_body:expr, $set_body:expr) => {
        fn $get(&self) -> $Type {
            let $self = self;
            $get_body
        }

        fn $set_mut(&mut self, $arg: $Type) {
            let $self = self;
            $set_body
        }

    };
}
