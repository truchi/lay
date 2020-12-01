//! # Geometry utilities

mod gen;
mod rect;

pub use rect::*;

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

macro_rules! types {
    // ======== //
    // 1D types //
    // ======== //
    ($(#[$doc:meta] $type:ident: $Type:ident)*) => {
        $(
            #[$doc]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
            pub struct $Type(pub usize);

            // ============== //
            // Implementation //
            // ============== //

            /// ### Consts
            impl $Type {
                doc!("A [`" s!($Type) "`](crate::" s!($Type) ") of `0`.",
                pub const ZERO: $Type = $Type(0););
            }

            // ====== //
            // Traits //
            // ====== //

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

            // =========== //
            // Conversions //
            // =========== //

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
    // ======== //
    // 2D types //
    // ======== //
    ($(#[$doc:meta] $type:ident: $Type:ident { $a:ident: $A:ident, $b:ident: $B:ident })*) => {
        $(
            #[$doc]
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $Type { pub $a: $A, pub $b: $B }

            // ============== //
            // Implementation //
            // ============== //

            /// ### Consts
            impl $Type {
                doc!("A [`" s!($Type) "`](crate::" s!($Type) ") of `(0, 0)`.",
                pub const ZERO: $Type = $Type { $a: $A::ZERO, $b: $B::ZERO };);
            }

            /// ### Constructors
            impl $Type {
                doc!("Returns a new [`" s!($Type) "`](crate::" s!($Type) ").",
                pub fn new($a: $A, $b: $B) -> Self {
                    Self { $a, $b }
                });
            }

            // ====== //
            // Traits //
            // ====== //

            /// Compares both fields simultaneously.
            impl PartialOrd for $Type {
                /// Compares both fields simultaneously.
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    match (self.$a.cmp(&other.$a), self.$b.cmp(&other.$b)) {
                        (a, b) if a == b => Some(a),
                        _ => None
                    }
                }
            }

            // =========== //
            // Conversions //
            // =========== //

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

            doc!("Returns a new `(" s!($a) ", " s!($b) ")` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<(usize, usize)> for $Type {
                doc!("Returns a new `(" s!($a) ", " s!($b) ")` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from(($a, $b): (usize, usize)) -> Self {
                    Self {
                        $a: $A($a),
                        $b: $B($b),
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
        )*

    };
}

types!(
    /// An [`X`](crate::X) coordinate.
    x: X
    /// An [`Y`](crate::Y) coordinate.
    y: Y
    /// A [`Width`](crate::Width) distance.
    width: Width
    /// An [`Height`](crate::Height) distance.
    height: Height
);

types!(
    /// A `(x, y)` [`Point`](crate::Point).
    point: Point { x: X, y: Y }
    /// A `(width, height)` [`Size`](crate::Size).
    size: Size { width: Width, height: Height }
);

/// Returns a new `(x, y)` [`Size`](crate::Size).
impl From<Point> for Size {
    /// Returns a new `(x, y)` [`Size`](crate::Size).
    fn from(point: Point) -> Self {
        Self {
            width:  Width(point.x.0),
            height: Height(point.y.0),
        }
    }
}

/// Returns a new `(width, height)` [`Point`](crate::Point).
impl From<Size> for Point {
    /// Returns a new `(width, height)` [`Point`](crate::Point).
    fn from(size: Size) -> Self {
        Self {
            x: X(size.width.0),
            y: Y(size.height.0),
        }
    }
}
