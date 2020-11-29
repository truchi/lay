//! # Geometry utilities

use std::ops::{
    Add,
    AddAssign,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    BitXor,
    BitXorAssign,
    Deref,
    DerefMut,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Not,
    Rem,
    RemAssign,
    Shl,
    ShlAssign,
    Shr,
    ShrAssign,
    Sub,
    SubAssign,
};

macro_rules! ops {
    ($Type:ty $(: $a:ident $b:ident)?) => {
        ops!($Type $(: $a $b)? =>
            Add(add) AddAssign(add_assign)
            Sub(sub) SubAssign(sub_assign)
            Mul(mul) MulAssign(mul_assign)
            Div(div) DivAssign(div_assign)
            Rem(rem) RemAssign(rem_assign)
            Shl(shl) ShlAssign(shl_assign)
            Shr(shr) ShrAssign(shr_assign)
            BitAnd(bitand) BitAndAssign(bitand_assign)
            BitOr(bitor)   BitOrAssign(bitor_assign)
            BitXor(bitxor) BitXorAssign(bitxor_assign)
        );
        ops!([OpUnary] $Type $(: $a $b)? => Not(not));
    };
    ($Type:ty $(: $a:ident $b:ident)? => $($Op:ident($op:ident) $OpAssign:ident($op_assign:ident))*) => {
        ops!([Op]       $Type $(: $a $b)? => $($Op($op))*);
        ops!([OpAssign] $Type $(: $a $b)? => $($OpAssign($op_assign))*);
    };

    ([OpUnary] $Type:ty => $($Op:ident($op:ident))*) => {
        $(impl $Op for $Type {
            type Output = Self;

            fn $op(self) -> Self {
                Self(self.0.$op())
            }
        })*
    };
    ([OpUnary] $Type:ty: $a:ident $b:ident => $($Op:ident($op:ident))*) => {
        $(impl $Op for $Type {
            type Output = Self;

            fn $op(self) -> Self {
                Self {
                    $a: self.$a.$op(),
                    $b: self.$b.$op(),
                }
            }
        })*
    };

    ([Op] $Type:ty => $($Op:ident($op:ident))*) => {
        $(impl $Op for $Type {
            type Output = Self;

            fn $op(self, rhs: Self) -> Self {
                Self(self.0.$op(rhs.0))
            }
        })*
    };
    ([Op] $Type:ty: $a:ident $b:ident => $($Op:ident($op:ident))*) => {
        $(impl $Op for $Type {
            type Output = Self;

            fn $op(self, rhs: Self) -> Self {
                Self {
                    $a: self.$a.$op(rhs.$a),
                    $b: self.$b.$op(rhs.$b),
                }
            }
        })*
    };

    ([OpAssign] $Type:ty => $($OpAssign:ident($op_assign:ident))*) => {
        $(impl $OpAssign for $Type {
            fn $op_assign(&mut self, rhs: Self) {
                self.0.$op_assign(rhs.0)
            }
        })*
    };
    ([OpAssign] $Type:ty: $a:ident $b:ident => $($OpAssign:ident($op_assign:ident))*) => {
        $(impl $OpAssign for $Type {
            fn $op_assign(&mut self, rhs: Self) {
                self.$a.$op_assign(rhs.$a);
                self.$b.$op_assign(rhs.$b);
            }
        })*
    };
}

macro_rules! convert {
    ($self:ident: $Type:ty) => {
        impl From<usize> for $Type {
            fn from(usize: usize) -> Self {
                Self(usize)
            }
        }

        impl From<$Type> for usize {
            fn from($self: $Type) -> Self {
                $self.0
            }
        }
    };
    ($self:ident: $Type:ty, $a:ident: $A:ident, $b:ident: $B:ident) => {
        impl From<usize> for $Type {
            fn from(usize: usize) -> Self {
                Self {
                    $a: $A(usize),
                    $b: $B(usize),
                }
            }
        }

        impl From<(usize, usize)> for $Type {
            fn from(($a, $b): (usize, usize)) -> Self {
                Self {
                    $a: $A($a),
                    $b: $B($b),
                }
            }
        }

        impl From<$A> for $Type {
            fn from($a: $A) -> Self {
                Self { $a, $b: $B(0) }
            }
        }

        impl From<$B> for $Type {
            fn from($b: $B) -> Self {
                Self { $a: $A(0), $b }
            }
        }

        impl From<($A, $B)> for $Type {
            fn from(($a, $b): ($A, $B)) -> Self {
                Self { $a, $b }
            }
        }

        impl From<$Type> for (usize, usize) {
            fn from($self: $Type) -> Self {
                ($self.$a.0, $self.$b.0)
            }
        }

        impl From<$Type> for $A {
            fn from($self: $Type) -> Self {
                $self.$a
            }
        }

        impl From<$Type> for $B {
            fn from($self: $Type) -> Self {
                $self.$b
            }
        }

        impl From<$Type> for ($A, $B) {
            fn from($self: $Type) -> Self {
                ($self.$a, $self.$b)
            }
        }
    };
}

