/// Impl `StylerMut` idx.
#[macro_export]
macro_rules! impl_styler_mut_idx {
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
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self)
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
macro_rules! __impl_styler_mut_idx {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) => $styler:expr) => {
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
            ::std::ops::IndexMut::<$crate::Fg>::index_mut(&mut $styler, $crate::Fg),
            ::std::ops::IndexMut::<$crate::Bg>::index_mut(&mut $styler, $crate::Bg),
            ::std::ops::IndexMut::<$crate::Wgt>::index_mut(&mut $styler, $crate::Wgt),
            ::std::ops::IndexMut::<$crate::Slt>::index_mut(&mut $styler, $crate::Slt),
            ::std::ops::IndexMut::<$crate::Blk>::index_mut(&mut $styler, $crate::Blk),
            ::std::ops::IndexMut::<$crate::Inv>::index_mut(&mut $styler, $crate::Inv),
            ::std::ops::IndexMut::<$crate::Stk>::index_mut(&mut $styler, $crate::Stk),
            ::std::ops::IndexMut::<$crate::Udl>::index_mut(&mut $styler, $crate::Udl),
            ::std::ops::IndexMut::<$crate::Ovl>::index_mut(&mut $styler, $crate::Ovl),
            ::std::ops::IndexMut::<$crate::Brd>::index_mut(&mut $styler, $crate::Brd),
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
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Fg  Foreground $foreground_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Bg  Background $background_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Wgt Weight     $weight_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Slt Slant      $slant_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Blk Blink      $blink_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Inv Invert     $invert_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Stk Strike     $strike_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Udl Underline  $underline_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Ovl Overline   $overline_expr);
        $crate::__impl_styler_mut_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) Brd Border     $border_expr);
    };
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) $Idx:ident $Attr:ident $body:expr) => {
        doc!("Indexes `Option<" stringify!($Attr) ">` mutably.",
        impl $(<$($G $(: $($B +)+)?,)+>)? std::ops::IndexMut<$crate::$Idx> for $Self {
            doc!("Indexes `Option<" stringify!($Attr) ">` mutably.",
            fn index_mut(&mut self, _idx: $crate::$Idx) -> &mut ::std::option::Option<$crate::$Attr> {
                let $self = self;
                $body
            });
        });
    };
}
