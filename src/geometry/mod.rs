//! # Geometry utilities

mod gen;
mod rect;

pub use rect::*;

use crate::*;
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
    // 0D types //
    // ======== //
    ($(#[$doc:meta] $type:ident: $Type:ident)*) => {
        $(
            #[$doc]
            #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
            pub struct $Type(pub u16);

            // ============== //
            // Implementation //
            // ============== //

            /// ### Consts
            impl $Type {
                doc!("A [`" s!($Type) "`](crate::" s!($Type) ") of `0`.",
                pub const ZERO: $Type = $Type(0););

                doc!("A [`" s!($Type) "`](crate::" s!($Type) ") of `1`.",
                pub const ONE: $Type = $Type(1););
            }

            /// ### Methods
            impl $Type {
                /// Clamps `self` within `[0, to - from]`.
                pub fn clamp_into(self, from: Self, to: Self) -> Self {
                    if to <= from {
                        Self::ZERO
                    } else {
                        self.min(to - from)
                    }
                }
            }

            // ====== //
            // Traits //
            // ====== //

            /// `Deref`s to `u16`.
            impl Deref for $Type {
                type Target = u16;

                /// `Deref`s to `u16`.
                fn deref(&self) -> &u16 {
                    &self.0
                }
            }

            /// `DerefMut`s to `u16`.
            impl DerefMut for $Type {
                /// `DerefMut`s to `u16`.
                fn deref_mut(&mut self) -> &mut u16 {
                    &mut self.0
                }
            }

            impl Debug for $Type {
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    write!(f, "{:?}", self.0)
                }
            }

            // =========== //
            // Conversions //
            // =========== //

            doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `" s!($type) "`.",
            impl From<$Type> for u16 {
                doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `" s!($type) "`.",
                fn from($type: $Type) -> Self {
                    $type.0
                });
            });

            doc!("Returns a new `" s!($type) "` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<u16> for $Type {
                doc!("Returns a new `" s!($type) "` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from($type: u16) -> Self {
                    Self($type)
                });
            });
        )*
    };
    // ======== //
    // 1D types //
    // ======== //
    ($(#[$doc:meta] $type:ident: $Type:ident { $a:ident: $A:ident, $b:ident: $B:ident })*) => {
        $(
            #[$doc]
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
            pub struct $Type { pub $a: $A, pub $b: $B }

            // ============== //
            // Implementation //
            // ============== //

            /// ### Consts
            impl $Type {
                doc!("A [`" s!($Type) "`](crate::" s!($Type) ") of `(0, 0)`.",
                pub const ZERO: $Type = $Type { $a: $A::ZERO, $b: $B::ZERO };);

                doc!("A [`" s!($Type) "`](crate::" s!($Type) ") of `(1, 1)`.",
                pub const ONE: $Type = $Type { $a: $A::ONE, $b: $B::ONE };);
            }

            /// ### Constructors
            impl $Type {
                doc!("Returns a new [`" s!($Type) "`](crate::" s!($Type) ").",
                pub fn new($a: $A, $b: $B) -> Self {
                    Self { $a, $b }
                });
            }

            /// ### Methods
            impl $Type {
                /// Clamps into field by field.
                pub fn clamp_into(self, from: Self, to: Self) -> Self {
                    Self {
                        $a: self.$a.clamp_into(from.$a, to.$a),
                        $b: self.$b.clamp_into(from.$b, to.$b),
                    }
                }
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

            impl Debug for $Type {
                fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                    write!(f, "({:?}, {:?})", self.$a, self.$b)
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
            impl From<$Type> for (u16, u16) {
                doc!("Returns the [`" s!($Type) "`](crate::" s!($Type) ")'s as `(" s!($a) ", " s!($b) ")`.",
                fn from($type: $Type) -> Self {
                    ($type.$a.0, $type.$b.0)
                });
            });

            doc!("Returns a new `(value, value)` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<u16> for $Type {
                doc!("Returns a new `(value, value)` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from(value: u16) -> Self {
                    Self {
                        $a: $A(value),
                        $b: $B(value),
                    }
                });
            });

            doc!("Returns a new `(" s!($a) ", " s!($b) ")` [`" s!($Type) "`](crate::" s!($Type) ").",
            impl From<(u16, u16)> for $Type {
                doc!("Returns a new `(" s!($a) ", " s!($b) ")` [`" s!($Type) "`](crate::" s!($Type) ").",
                fn from(($a, $b): (u16, u16)) -> Self {
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

macro_rules! xfrom {
    ($($type1:ident: $Type1:ident <-> $type2:ident: $Type2:ident)*) => { $(
        xfrom!($type1: $Type1 -> $Type2);
        xfrom!($type2: $Type2 -> $Type1);
    )* };
    ($type1:ident: $Type1:ident -> $Type2:ident) => {
        doc!("Returns a new [`" s!($Type2) "`](crate::" s!($Type2) ").",
        impl From<$Type1> for $Type2 {
            doc!("Returns a new [`" s!($Type2) "`](crate::" s!($Type2) ").",
            fn from($type1: $Type1) -> Self {
                Self($type1.0)
            });
        });
    };
    ($(
        $type1:ident: $Type1:ident { $a1:ident: $A1:ident, $b1:ident: $B1:ident } <->
        $type2:ident: $Type2:ident { $a2:ident: $A2:ident, $b2:ident: $B2:ident }
    )*) => { $(
        xfrom!($type1: $Type1 { $a1, $b1 } -> $Type2 { $a2: $A2, $b2: $B2 });
        xfrom!($type2: $Type2 { $a2, $b2 } -> $Type1 { $a1: $A1, $b1: $B1 });
    )* };
    (
        $type1:ident: $Type1:ident { $a1:ident, $b1:ident } ->
        $Type2:ident { $a2:ident: $A2:ident, $b2:ident: $B2:ident }
    ) => {
        doc!("Returns `" s!($type1) "` as a new [`" s!($Type2) "`](crate::" s!($Type2) ").",
        impl From<$Type1> for $Type2 {
            doc!("Returns `" s!($type1) "` as a new [`" s!($Type2) "`](crate::" s!($Type2) ").",
            fn from($type1: $Type1) -> Self {
                Self {
                    $a2: $A2($type1.$a1.0),
                    $b2: $B2($type1.$b1.0),
                }
            });
        });
    };

}

xfrom!(
    x: X <-> width: Width
    y: Y <-> height: Height
);

xfrom!(point: Point { x: X, y: Y } <-> size: Size { width: Width, height: Height });
