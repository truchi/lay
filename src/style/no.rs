macro_rules! no {
    ($($Self:ident)*) => {
        $(
            doc!("Sets `Option<" stringify!($Self) ">` to `None`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $Self;);
        )*
    };
}

no!(Foreground Background Weight Slant Blink Invert Strike Underline Overline Border);
