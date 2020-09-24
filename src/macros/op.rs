/// op
#[macro_export]
macro_rules! op {
    // ======
    // Public
    // ======

    // Explicit, inline op
    ($(
        $(#[$meta:meta])* $(<$($G:ident $(: $B:path)?,)*>)? ($self:ident: $Self:ty [$op:tt] $rhs:tt: $Rhs:ty)
        $($(#[$ometa:meta])* -> $Output:ty)? $body:block
    )*) => {
        $(op!(@impl
            $(#[$meta])* <$($($G $(: $B)?,)*)?> $op $Self:
            $(#[$meta])* ($self, $rhs: $Rhs)
            $($(#[$ometa])* -> $Output)? $body
        );)*
    };

    // Explicit
    ($(
        $(#[$Meta:meta])* [$op:tt]
        $(#[$meta:meta])* $(<$($G:ident $(: $B:path)?,)*>)? ($self:ident: $Self:ty, $rhs:tt: $Rhs:ty)
        $($(#[$ometa:meta])* -> $Output:ty)? $body:block
    )*) => {
        $(op!(@impl
            $(#[$Meta])* <$($($G $(: $B)?,)*)?> $op $Self:
            $(#[$meta])* ($self, $rhs: $Rhs)
            $($(#[$ometa])* -> $Output)? $body
        );)*
    };

    // Factored by Self, inline op
    ($($Self:ty {
        $(
            $(#[$meta:meta])* $(<$($G:ident $(: $B:path)?,)*>)? ($self:ident [$op:tt] $rhs:tt: $Rhs:ty)
            $($(#[$ometa:meta])* -> $Output:ty)? $body:block
        )*
    })*) => {
        $($(op!(@impl
            $(#[$meta])* <$($($G $(: $B)?,)*)?> $op $Self:
            $(#[$meta])* ($self, $rhs: $Rhs)
            $($(#[$ometa])* -> $Output)? $body
        );)*)*
    };

    // Factored by Self
    ($($Self:ty {
        $(
            $(#[$Meta:meta])* [$op:tt]
            $(#[$meta:meta])* $(<$($G:ident $(: $B:path)?,)*>)? ($self:ident, $rhs:tt: $Rhs:ty)
            $($(#[$ometa:meta])* -> $Output:ty)? $body:block
        )*
    })*) => {
        $($(op!(@impl
            $(#[$Meta])* <$($($G $(: $B)?,)*)?> $op $Self:
            $(#[$meta])* ($self, $rhs: $Rhs)
            $($(#[$ometa])* -> $Output)? $body
        );)*)*
    };

    // Factored by Self & Op, inline Rhs
    ($($Self:ty {
        $([$op:tt] {
            $(
                $(#[$meta:meta])* $(<$($G:ident $(: $B:path)?,)*>)? ($self:ident, $rhs:tt: $Rhs:ty)
                $($(#[$ometa:meta])* -> $Output:ty)? $body:block
            )*
        })*
    })*) => {
        $($($(op!(@impl
            $(#[$meta])* <$($($G $(: $B)?,)*)?> $op $Self:
            $(#[$meta])* ($self, $rhs: $Rhs)
            $($(#[$ometa])* -> $Output)? $body
        );)*)*)*
    };

    // Factored by Self & Op
    ($($Self:ty {
        $([$op:tt] {
            $(
                $(#[$Meta:meta])* $Rhs:ty:
                $(#[$meta:meta])* $(<$($G:ident $(: $B:path)?,)*>)? ($self:ident, $rhs:tt)
                $($(#[$ometa:meta])* -> $Output:ty)? $body:block
            )*
        })*
    })*) => {
        $($($(op!(@impl
            $(#[$Meta])* <$($($G $(: $B)?,)*)?> $op $Self:
            $(#[$meta])* ($self, $rhs: $Rhs)
            $($(#[$ometa])* -> $Output)? $body
        );)*)*)*
    };

    // Factored by Self & Rhs, inline op
    ($($Self:ty {
        $($Rhs:ty {
            $(
                $(#[$meta:meta])* $(<$($G:ident $(: $B:path)?,)*>)? ($self:ident [$op:tt] $rhs:tt)
                $($(#[$ometa:meta])* -> $Output:ty)? $body:block
            )*
        })*
    })*) => {
        $($($(op!(@impl
            $(#[$meta])* <$($($G $(: $B)?,)*)?> $op $Self:
            $(#[$meta])* ($self, $rhs: $Rhs)
            $($(#[$ometa])* -> $Output)? $body
        );)*)*)*
    };

    // Factored by Self & Rhs
    ($($Self:ty {
        $($Rhs:ty {
            $(
                $(#[$Meta:meta])* [$op:tt]
                $(#[$meta:meta])* $(<$($G:ident $(: $B:path)?,)*>)? ($self:ident, $rhs:tt)
                $($(#[$ometa:meta])* -> $Output:ty)? $body:block
            )*
        })*
    })*) => {
        $($($(op!(@impl
            $(#[$Meta])* <$($($G $(: $B)?,)*)?> $op $Self:
            $(#[$meta])* ($self, $rhs: $Rhs)
            $($(#[$ometa])* -> $Output)? $body
        );)*)*)*
    };

    // ===============
    // Implementations
    // ===============

    // Unary
    // TODO
    () => {};

    // Binary
    (@impl
        $(#[$Meta:meta])* <$($G:ident $(: $B:path)?,)*> $Op:ident $Type:ty:
        $(#[$meta:meta])* $op:ident ($self:ident, $rhs:tt: $Rhs:ty)
        $(#[$ometa:meta])* -> $Output:ty $body:block
    ) => {
        $(#[$Meta])*
        impl<$($G $(: $B)?,)*> ::std::ops::$Op<$Rhs> for $Type {
            $(#[$ometa])*
            type Output = $Output;

            $(#[$meta])*
            fn $op(self, $rhs: $Rhs) -> $Output {
                let $self = self;
                $body
            }
        }
    };

    // Binary Assign
    (@impl
        $(#[$Meta:meta])* <$($G:ident $(: $B:path)?,)*> $Op:ident $Type:ty:
        $(#[$meta:meta])* $op:ident ($self:ident, $rhs:tt: $Rhs:ty) $body:block
    ) => {
        $(#[$Meta])*
        impl<$($G $(: $B)?,)*> ::std::ops::$Op<$Rhs> for $Type {
            $(#[$meta])*
            fn $op(&mut self, $rhs: $Rhs) {
                let $self = self;
                $body
            }
        }
    };

    // ==========
    // Operations
    // ==========

    // Add
    (@impl
        $(#[$Meta:meta])* <$($G:ident $(: $B:path)?,)*> + $Type:ty:
        $(#[$meta:meta])* ($self:ident, $rhs:tt: $Rhs:ty)
        $(#[$ometa:meta])* -> $Output:ty $body:block
    ) => {
        op!(@impl
            $(#[$Meta])* <$($G $(: $B)?,)*> Add $Type:
            $(#[$meta])* add($self, $rhs: $Rhs)
            $(#[$ometa])* -> $Output $body
        );
    };

    // AddAssign
    (@impl
        $(#[$Meta:meta])* <$($G:ident $(: $B:path)?,)*> += $Type:ty:
        $(#[$meta:meta])* ($self:ident, $rhs:tt: $Rhs:ty) $body:block
    ) => {
        op!(@impl
            $(#[$Meta])* <$($G $(: $B)?,)*> AddAssign $Type:
            $(#[$meta])* add_assign($self, $rhs: $Rhs) $body
        );
    };

    // Add auto
    (@impl
        $(#[$Meta:meta])* <$($G:ident $(: $B:path)?,)*> + $Type:ty:
        $(#[$meta:meta])* ($self:ident, $rhs:tt: $Rhs:ty)
        $(#[$ometa:meta])* -> $Output:ty [auto]
    ) => {
        op!(@impl
            $(#[$Meta])* <$($G $(: $B)?,)*> Add $Type:
            $(#[$meta])* add($self, $rhs: $Rhs)
            $(#[$ometa])* -> $Output {
                <$Type as ::std::ops::AddAssign>::add_assign(&mut $self, $rhs);
                $self
            }
        );
    };

    // Sub
    (@impl
        $(#[$Meta:meta])* <$($G:ident $(: $B:path)?,)*> - $Type:ty:
        $(#[$meta:meta])* ($self:ident, $rhs:tt: $Rhs:ty)
        $(#[$ometa:meta])* -> $Output:ty $body:block
    ) => {
        op!(@impl
            $(#[$Meta])* <$($G $(: $B)?,)*> Sub $Type:
            $(#[$meta])* sub($self, $rhs: $Rhs)
            $(#[$ometa])* -> $Output $body
        );
    };

    // SubAssign
    (@impl
        $(#[$Meta:meta])* <$($G:ident $(: $B:path)?,)*> $(-)?$(-=)? $Type:ty:
        $(#[$meta:meta])* ($self:ident, $rhs:tt: $Rhs:ty) $body:block
    ) => {
        op!(@impl
            $(#[$Meta])* <$($G $(: $B)?,)*> SubAssign $Type:
            $(#[$meta])* sub_assign($self, $rhs: $Rhs) $body
        );
    };
}
// Add +
// Sub -
// Mul *
// Div /
// Rem %
// Shl <<
// Shr >>
// BitAnd &
// BitOr |
// BitXor ^
// Not !
// Neg -

/*
Type
    <>+(){}
    <>+(){}{}
<>(self: Type [+] rhs: Rhs) -> Output {} {}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn explicit() {
        struct S<T, U>(T, U);

        op!(
            /// IMPL
            [+]
            /// METHOD
            <T,>(o1: S<T, u8>, o2: S<T, u8>)
            /// OUTPUT
            -> u8 { o1.1 + o2.1 }
            /// IMPL
            [+]
            /// METHOD
            <T,>(o1: S<T, u16>, o2: S<T, u16>)
            /// OUTPUT
            -> u16 { o1.1 + o2.1 }
        );
    }

    #[test]
    fn explicit_inline_op() {
        struct S<T, U>(T, U);

        op!(
            /// METHOD
            <T,>(o1: S<T, u8> [+] o2: S<T, u8>)
            /// OUTPUT
            -> u8 { o1.1 + o2.1 }
            /// METHOD
            <T,>(o1: S<T, u16> [+] o2: S<T, u16>)
            /// OUTPUT
            -> u16 { o1.1 + o2.1 }
        );
    }

    #[test]
    fn factored_by_self() {
        struct S<T, U>(T, U);

        op!(
            S<T, u8> {
                /// IMPL
                [+]
                /// METHOD
                <T,>(o1, o2: S<T, u8>)
                /// OUTPUT
                -> u8 { o1.1 + o2.1 }
                /// IMPL
                [+]
                /// METHOD
                <T,>(o1, o2: S<T, u16>)
                /// OUTPUT
                -> u16 { o1.1 as u16 + o2.1 }
            }
            S<T, u16> {
                /// IMPL
                [+]
                /// METHOD
                <T,>(o1, o2: S<T, u8>)
                /// OUTPUT
                -> u8 { o1.1 as u8 + o2.1 }
                /// IMPL
                [+]
                /// METHOD
                <T,>(o1, o2: S<T, u16>)
                /// OUTPUT
                -> u16 { o1.1 + o2.1 }
            }
        );
    }

    #[test]
    fn factored_by_self_inlined_op() {
        struct S<T, U>(T, U);

        op!(
            S<T, u8> {
                /// METHOD
                <T,>(o1 [+] o2: S<T, u8>)
                /// OUTPUT
                -> u8 { o1.1 + o2.1 }
                /// METHOD
                <T,>(o1 [+] o2: S<T, u16>)
                /// OUTPUT
                -> u16 { o1.1 as u16 + o2.1 }
            }
            S<T, u16> {
                /// METHOD
                <T,>(o1 [+] o2: S<T, u8>)
                /// OUTPUT
                -> u8 { o1.1 as u8 + o2.1 }
                /// METHOD
                <T,>(o1 [+] o2: S<T, u16>)
                /// OUTPUT
                -> u16 { o1.1 + o2.1 }
            }
        );
    }

    #[test]
    fn factored_by_self_and_op() {
        struct S<T, U>(T, U);

        op!(
            S<T, u8> {
                [+] {
                    /// IMPL
                    S<T, u8>:
                    /// METHOD
                    <T,>(o1, o2)
                    /// OUTPUT
                    -> u8 { o1.1 + o2.1 }
                    /// IMPL
                    S<T, u16>:
                    /// METHOD
                    <T,>(o1, o2)
                    /// OUTPUT
                    -> u16 { o1.1 as u16 + o2.1 }
                }
                [-] {
                    /// IMPL
                    S<T, u8>:
                    /// METHOD
                    <T,>(o1, o2)
                    /// OUTPUT
                    -> u8 { o1.1 - o2.1 }
                }
            }
            S<T, u16> {
                [+] {
                    /// IMPL
                    S<T, u8>:
                    /// METHOD
                    <T,>(o1, o2)
                    /// OUTPUT
                    -> u8 { o1.1 as u8 + o2.1 }
                }
                [-] {
                    /// IMPL
                    S<T, u8>:
                    /// METHOD
                    <T,>(o1, o2)
                    /// OUTPUT
                    -> u8 { o1.1 as u8 - o2.1 }
                }
            }
        );
    }

    #[test]
    fn factored_by_self_and_op_inline_rhs() {
        struct S<T, U>(T, U);

        op!(
            S<T, u8> {
                [+] {
                    /// METHOD
                    <T,>(o1, o2: S<T, u8>)
                    /// OUTPUT
                    -> u8 { o1.1 + o2.1 }
                    /// METHOD
                    <T,>(o1, o2: S<T, u16>)
                    /// OUTPUT
                    -> u16 { o1.1 as u16 + o2.1 }
                }
                [-] {
                    /// METHOD
                    <T,>(o1, o2: S<T, u8>)
                    /// OUTPUT
                    -> u8 { o1.1 - o2.1 }
                }
            }
            S<T, u16> {
                [+] {
                    /// METHOD
                    <T,>(o1, o2: S<T, u8>)
                    /// OUTPUT
                    -> u8 { o1.1 as u8 + o2.1 }
                }
                [-] {
                    /// METHOD
                    <T,>(o1, o2: S<T, u8>)
                    /// OUTPUT
                    -> u8 { o1.1 as u8 - o2.1 }
                }
            }
        );
    }

    #[test]
    fn factored_by_self_and_rhs() {
        struct S<T, U>(T, U);

        op!(
            S<T, u8> {
                S<T, u8> {
                    /// IMPL
                    [+]
                    /// METHOD
                    <T,>(o1, o2)
                    /// OUTPUT
                    -> u8 { o1.1 + o2.1 }
                    /// IMPL
                    [-]
                    /// METHOD
                    <T,>(o1, o2)
                    /// OUTPUT
                    -> u8 { o1.1 - o2.1 }
                }
                S<T, u16> {
                    /// IMPL
                    [+]
                    /// METHOD
                    <T,>(o1, o2)
                    /// OUTPUT
                    -> u8 { o1.1 + o2.1 as u8 }
                }
            }
        );
    }

    #[test]
    fn factored_by_self_and_rhs_inline_op() {
        struct S<T, U>(T, U);

        op!(
            S<T, u8> {
                S<T, u8> {
                    /// METHOD
                    <T,>(o1 [+] o2)
                    /// OUTPUT
                    -> u8 { o1.1 + o2.1 }
                    /// METHOD
                    <T,>(o1 [-] o2)
                    /// OUTPUT
                    -> u8 { o1.1 - o2.1 }
                }
            }
        );
    }
}
