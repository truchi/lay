#[cfg(feature = "styler-idx")]
macro_rules! index {
    ($($Self:ident $Idx:ident)*) => {
        $(
            doc!("Indexes `Option<" stringify!($Self) ">`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $Idx;);
        )*
    };
}
