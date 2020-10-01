/// Impl `Styler`.
#[macro_export]
macro_rules! impl_styler {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
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
        $crate::__impl_styler!([field] $(<$($G $(: $($B)+)?,)+>)? ($self: $Type) {
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
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) => $style:expr) => {
        $crate::__impl_styler!([method] $(<$($G $(: $($B)+)?,)+>)? ($self: $Type) {
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
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_styler {
    ([$mode:ident] $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Type:ty) {
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
        impl $(<$($G $(: $($B +)+)?,)+>)? $crate::Styler for $Type {
            __impl_styler!([$mode] $self {
                Foreground get_foreground foreground_mut $foreground
                Background get_background background_mut $background
                Weight     get_weight     weight_mut     $weight
                Slant      get_slant      slant_mut      $slant
                Blink      get_blink      blink_mut      $blink
                Invert     get_invert     invert_mut     $invert
                Strike     get_strike     strike_mut     $strike
                Underline  get_underline  underline_mut  $underline
                Overline   get_overline   overline_mut   $overline
                Border     get_border     border_mut     $border
            });
        }
    };
    ([$mode:ident] $self:ident {
        $($Attr:ident $get:ident $set_mut:ident $field:expr)*
    }) => {
        $(__impl_styler!([$mode] $Attr $get $set_mut $self $field);)*
    };
    ([field] $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $field
        }

        fn $set_mut(&mut self, arg: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $field = arg.into();
        }
    };
    ([method] $Attr:ident $get:ident $set_mut:ident $self:ident $field:expr) => {
        fn $get(&self) -> ::std::option::Option<$crate::$Attr> {
            let $self = self;
            $field.$get()
        }

        fn $set_mut(&mut self, arg: impl ::std::convert::Into<::std::option::Option<$crate::$Attr>>) {
            let $self = self;
            $field.$set_mut(arg);
        }
    };
}
