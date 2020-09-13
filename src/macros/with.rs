macro_rules! getters {
    ([$get:ident $get_mut:ident] -> $ReturnType:ty) => {
        doc!("Returns a `" stringify!($ReturnType) "`",
            fn $get(&self) -> $ReturnType;
        );
        doc!("Returns a `&mut " stringify!($ReturnType) "`",
            fn $get_mut(&mut self) -> &mut $ReturnType;
        );
    }
}

macro_rules! setter {
    ($($doc:expr)*, [$set:ident $set_mut:ident] ($($arg:ident: $ArgType:ty)*)) => {
        doc!($($doc)*,
            fn $set(mut self $(, $arg: $ArgType)*) -> Self {
                self.$set_mut($($arg,)*);
                self
            }
        );

    }
}

macro_rules! setter_mut {
    ($($doc:expr)*, [$get_mut:ident $set_mut:ident] ($($arg:ident: $ArgType:ty)*) $body:tt) => {
        doc!($($doc)*,
            fn $set_mut(&mut self $(, $arg: $ArgType)*) {
                *self.$get_mut() = { $body };
            }
        );

    }
}

macro_rules! with {
    ($($doc:expr)*, $Trait:ident {
        get { $get:ident $get_mut:ident -> $ReturnType:ty }
        set {
            $($set_doc:expr)*,
            [$set:ident $set_mut:ident]
            ($set_arg:ident: $SetArgType:ty)
            $set_body:tt
        }
        $(unset { [$unset:ident $unset_mut:ident] })?
        $(specials { $(
            $($special_doc:expr)*,
            [$special:ident $special_mut:ident]
            ($($special_arg:ident: $SpecialArgType:ty)*)
            $special_body:tt
        )* })?
        variants { $(
            $($variant_doc:expr)*,
            [$set_variant:ident $set_variant_mut:ident]
            $variant_body:tt
        )* }
    }) => {
        doc!($($doc)*,
            pub trait $Trait: Sized {
                getters!([$get $get_mut] -> $ReturnType);

                setter!($($set_doc)*,
                    [$set $set_mut]
                    ($set_arg: $SetArgType));
                setter_mut!($($set_doc)*,
                    [$get_mut $set_mut]
                    ($set_arg: $SetArgType)
                    $set_body);

                $(
                    setter!("Sets `None`",
                        [$unset $unset_mut] ());
                    setter_mut!("Sets `None`",
                        [$get_mut $unset_mut] ()
                        { None });
                )?

                $(
                    setter!($($variant_doc)*,
                        [$set_variant $set_variant_mut] ());
                    setter_mut!($($variant_doc)*,
                        [$get_mut $set_variant_mut] ()
                        $variant_body);
                )*

                $($(
                    setter!($($special_doc)*,
                        [$special $special_mut]
                        ($($special_arg: $SpecialArgType)*));
                    setter_mut!($($special_doc)*,
                        [$get_mut $special_mut]
                        ($($special_arg: $SpecialArgType)*)
                        $special_body);
                )*)?
            }
        );
    };
}

macro_rules! with_color {
    (
        $Trait:ident
        $OptionalTrait:ident
        $Type:ident {
            get { [$get:ident $get_mut:ident] }
            set { [$set:ident $set_mut:ident] }
            unset { [$unset:ident $unset_mut:ident] }
            rgb { [$set_rgb:ident $set_rgb_mut:ident] }
            ansi { [$set_ansi:ident $set_ansi_mut:ident] }
            variants {
                $($variant:ident => [$set_variant:ident $set_variant_mut:ident])*
            }
        }
    ) => {
        with!("A trait for styles with a `" stringify!($Type) "` field.",
            $Trait {
                get { $get $get_mut -> $Type }
                set {
                    "Sets `" stringify!($Type) "(color)`",
                    [$set $set_mut]
                    (color: Color)
                    { color.into() }
                }
                specials {
                    "Sets `" stringify!($Type) "(Color::Rgb { r, g, b })`",
                    [$set_rgb $set_rgb_mut]
                    (r: u8 g: u8 b: u8)
                    { Color::Rgb { r, g, b }.into() }
                    "Sets `" stringify!($Type) "(Color::AnsiValue(value))`",
                    [$set_ansi $set_ansi_mut]
                    (value: u8)
                    { Color::AnsiValue(value).into() }
                }
                variants { $(
                    "Sets `" stringify!($Type) "(Color::" stringify!($variant) ")`",
                    [$set_variant $set_variant_mut]
                    { Color::$variant.into() }
                )* }
            }
        );
        with!("A trait for styles with an `Option<" stringify!($Type) ">` field.",
            $OptionalTrait {
                get { $get $get_mut -> Option<$Type> }
                set {
                    "Sets `Some(" stringify!($Type) "(color))`",
                    [$set $set_mut]
                    (color: Color)
                    { Some(color.into()) }
                }
                unset { [$unset $unset_mut] }
                specials {
                    "Sets `Some(" stringify!($Type) "(Color::Rgb { r, g, b }))`",
                    [$set_rgb $set_rgb_mut]
                    (r: u8 g: u8 b: u8)
                    { Some(Color::Rgb { r, g, b }.into()) }
                    "Sets `Some(" stringify!($Type) "(Color::AnsiValue(value)))`",
                    [$set_ansi $set_ansi_mut]
                    (value: u8)
                    { Some(Color::AnsiValue(value).into()) }
                }
                variants { $(
                    "Sets `Some(" stringify!($Type) "(Color::" stringify!($variant) "))`",
                    [$set_variant $set_variant_mut]
                    { Some(Color::$variant.into()) }
                )* }
            }
        );
    };
}

macro_rules! with_attribute {
    (
        $Trait:ident
        $OptionalTrait:ident
        $Type:ident $type:ident {
            get { [$get:ident $get_mut:ident] }
            set { [$set:ident $set_mut:ident] }
            unset { [$unset:ident $unset_mut:ident] }
            variants {
                $($variant:ident => [$set_variant:ident $set_variant_mut:ident])*
            }
        }
    ) => {
        with!("A trait for styles with a `" stringify!($Type) "` field.",
            $Trait {
                get { $get $get_mut -> $Type }
                set {
                    "Sets `" stringify!($type) "`",
                    [$set $set_mut]
                    ($type: $Type)
                    { $type }
                }
                variants { $(
                    "Sets `" stringify!($Type) "::" stringify!($variant) "`",
                    [$set_variant $set_variant_mut]
                    { <$Type>::$variant }
                )* }
            }
        );
        with!("A trait for styles with an `Option<" stringify!($Type) ">` field.",
            $OptionalTrait {
                get { $get $get_mut -> Option<$Type> }
                set {
                    "Sets `Some(" stringify!($type) ")`",
                    [$set $set_mut]
                    ($type: $Type)
                    { Some($type) }
                }
                unset { [$unset $unset_mut] }
                variants { $(
                    "Sets `Some(" stringify!($Type) "::" stringify!($variant) ")`",
                    [$set_variant $set_variant_mut]
                    { Some(<$Type>::$variant) }
                )* }
            }
        );
    };
}
