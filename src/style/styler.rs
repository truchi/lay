macro_rules! styler {
    (
        Colors { $(
            $Color:ident($color:ident) {
                $get_color:ident $get_mut_color:ident
                $set_color:ident $set_mut_color:ident
                $unset_color:ident $unset_mut_color:ident
                Rgb: $set_rgb:ident $set_rgb_mut:ident
                Ansi: $set_ansi:ident $set_ansi_mut:ident
                $reset_color:ident: $set_reset_color:ident $set_reset_mut_color:ident
                $($variant_color:ident: $set_variant_color:ident $set_variant_mut_color:ident)*
            }
        )* }
        Attributes { $(
            $Attr:ident($attr:ident) {
                $get_attr:ident $get_mut_attr:ident
                $set_attr:ident $set_mut_attr:ident
                $unset_attr:ident $unset_mut_attr:ident
                $reset_attr:ident: $set_reset_attr:ident $set_reset_mut_attr:ident
                $($variant_attr:ident: $set_variant_attr:ident $set_variant_mut_attr:ident)*
            }
        )* }
    ) => {
        /// A trait for getting `Option`al attributes on styled types.
        pub trait StylerIndex {
           $(__styler!([get] $Color $get_color);)*
           $(__styler!([get] $Attr $get_attr);)*
        }

        /// A trait for getting `Option`al attributes on mutable styled types.
        pub trait StylerIndexMut {
           $(__styler!([get mut] $Color $get_mut_color);)*
           $(__styler!([get mut] $Attr $get_mut_attr);)*
        }

        /// A trait for setting `Option`al attributes on styled types.
        pub trait Styler: StylerIndex + Sized {
            $(
                __styler!([set] $Color($color) $set_color);
                __styler!([unset] $Color $unset_color $set_color);
                $(__styler!([variant]
                    stringify!($Color) "(Color::" stringify!($variant_color) ")",
                    $set_variant_color $set_color($Color(Color::$variant_color))
                );)*
                __styler!([rgb] $Color $set_rgb $set_color);
                __styler!([ansi] $Color $set_ansi $set_color);
                __styler!([variant]
                    stringify!($Color) "(Color::" stringify!($reset_color) ")",
                    $set_reset_color $set_color($Color(Color::$reset_color))
                );
            )*
            $(
                __styler!([set] $Attr($attr) $set_attr);
                __styler!([unset] $Attr $unset_attr $set_attr);
                $(__styler!([variant]
                    stringify!($Attr) "::" stringify!($variant_attr),
                    $set_variant_attr $set_attr($Attr::$variant_attr)
                );)*
                __styler!([variant]
                    stringify!($Attr) "::" stringify!($reset_attr),
                    $set_reset_attr $set_attr($Attr::$reset_attr)
                );
            )*

            __styler!([and] and $($get_color $set_color)* $($get_attr $set_attr)*);
            __styler!([or ] or  $($get_color $set_color)* $($get_attr $set_attr)*);
            __styler!([xor] xor $($get_color $set_color)* $($get_attr $set_attr)*);
            __styler!([dedup] $($get_color $set_color)* $($get_attr $set_attr)*);
            __styler!([reset]
                $($get_color $set_color($Color(Color::$reset_color)))*
                $($get_attr  $set_attr(($Attr::$reset_attr)))*
            );
            __styler!([fmt] $($get_color)* $($get_attr)*);
        }

        /// A trait for setting `Option`al attributes on mutable styled types.
        pub trait StylerMut: StylerIndex {
            $(
                __styler!([set mut] $Color($color) $set_mut_color);
                __styler!([unset mut] $Color $unset_mut_color $set_mut_color);
                $(__styler!([variant mut]
                    stringify!($Color) "(Color::" stringify!($variant_color) ")",
                    $set_variant_mut_color $set_mut_color($Color(Color::$variant_color))
                );)*
                __styler!([rgb mut] $Color $set_rgb_mut $set_mut_color);
                __styler!([ansi mut] $Color $set_ansi_mut $set_mut_color);
                __styler!([variant mut]
                    stringify!($Color) "(Color::" stringify!($reset_color) ")",
                    $set_reset_mut_color $set_mut_color($Color(Color::$reset_color))
                );
            )*
            $(
                __styler!([set mut] $Attr($attr) $set_mut_attr);
                __styler!([unset mut] $Attr $unset_mut_attr $set_mut_attr);
                $(__styler!([variant mut]
                    stringify!($Attr) "::" stringify!($variant_attr),
                    $set_variant_mut_attr $set_mut_attr($Attr::$variant_attr)
                );)*
                __styler!([variant mut]
                    stringify!($Attr) "::" stringify!($reset_attr),
                    $set_reset_mut_attr $set_mut_attr($Attr::$reset_attr)
                );
            )*

            __styler!([and mut] and_mut $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            __styler!([or  mut] or_mut  $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            __styler!([xor mut] xor_mut $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            __styler!([dedup mut] $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            __styler!([reset mut]
                $($get_color $set_mut_color($Color(Color::$reset_color)))*
                $($get_attr  $set_mut_attr(($Attr::$reset_attr)))*
            );
        }
   };
}

macro_rules! __styler {
    ([get] $Self:ident $get:ident) => {
        $crate::doc!("Gets `Option<" stringify!($Self) ">`.",
        fn $get(&self) -> Option<$Self>;);
    };
    ([get mut] $Self:ident $get_mut:ident) => {
        $crate::doc!("Gets `Option<" stringify!($Self) ">`.",
        fn $get_mut(&mut self) -> &mut Option<$Self>;);
    };

    ([set] $Self:ident($self:ident) $set:ident) => {
        $crate::doc!("Sets `Option<" stringify!($Self) ">`.",
        fn $set(self, $self: impl Into<Option<$Self>>) -> Self;);
    };
    ([set mut] $Self:ident($self:ident) $set_mut:ident) => {
        $crate::doc!("Sets `Option<" stringify!($Self) ">` mutably.",
        fn $set_mut(&mut self, $self: impl Into<Option<$Self>>););
    };

    ([unset] $Self:ident $unset:ident $set:ident) => {
        $crate::doc!("Sets " stringify!($self) " to `None`.",
        fn $unset(self) -> Self {
            self.$set(None)
        });
    };
    ([unset mut] $Self:ident $unset_mut:ident $set_mut:ident) => {
        $crate::doc!("Sets " stringify!($self) " to `None` mutably.",
        fn $unset_mut(&mut self) {
            self.$set_mut(None);
        });
    };

    ([variant] $($doc:expr)*, $set_variant:ident $set:ident($body:expr)) => {
        $crate::doc!("Sets `Some(" $($doc)* ")`.",
        fn $set_variant(self) -> Self {
            self.$set(Some($body))
        });
    };
    ([variant mut] $($doc:expr)*, $set_variant_mut:ident $set_mut:ident($body:expr)) => {
        $crate::doc!("Sets `Some(" $($doc)* ")` mutably.",
        fn $set_variant_mut(&mut self) {
            self.$set_mut(Some($body));
        });
    };

    ([rgb] $Self:ident $set_rgb:ident $set:ident) => {
        $crate::doc!("Sets `Some(" stringify!($Self) "(Color::Rgb(r, g, b)))`.",
        fn $set_rgb(self, r: u8, g: u8, b: u8) -> Self {
            self.$set(Some($Self(Color::Rgb(r, g, b))))
        });
    };
    ([rgb mut] $Self:ident $set_rgb_mut:ident $set_mut:ident) => {
        $crate::doc!("Sets `Some(" stringify!($Self) "(Color::Rgb(r, g, b)))` mutably.",
        fn $set_rgb_mut(&mut self, r: u8, g: u8, b: u8) {
            self.$set_mut(Some($Self(Color::Rgb(r, g, b))));
        });
    };

    ([ansi] $Self:ident $set_ansi:ident $set:ident) => {
        $crate::doc!("Sets `Some(" stringify!($Self) "(Color::Ansi(ansi)))`.",
        fn $set_ansi(self, ansi: u8) -> Self {
            self.$set(Some($Self(Color::Ansi(ansi))))
        });
    };
    ([ansi mut] $Self:ident $set_ansi_mut:ident $set_mut:ident) => {
        $crate::doc!("Sets `Some(" stringify!($Self) "(Color::Ansi(ansi)))` mutably.",
        fn $set_ansi_mut(&mut self, ansi: u8) {
            self.$set_mut(Some($Self(Color::Ansi(ansi))));
        });
    };

    ([$op:ident] $fn:ident $($get:ident $set:ident)*) => {
        $crate::doc!("`Option::" stringify!($op) "` fields.",
        fn $fn(mut self, other: &impl Styler) -> Self {
            $(
                let $op = self.$get().$op(other.$get());
                self = self.$set($op);
            )*
            self
        });

    };
    ([$op:ident mut] $fn_mut:ident $($get:ident $set_mut:ident)*) => {
        $crate::doc!("`Option::" stringify!($op) "` fields mutably.",
        fn $fn_mut(&mut self, other: &impl Styler) {
            $(self.$set_mut(self.$get().$op(other.$get()));)*
        });
    };

    ([dedup] $($get:ident $set:ident)*) => {
        /// Dedups (`None`s if identicals) fields.
        fn dedup(mut self, before: &impl Styler) -> Self {
            $(if self.$get() == before.$get() {
                self = self.$set(None);
            })*
            self
        }
    };
    ([dedup mut] $($get:ident $set_mut:ident)*) => {
        /// Dedups (`None`s if identicals) fields mutably.
        fn dedup_mut(&mut self, before: &impl Styler) {
            $(if self.$get() == before.$get() {
                self.$set_mut(None);
            })*
        }
    };

    // TODO None if already reset
    ([reset] $($get:ident $set:ident($body:expr))*) => {
        /// Resets (sets to reset value) fields which are `Some`.
        fn reset(mut self) -> Self {
            $(if let Some(_) = self.$get() {
                self = self.$set(Some($body));
            })*
            self
        }
    };
    ([reset mut] $($get:ident $set_mut:ident($body:expr))*) => {
        /// Resets (sets to reset value) fields which are `Some` mutably.
        fn reset_mut(&mut self) {
            $(if let Some(_) = self.$get() {
                self.$set_mut(Some($body));
            })*
        }
    };

    ([fmt] $($get:ident)*) => {
        /// Formats the CSIs of `self`'s `Some` fields.
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            $(if let Some(t) = self.$get() { t.fmt(f)?; })*
            Ok(())
        }
    }
}
