use super::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! coords {
    ($($(#[$TMeta:meta])* $type:ident: $Type:ident $(#[$XMeta:meta])* $x:ident $(#[$YMeta:meta])* $y:ident)*) => { $(
        $(#[$TMeta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct $Type<T> {
            $(#[$XMeta])*
            pub $x: T,
            $(#[$YMeta])*
            pub $y: T,
        }

        /// ### Convertions
        impl<T> $Type<T> {
            /// Converts to another unit.
            pub fn to<U>(&self) -> $Type<U>
            where
                T: Into<U> + Clone,
            {
                let ($x, $y) = self.clone().into();
                $Type { $x: $x.into(), $y: $y.into() }
            }
        }

        impl<T: Clone> From<T> for $Type<T> {
            fn from(value: T) -> Self {
                Self {
                    $x: value.clone(),
                    $y: value,
                }
            }
        }

        impl<T> From<(T, T)> for $Type<T> {
            fn from(($x, $y): (T, T)) -> Self {
                Self { $x, $y }
            }
        }

        impl<T> From<$Type<T>> for (T, T) {
            fn from($type: $Type<T>) -> Self {
                ($type.$x, $type.$y)
            }
        }

        impl<T: Clamp> Clamp for $Type<T> {
            fn clamp_min(self, min: Self) -> Self {
                $Type {
                    $x: self.$x.clamp_min(min.$x),
                    $y: self.$y.clamp_min(min.$y),
                }
            }
            fn clamp_max(self, max: Self) -> Self {
                $Type {
                    $x: self.$x.clamp_max(max.$x),
                    $y: self.$y.clamp_max(max.$y),
                }
            }
            fn clamp(self, a: Self, b: Self) -> Self {
                $Type {
                    $x: self.$x.clamp(a.$x, b.$x),
                    $y: self.$y.clamp(a.$y, b.$y),
                }
            }
        }

        coords!([Op
            /// Addition.
            Add add
            /// Substraction.
            Sub sub
            /// Multiplication.
            Mul mul
            /// Division.
            Div div
        ] $Type $x $y);
        coords!([CheckedOp
            /// Checked addition.
            CheckedAdd checked_add
            /// Checked substraction.
            CheckedSub checked_sub
            /// Checked multiplication.
            CheckedMul checked_mul
            /// Checked division.
            CheckedDiv checked_div
        ] $Type $x $y);
        coords!([OpAssign
            AddAssign add_assign
            SubAssign sub_assign
            MulAssign mul_assign
            DivAssign div_assign
        ] $Type $x $y);
    )* };
    ([Op $($(#[$meta:meta])* $Op:ident $op:ident)*] $Type:ident $x:ident $y:ident) => {
        /// ### Operations
        impl<T> $Type<T> { $(
            $(#[$meta])*
            pub fn $op<U, Rhs: Into<$Type<U>>>(self, rhs: Rhs) -> $Type<<T as $Op<U>>::Output>
            where
                T: $Op<U>
            {
                let rhs = rhs.into();
                $Type {
                    $x: self.$x.$op(rhs.$x),
                    $y: self.$y.$op(rhs.$y),
                }
            }
        )* }

        $(impl<T: $Op, Rhs: Into<Self>> $Op<Rhs> for $Type<T> {
            type Output = $Type<<T as $Op>::Output>;
            fn $op(self, rhs: Rhs) -> Self::Output {
                Self::$op(self, rhs)
            }
        })*
    };
    ([OpAssign $($Op:ident $op:ident)*] $Type:ident $x:ident $y:ident) => {
        $(impl<T: $Op, Rhs: Into<Self>> $Op<Rhs> for $Type<T> {
            fn $op(&mut self, rhs: Rhs) {
                let rhs = rhs.into();
                self.$x = rhs.$x;
                self.$y = rhs.$y;
            }
        })*
    };
    ([CheckedOp $($(#[$meta:meta])* $Op:ident $op:ident)*] $Type:ident $x:ident $y:ident) => {
        /// ### Checked operations
        impl<T> $Type<T> { $(
            $(#[$meta])*
            pub fn $op<U, Rhs: Into<$Type<U>>>(self, rhs: Rhs) -> $Type<Option<<T as $Op<U>>::Output>>
            where
                T: $Op<U>
            {
                let rhs = rhs.into();
                $Type {
                    $x: self.$x.$op(rhs.$x),
                    $y: self.$y.$op(rhs.$y),
                }
            }
        )* }
    };
}

coords!(
    /// A `x`, `y` [`Point`](crate::Point).
    point: Point
        /// X axis `x` component.
        x
        /// Y axis `y` component.
        y
    /// A `width`, `height` [`Size`](crate::Size).
    size: Size
        /// X axis `width` component.
        width
        /// Y axis `height` component.
        height
);

/// ### Area
impl<T> Size<T> {
    /// Returns the area (`x * y`).
    pub fn area(&self) -> <T as Mul>::Output
    where
        T: Mul + Clone,
    {
        let (x, y) = self.clone().into();
        x * y
    }

    /// Returns the area (`x * y`) or `None` if overflowing.
    pub fn checked_area(&self) -> Option<<T as CheckedMul>::Output>
    where
        T: CheckedMul + Clone,
    {
        let (x, y) = self.clone().into();
        x.checked_mul(y)
    }
}
