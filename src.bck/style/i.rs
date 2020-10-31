//! Attributes `Index`ers.

macro_rules! index {
    ($($Self:ident)*) => {
        $(
            doc!("`Index`es `Option<" stringify!($Self) ">`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $Self;);
        )*
    };
}

index!(Foreground Background Weight Slant Blink Invert Strike Underline Overline Border);
