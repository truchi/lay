use crate::*;
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

macro_rules! coords {
    ($(
        $(#[$TMeta:meta])*
        $type:ident: $Type:ident ($as:ident $As:ident $a:ident: $A:ident $b:ident: $B:ident)
        $(#[$XMeta:meta])* $x:ident: $X:ident
        $(#[$YMeta:meta])* $y:ident: $Y:ident
    )*) => { $(
        $(#[$TMeta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct $Type<$X, $Y = $X> {
            $(#[$XMeta])* pub $x: $X,
            $(#[$YMeta])* pub $y: $Y,
        }

        /// ### Convertions
        impl<$X, $Y> $Type<$X, $Y> {
            /// Converts to other units.
            pub fn to<T, U>(self) -> $Type<T, U>
            where
                $X: Into<T>,
                $Y: Into<U>,
            {
                $Type { $x: self.$x.into(), $y: self.$y.into() }
            }

            doc!("Converts to [`" stringify!($As) "`](crate::" stringify!($As) ").",
            pub fn $as<$A, $B>(self) -> $As<$A, $B>
            where
                $X: Into<$A>,
                $Y: Into<$B>,
            {
                $As { $a: self.$x.into(), $b: self.$y.into() }
            });
        }

        impl<T: Clone> From<T> for $Type<T> {
            fn from(value: T) -> Self { Self { $x: value.clone(), $y: value } }
        }

        impl<$X, $Y> From<($X, $Y)> for $Type<$X, $Y> {
            fn from(($x, $y): ($X, $Y)) -> Self { Self { $x, $y } }
        }

        impl<$X, $Y> From<$Type<$X, $Y>> for ($X, $Y) {
            fn from($type: $Type<$X, $Y>) -> Self { ($type.$x, $type.$y) }
        }

        impl<$X: PartialOrd, $Y: PartialOrd> PartialOrd for $Type<$X, $Y> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match (self.$x.partial_cmp(&other.$x), self.$y.partial_cmp(&other.$y)) {
                    (Some(self_ord), Some(other_ord)) if self_ord == other_ord => Some(self_ord),
                    _ => None,
                }
            }
        }

        impl<$X: Clamp, $Y: Clamp> Clamp<$Type<$X, $Y>> for $Type<$X, $Y> {
            type Output = $Type<<$X as Clamp>::Output, <$Y as Clamp>::Output>;

            fn clamp_min(self, min: $Type<$X, $Y>) -> Self::Output {
                $Type {
                    $x: self.$x.clamp_min(min.$x),
                    $y: self.$y.clamp_min(min.$y),
                }
            }

            fn clamp_max(self, max: $Type<$X, $Y>) -> Self::Output {
                $Type {
                    $x: self.$x.clamp_max(max.$x),
                    $y: self.$y.clamp_max(max.$y),
                }
            }

            fn clamp(self, min: $Type<$X, $Y>, max: $Type<$X, $Y>) -> Self::Output {
                $Type {
                    $x: self.$x.clamp(min.$x, max.$x),
                    $y: self.$y.clamp(min.$y, max.$y),
                }
            }
        }

        coords!(const $Type $x $X $y $Y Zero ZERO One ONE Min MIN Max MAX);
        coords!([Op
            /// Addition.
            Add add
            /// Substraction.
            Sub sub
            /// Multiplication.
            Mul mul
            /// Division.
            Div div
        ] $Type $x $X $y $Y);
        coords!([OpAssign
            AddAssign add_assign
            SubAssign sub_assign
            MulAssign mul_assign
            DivAssign div_assign
        ] $Type $x $X $y $Y);
        coords!([CheckedOp
            /// Checked addition.
            CheckedAdd checked_add
            /// Checked substraction.
            CheckedSub checked_sub
            /// Checked multiplication.
            CheckedMul checked_mul
            /// Checked division.
            CheckedDiv checked_div
        ] $Type $x $X $y $Y);
        coords!([SaturatingOp
            /// Saturating addition.
            SaturatingAdd saturating_add
            /// Saturating substraction.
            SaturatingSub saturating_sub
            /// Saturating multiplication.
            SaturatingMul saturating_mul
        ] $Type $x $X $y $Y);
    )* };
    (const $Type:ident $x:ident $X:ident $y:ident $Y:ident $($Trait:ident $const:ident)*) => { $(
        impl<$X: $Trait, $Y: $Trait> $Trait for $Type<$X, $Y> {
            const $const: Self = Self { $x: $X::$const, $y: $Y::$const };
        }
    )* };
    ([Op $($(#[$meta:meta])* $Op:ident $op:ident)*] $Type:ident $x:ident $X:ident $y:ident $Y:ident) => {
        $(impl<T, U, $X: $Op<T>, $Y: $Op<U>> $Op<$Type<T, U>> for $Type<$X, $Y> {
            type Output = $Type<<$X as $Op<T>>::Output, <$Y as $Op<U>>::Output>;

            fn $op(self, rhs: $Type<T, U>) -> Self::Output {
                $Type { $x: self.$x.$op(rhs.$x), $y: self.$y.$op(rhs.$y) }
            }
        })*
    };
    ([OpAssign $($Op:ident $op:ident)*] $Type:ident $x:ident $X:ident $y:ident $Y:ident) => {
        $(impl<$X: $Op, $Y: $Op> $Op<$Type<$X, $Y>> for $Type<$X, $Y> {
            fn $op(&mut self, rhs: $Type<$X, $Y>) {
                self.$x = rhs.$x;
                self.$y = rhs.$y;
            }
        })*
    };
    ([CheckedOp $($(#[$meta:meta])* $Op:ident $op:ident)*] $Type:ident $x:ident $X:ident $y:ident $Y:ident) => {
        /// ### Checked operations
        impl<$X, $Y> $Type<$X, $Y> { $(
            $(#[$meta])*
            pub fn $op<T, U>(self, rhs: $Type<T, U>) -> $Type<Option<<$X as $Op<T>>::Output>, Option<<$Y as $Op<U>>::Output>>
            where
                $X: $Op<T>,
                $Y: $Op<U>,
            { $Type { $x: self.$x.$op(rhs.$x), $y: self.$y.$op(rhs.$y) } }
        )* }
    };
    ([SaturatingOp $($(#[$meta:meta])* $Op:ident $op:ident)*] $Type:ident $x:ident $X:ident $y:ident $Y:ident) => {
        /// ### Saturating operations
        impl<$X, $Y> $Type<$X, $Y> { $(
            $(#[$meta])*
            pub fn $op<T, U>(self, rhs: $Type<T, U>) -> $Type<<$X as $Op<T>>::Output, <$Y as $Op<U>>::Output>
            where
                $X: $Op<T>,
                $Y: $Op<U>,
            { $Type { $x: self.$x.$op(rhs.$x), $y: self.$y.$op(rhs.$y) } }
        )* }
    };
}

coords!(
    /// A `x`, `y` [`Point`](crate::Point).
    point: Point (as_size Size width: W height: H)
        /// X axis `x` component.
        x: X
        /// Y axis `y` component.
        y: Y
    /// A `width`, `height` [`Size`](crate::Size).
    size: Size (as_point Point x: X y: Y)
        /// X axis `width` component.
        width: W
        /// Y axis `height` component.
        height: H
);

/// ### Area
impl<W, H> Size<W, H> {
    /// Returns the area (`width * height`).
    pub fn area(self) -> <W as Mul<H>>::Output
    where
        W: Mul<H>,
    {
        let (width, height) = self.into();
        width * height
    }

    /// Returns the area (`width * height`) or `None` if overflowing.
    pub fn checked_area(self) -> Option<<W as CheckedMul<H>>::Output>
    where
        W: CheckedMul<H>,
    {
        let (width, height) = self.into();
        width.checked_mul(height)
    }
}
