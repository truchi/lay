//! # Geometry utilities

mod gen;
pub use gen::*;

use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

macro_rules! s {
    ($($t:tt)*) => { stringify!($($t)*) };
}

macro_rules! doc {
    ($($doc:expr)*, $item:item) => { doc!{impl concat![" ", $($doc,)*], $item} };
    (impl $doc:expr, $item:item) => { #[doc = $doc] $item };
}

macro_rules! wrapper {
    ($(#[$doc:meta] $type:ident: $Type:ident)*) => {
        $(
            #[$doc]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
            pub struct $Type(pub usize);

            /// ### Consts
            impl $Type {
                doc!("A [`" s!($Type) "`](crate::" s!($Type) ") of `0`.",
                pub const ZERO: $Type = $Type(0););
            }

            /// `Deref`s to `usize`.
            impl Deref for $Type {
                type Target = usize;

                /// `Deref`s to `usize`.
                fn deref(&self) -> &usize {
                    &self.0
                }
            }

            /// `DerefMut`s to `usize`.
            impl DerefMut for $Type {
                /// `DerefMut`s to `usize`.
                fn deref_mut(&mut self) -> &mut usize {
                    &mut self.0
                }
            }

            doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `" s!($type) "`.",
            impl From<$Type> for usize {
                doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `" s!($type) "`.",
                fn from($type: $Type) -> Self {
                    $type.0
                });
            });

            doc!("Returns a new `" s!($type) "` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<usize> for $Type {
                doc!("Returns a new `" s!($type) "` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from($type: usize) -> Self {
                    Self($type)
                });
            });
        )*
    };
    ($(#[$doc:meta] $type:ident: $Type:ident { $a:ident: $A:ident, $b:ident: $B:ident })*) => {
        $(
            #[$doc]
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $Type { pub $a: $A, pub $b: $B }

            /// ### Consts
            impl $Type {
                doc!("A [`" s!($Type) "`](crate::" s!($Type) ") of `(0, 0)`.",
                pub const ZERO: $Type = $Type { $a: $A::ZERO, $b: $B::ZERO };);
            }

            /// ### Constructors
            impl $Type {
                pub fn new($a: $A, $b: $B) -> Self {
                    Self { $a, $b }
                }
            }

            /// Compares both fields simultaneously.
            impl PartialOrd for $Type {
                /// Compares both fields simultaneously.
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    match (self.$a.cmp(&other.$a), self.$b.cmp(&other.$b)) {
                        (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
                        (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
                        (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
                        _ => None
                    }
                }
            }

            doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `(" s!($a) ", " s!($b) ")`.",
            impl From<$Type> for (usize, usize) {
                doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `(" s!($a) ", " s!($b) ")`.",
                fn from($type: $Type) -> Self {
                    ($type.$a.0, $type.$b.0)
                });
            });

            doc!("Returns a new `(value, value)` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<usize> for $Type {
                doc!("Returns a new `(value, value)` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from(value: usize) -> Self {
                    Self {
                        $a: $A(value),
                        $b: $B(value),
                    }
                });
            });

            doc!("Returns a new `(" s!($a) ", 0)` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<$A> for $Type {
                doc!("Returns a new `(" s!($a) ", 0)` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from($a: $A) -> Self {
                    Self { $a, $b: $B::ZERO }
                });
            });

            doc!("Returns a new `(0, " s!($b) ")` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<$B> for $Type {
                doc!("Returns a new `(0, " s!($b) ")` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from($b: $B) -> Self {
                    Self { $a: $A::ZERO, $b }
                });
            });

            doc!("Returns a new `(" s!($a) ", " s!($b) ")` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<($A, $B)> for $Type {
                doc!("Returns a new `(" s!($a) ", " s!($b) ")` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from(($a, $b): ($A, $B)) -> Self {
                    Self { $a, $b }
                });
            });

            doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s [`" s!($A) "`](crate::" s!($A) ").",
            impl From<$Type> for $A {
                doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s [`" s!($A) "`](crate::" s!($A) ").",
                fn from($type: $Type) -> Self {
                    $type.$a
                });
            });

            doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s [`" s!($B) "`](crate::" s!($B) ").",
            impl From<$Type> for $B {
                doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s [`" s!($B) "`](crate::" s!($B) ").",
                fn from($type: $Type) -> Self {
                    $type.$b
                });
            });

            doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `(" s!($A) ", " s!($B) ")`.",
            impl From<$Type> for ($A, $B) {
                doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `(" s!($A) ", " s!($B) ")`.",
                fn from($type: $Type) -> Self {
                    ($type.$a, $type.$b)
                });
            });
        )*

    };
}

wrapper!(
    /// An [`X`](crate::X) coordinate.
    x: X
    /// An [`Y`](crate::Y) coordinate.
    y: Y
    /// A [`Width`](crate::Width) distance.
    width: Width
    /// An [`Height`](crate::Height) distance.
    height: Height
);

wrapper!(
    /// A `(x, y)` [`Point`](crate::Point).
    point: Point { x: X, y: Y }
    /// A `(width, height)` [`Size`](crate::Size).
    size: Size { width: Width, height: Height }
);
