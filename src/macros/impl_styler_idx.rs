/// Impl `Styler`/`StylerMut` idx.
#[macro_export]
macro_rules! impl_styler_idx {
    ($(
        // "" or "mut"
        $($mut:ident)?
        // Generics and corresponding bounds
        $(<$($G:ident $(: $($B:path)+)?,)+>)?
        // self argument and type
        ($self:tt: $Self:path)
        // One of:
            // List of getters to attributes
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
            // A getter to a Styler field
            $(=> $styler:expr)?
    )+) => {
        $($crate::priv_impl_styler_idx!($($mut)? $(<$($G $(: $($B)+)?,)+>)? ($self: $Self)
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
        );)+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! priv_impl_styler_idx {
    ($(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) => $styler:expr) => {
        $crate::priv_impl_styler_idx!($(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
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
    (mut $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:tt: $Self:path) => $styler:expr) => {
        $crate::priv_impl_styler_idx!(mut $(<$($G $(: $($B)+)?,)+>)? ($self: $Self) {
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
        $crate::priv_impl_styler_idx!(($self: $Self)
            $(<$($G $(: $($B)+)?,)+>)? Fg  Foreground $foreground_expr,
            $(<$($G $(: $($B)+)?,)+>)? Bg  Background $background_expr,
            $(<$($G $(: $($B)+)?,)+>)? Wgt Weight     $weight_expr,
            $(<$($G $(: $($B)+)?,)+>)? Slt Slant      $slant_expr,
            $(<$($G $(: $($B)+)?,)+>)? Blk Blink      $blink_expr,
            $(<$($G $(: $($B)+)?,)+>)? Inv Invert     $invert_expr,
            $(<$($G $(: $($B)+)?,)+>)? Stk Strike     $strike_expr,
            $(<$($G $(: $($B)+)?,)+>)? Udl Underline  $underline_expr,
            $(<$($G $(: $($B)+)?,)+>)? Ovl Overline   $overline_expr,
            $(<$($G $(: $($B)+)?,)+>)? Brd Border     $border_expr,
        );
    };
    (mut $(<$($G:ident $(: $($B:path)+)?,)+>)? ($self:ident: $Self:path) {
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
        $crate::priv_impl_styler_idx!(mut ($self: $Self)
            $(<$($G $(: $($B)+)?,)+>)? Fg  Foreground $foreground_expr,
            $(<$($G $(: $($B)+)?,)+>)? Bg  Background $background_expr,
            $(<$($G $(: $($B)+)?,)+>)? Wgt Weight     $weight_expr,
            $(<$($G $(: $($B)+)?,)+>)? Slt Slant      $slant_expr,
            $(<$($G $(: $($B)+)?,)+>)? Blk Blink      $blink_expr,
            $(<$($G $(: $($B)+)?,)+>)? Inv Invert     $invert_expr,
            $(<$($G $(: $($B)+)?,)+>)? Stk Strike     $strike_expr,
            $(<$($G $(: $($B)+)?,)+>)? Udl Underline  $underline_expr,
            $(<$($G $(: $($B)+)?,)+>)? Ovl Overline   $overline_expr,
            $(<$($G $(: $($B)+)?,)+>)? Brd Border     $border_expr,
        );
    };

    (($self:tt: $Self:path) $($(<$($G:ident $(: $($B:path)+)?,)+>)? $Idx:ident $Attr:ident $body:expr,)*) => {
        $(
            doc!("Indexes `Option<" stringify!($Attr) ">`.",
            impl $(<$($G $(: $($B +)+)?,)+>)? std::ops::Index<$crate::$Idx> for $Self {
                $crate::doc!("`Option<" stringify!($Attr) ">`.",
                type Output = ::std::option::Option<$crate::$Attr>;);

                doc!("Indexes `Option<" stringify!($Attr) ">`.",
                fn index(&self, _: $crate::$Idx) -> &::std::option::Option<$crate::$Attr> {
                    let $self = self;
                    $body
                });
            });
        )*
    };
    (mut ($self:tt: $Self:path) $($(<$($G:ident $(: $($B:path)+)?,)+>)? $Idx:ident $Attr:ident $body:expr,)*) => {
        $(
            doc!("Indexes `Option<" stringify!($Attr) ">` mutably.",
            impl $(<$($G $(: $($B +)+)?,)+>)? std::ops::IndexMut<$crate::$Idx> for $Self {
                doc!("Indexes `Option<" stringify!($Attr) ">` mutably.",
                fn index_mut(&mut self, _: $crate::$Idx) -> &mut ::std::option::Option<$crate::$Attr> {
                    let $self = self;
                    $body
                });
            });
        )*
    };
}
