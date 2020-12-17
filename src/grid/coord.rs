use super::*;
use std::ops::{Add, Div, Mul, Sub};

macro_rules! ops {
    ($($Ret:ty { $($(#[$meta:meta])* $Trait:ident $fn:ident)* })*) => { $( $(
        $(#[$meta])*
        fn $fn<Rhs: Into<Self>>(self, rhs: Rhs) -> $Ret
        where
            T: $Trait<Output = T>,
        {
            let (x, y) = self.into();
            let (x2, y2) = rhs.into().into();
            <$Ret>::from((x.$fn(x2), y.$fn(y2)))
        }
    )* )* };
}

/// A 2D [`Coord`](crate::Coord) trait.
pub trait Coord<T: Clone>: Sized + Clone + From<T> + From<(T, T)> + Into<(T, T)> {
    /// Return type for checked operations.
    type Checked: Coord<Option<T>>;

    /// Returns the area (`x * y`).
    fn area(&self) -> <T as Mul>::Output
    where
        T: Mul,
    {
        let (x, y) = self.clone().into();
        x * y
    }

    /// Returns the area (`x * y`) or `None` if overflowing.
    fn checked_area(&self) -> Option<<T as CheckedMul>::Output>
    where
        T: CheckedMul,
    {
        let (x, y) = self.clone().into();
        x.checked_mul(y)
    }

    ops!(
        Self::Checked {
            /// Checked addition.
            CheckedAdd checked_add
            /// Checked substraction.
            CheckedSub checked_sub
            /// Checked multiplication.
            CheckedMul checked_mul
            /// Checked division.
            CheckedDiv checked_div
        }
        Self {
            /// Addition.
            Add add
            /// Substraction.
            Sub sub
            /// Multiplication.
            Mul mul
            /// Division.
            Div div
        }
    );
}
