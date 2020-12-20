macro_rules! consts {
    ($($Trait:ident $const:ident $([$ival:literal $fval:literal])?)*) => { $(
        doc!("[`" stringify!($Trait) "`](crate::" stringify!($Trait) ") constants.",
        pub trait $Trait {
            doc!("[`" stringify!($const) "`](crate::" stringify!($Trait) "::" stringify!($const) ") constant.",
            const $const: Self;);
        });

        consts!(impl $Trait $const
            $($ival)? u8 i8 u16 i16 u32 i32 u128 i128 usize isize
            $($fval)? f32 f64
        );
    )* };
    (impl $Trait:ident $const:ident $($val:literal $($T:ty)*)*) => { $( $(
        impl $Trait for $T {
            doc!("```\n"
                "# use lay::*;\n"
                "assert!(<" stringify!($T) " as " stringify!($Trait) ">::" stringify!($const)
                " == " stringify!($val) ");\n"
                "```",
            const $const: Self = $val;);
        }
    )* )* };
    (impl $Trait:ident $const:ident $($T:ty)*) => { $(
        impl $Trait for $T {
            doc!("```\n"
                "# use lay::*;\n"
                "assert!(<" stringify!($T) " as " stringify!($Trait) ">::" stringify!($const)
                " == " stringify!($T) "::" stringify!($const) ");\n"
                "```",
            const $const: Self = Self::$const;);
        }
    )* };
}

consts!(
    Zero ZERO [0 0.]
    One  ONE  [1 1.]
    Min  MIN
    Max  MAX
);
