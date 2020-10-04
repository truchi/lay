macro_rules! no {
    ($($Self:ident $No:ident)*) => {
        $(
            #[cfg(feature = "styler-ops")]
            $crate::doc!("Sets `Option<" stringify!($Self) ">` to `None`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $No;);
        )*
    };
}