macro_rules! deref {
    ($Type:ident) => {
        impl Deref for $Type {
            type Target = usize;

            fn deref(&self) -> &usize {
                &self.0
            }
        }

        impl DerefMut for $Type {
            fn deref_mut(&mut self) -> &mut usize {
                &mut self.0
            }
        }
    };
}

macro_rules! wrapper {
    ($(#[$doc:meta] $self:ident: $Type:ident)*) => {
        $(
            #[$doc]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
            pub struct $Type(pub usize);

            ops!($Type);
            convert!($self: $Type);
            deref!($Type);
        )*
    };
    ($(#[$doc:meta] $self:ident: $Type:ident { $a:ident: $A:ident, $b:ident: $B:ident })*) => {
        $(
            #[$doc]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
            pub struct $Type { pub $a: $A, pub $b: $B }

            /// ### Constructors
            impl $Type {
                pub fn new($a: $A, $b: $B) -> Self {
                    Self { $a, $b }
                }
            }

            /// ### Methods
            impl $Type {
                /// Returns true if both fields are lower than those in `other`.
                pub fn lt(&self, other: &Self) -> bool {
                    self.$a < other.$a && self.$b < other.$b
                }
            }

            ops!($Type: $a $b);
            convert!($self: $Type, $a: $A, $b: $B);
        )*

    };
}

wrapper!(
    /// An [`X`](crate::geometry::X) coordinate.
    x: X
    /// An [`Y`](crate::geometry::Y) coordinate.
    y: Y
    /// A [`Width`](crate::geometry::Width) distance.
    width: Width
    /// An [`Height`](crate::geometry::Height) distance.
    height: Height
);

wrapper!(
    /// A `(x, y)` [`Position`](crate::geometry::Position).
    position: Position { x: X, y: Y }
    /// A `(width, height)` [`Size`](crate::geometry::Size).
    size: Size { width: Width, height: Height }
);

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        assert_eq!(
            Position::new(X(1), Y(2)),
            Position { x: X(1), y: Y(2) },
            "Position::new"
        );
        assert_eq!(
            Size::new(Width(3), Height(4)),
            Size {
                width:  Width(3),
                height: Height(4),
            },
            "Size::new"
        );
    }

    #[test]
    fn lt() {
        macro_rules! lt {
            ($($T:ident)*) => { $(
                assert_eq!($T::from((1, 1)).lt(&$T::from((2, 2))), true);
                assert_eq!($T::from((1, 2)).lt(&$T::from((2, 2))), false);
                assert_eq!($T::from((2, 1)).lt(&$T::from((2, 2))), false);
                assert_eq!($T::from((2, 2)).lt(&$T::from((2, 2))), false);
            )* };
        }

        lt!(Position Size);
    }

    #[test]
    fn convert() {
        macro_rules! convert {
            ($($T:ident)*) => { $(
                assert_eq!(usize::from($T(0)), 0,
                    concat!("From<", stringify!($T) ,"> for usize"));
                assert_eq!($T::from(0), $T(0),
                    concat!("From<usize> for ", stringify!($T)));
            )* };
            ($($a:ident $A:ident $b:ident $B:ident: $T:ident)*) => { $(
                assert_eq!($T::from(1), $T { $a: $A(1), $b: $B(1) },
                    concat!("From<usize> for ", stringify!($T)));
                assert_eq!($T::from((2, 3)), $T { $a: $A(2), $b: $B(3) },
                    concat!("From <(usize, usize)> for ", stringify!($T)));
                assert_eq!($T::from($A(4)), $T { $a: $A(4), $b: $B(0) },
                    concat!("From<", stringify!($A), "> for ", stringify!($T)));
                assert_eq!($T::from($B(5)), $T { $a: $A(0), $b: $B(5) },
                    concat!("From<", stringify!($B), "> for ", stringify!($T)));
                assert_eq!($T::from(($A(6), $B(7))), $T { $a: $A(6), $b: $B(7) },
                    concat!("From<(", stringify!($A), ", ", stringify!($B), ")> for ", stringify!($T)));

                assert_eq!(<(usize, usize)>::from($T { $a: $A(8), $b: $B(9) }), (8, 9),
                    concat!("From<", stringify!($T), "> for (usize, usize)"));
                assert_eq!($A::from($T { $a: $A(10), $b: $B(11) }), $A(10),
                    concat!("From<", stringify!($T), "> for A"));
                assert_eq!($B::from($T { $a: $A(12), $b: $B(13) }), $B(13),
                    concat!("From<", stringify!($T), "> for B"));
                assert_eq!(<($A, $B)>::from($T { $a: $A(14), $b: $B(15) }), ($A(14), $B(15)),
                    concat!("From<", stringify!($T), "> for (A, B)"));
            )* };
        }

        convert!(X Y Width Height);
        convert!(x X y Y: Position width Width height Height: Size);
    }

    #[test]
    fn ops() {
        macro_rules! ops {
            ($($T:ident)*) => { $(
                assert_eq!($T(4) + $T(2), $T(4 + 2), concat!("Add ", stringify!($T)));
                assert_eq!($T(4) - $T(2), $T(4 - 2), concat!("Sub ", stringify!($T)));
                assert_eq!($T(4) * $T(2), $T(4 * 2), concat!("Mul ", stringify!($T)));
                assert_eq!($T(4) / $T(2), $T(4 / 2), concat!("Div ", stringify!($T)));
                assert_eq!($T(4) % $T(2), $T(4 % 2), concat!("Rem ", stringify!($T)));
                assert_eq!($T(4) << $T(2), $T(4 << 2), concat!("Shl ", stringify!($T)));
                assert_eq!($T(4) >> $T(2), $T(4 >> 2), concat!("Shr ", stringify!($T)));
                assert_eq!($T(4) & $T(2), $T(4 & 2), concat!("BitAnd ", stringify!($T)));
                assert_eq!($T(4) | $T(2), $T(4 | 2), concat!("BitOr ", stringify!($T)));
                assert_eq!($T(4) ^ $T(2), $T(4 ^ 2), concat!("BitXor ", stringify!($T)));
                assert_eq!(!$T(4), $T(!4), concat!("Not ", stringify!($T)));
            )* };
            ($($a:ident $A:ident $b:ident $B:ident: $T:ident)*) => { $(
                assert_eq!($T { $a: $A(4), $b: $B(4) } + $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 + 2), $b: $B(4 + 2) }, concat!("Add ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } - $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 - 2), $b: $B(4 - 2) }, concat!("Sub ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } * $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 * 2), $b: $B(4 * 2) }, concat!("Mul ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } / $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 / 2), $b: $B(4 / 2) }, concat!("Div ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } % $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 % 2), $b: $B(4 % 2) }, concat!("Rem ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } << $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 << 2), $b: $B(4 << 2) }, concat!("Shl ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } >> $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 >> 2), $b: $B(4 >> 2) }, concat!("Shr ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } & $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 & 2), $b: $B(4 & 2), }, concat!("BitAnd ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } | $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 | 2), $b: $B(4 | 2), }, concat!("BitOr ", stringify!($T)));
                assert_eq!($T { $a: $A(4), $b: $B(4) } ^ $T { $a: $A(2), $b: $B(2) }, $T { $a: $A(4 ^ 2), $b: $B(4 ^ 2), }, concat!("BitXor ", stringify!($T)));
                assert_eq!(!$T { $a: $A(4), $b: $B(4) }, $T{ $a: $A(!4), $b: $B(!4) }, concat!("Not ", stringify!($T)));
            )* };
        }

        ops!(X Y Width Height);
        ops!(x X y Y: Position width Width height Height: Size);
    }

    #[test]
    fn deref() {
        assert_eq!((&X(1)).deref(), &1);
        assert_eq!((&Y(2)).deref(), &2);
        assert_eq!((&Width(3)).deref(), &3);
        assert_eq!((&Height(4)).deref(), &4);
    }
}
