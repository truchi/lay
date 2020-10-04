/// Impl `Styler` idx.
#[macro_export]
macro_rules! impl_styler_idx {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path)
        $({
            $foreground_expr:expr,
            $background_expr:expr,
            $weight_expr:expr,
            $slant_expr:expr,
            $blink_expr:expr,
            $invert_expr:expr,
            $strike_expr:expr,
            $underline_expr:expr,
            $overline_expr:expr,
            $border_expr:expr,
        })?
        $(=> $styler:expr)?
    ) => {
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self)
            $(=> $styler)?
            $({
                $foreground_expr,
                $background_expr,
                $weight_expr,
                $slant_expr,
                $blink_expr,
                $invert_expr,
                $strike_expr,
                $underline_expr,
                $overline_expr,
                $border_expr,
            })?
        );
    };
}

#[macro_export]
macro_rules! __impl_styler_idx {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) => $styler:expr) => {
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            ::std::ops::Index::<$crate::Fg>::index(&$styler, $crate::Fg),
            ::std::ops::Index::<$crate::Bg>::index(&$styler, $crate::Bg),
            ::std::ops::Index::<$crate::Wgt>::index(&$styler, $crate::Wgt),
            ::std::ops::Index::<$crate::Slt>::index(&$styler, $crate::Slt),
            ::std::ops::Index::<$crate::Blk>::index(&$styler, $crate::Blk),
            ::std::ops::Index::<$crate::Inv>::index(&$styler, $crate::Inv),
            ::std::ops::Index::<$crate::Stk>::index(&$styler, $crate::Stk),
            ::std::ops::Index::<$crate::Udl>::index(&$styler, $crate::Udl),
            ::std::ops::Index::<$crate::Ovl>::index(&$styler, $crate::Ovl),
            ::std::ops::Index::<$crate::Brd>::index(&$styler, $crate::Brd),
        });
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path) {
        $foreground_expr:expr,
        $background_expr:expr,
        $weight_expr:expr,
        $slant_expr:expr,
        $blink_expr:expr,
        $invert_expr:expr,
        $strike_expr:expr,
        $underline_expr:expr,
        $overline_expr:expr,
        $border_expr:expr,
    }) => {
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Fg  Foreground $foreground_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Bg  Background $background_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Wgt Weight     $weight_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Slt Slant      $slant_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Blk Blink      $blink_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Inv Invert     $invert_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Stk Strike     $strike_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Udl Underline  $underline_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Ovl Overline   $overline_expr);
        $crate::__impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Brd Border     $border_expr);
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) $Idx:ident $Attr:ident $body:expr) => {
        doc!("Indexes `Option<" stringify!($Attr) ">`.",
        impl $(<$($G $(: $($B +)+)?,)+>)? std::ops::Index<$crate::$Idx> for $Self {
            $crate::doc!("`Option<" stringify!($Attr) ">`.",
            type Output = ::std::option::Option<$crate::$Attr>;);

            doc!("Indexes `Option<" stringify!($Attr) ">`.",
            fn index(&self, _idx: $crate::$Idx) -> &::std::option::Option<$crate::$Attr> {
                let $self = self;
                $body
            });
        });
    };
}
