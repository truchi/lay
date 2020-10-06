macro_rules! styler {
    (Colors { $(
        $Color:ident($color:ident) {
            $get_color:ident $get_mut_color:ident
            $set_color:ident $set_mut_color:ident
            $unset_color:ident $unset_mut_color:ident
            Rgb: $set_rgb:ident $set_rgb_mut:ident
            Ansi: $set_ansi:ident $set_ansi_mut:ident
            $reset_color:ident: $set_reset_color:ident $set_reset_mut_color:ident
            $($variant_color:ident: $set_variant_color:ident $set_variant_mut_color:ident)*
        } )*
    }
    Attributes { $(
        $Attr:ident($attr:ident) {
            $get_attr:ident $get_mut_attr:ident
            $set_attr:ident $set_mut_attr:ident
            $unset_attr:ident $unset_mut_attr:ident
            $reset_attr:ident: $set_reset_attr:ident $set_reset_mut_attr:ident
            $($variant_attr:ident: $set_variant_attr:ident $set_variant_mut_attr:ident)*
        } )*
    }) => {
        /// A trait for getting `Option`al attributes on styled types.
        pub trait StylerIndex {
            priv_styler!(
                $($get_color(&Self) -> Option<$Color>)*
                $($get_attr (&Self) -> Option<$Attr>)*
            );

            /// Returns a `Style`.
            fn get_style(&self) -> Style {
                Style {
                    $($color: self.$get_color(),)*
                    $($attr : self.$get_attr(),)*
                }
            }
        }

        /// A trait for getting `Option`al attributes on mutable styled types.
        pub trait StylerIndexMut {
            priv_styler!(
                $($get_mut_color(&mut Self) -> &mut Option<$Color>)*
                $($get_mut_attr (&mut Self) -> &mut Option<$Attr>)*
            );
        }

        /// A trait for setting `Option`al attributes on styled types.
        pub trait Styler: StylerIndex + Sized {
            /// The resulting type of the setters.
            type Output;

            priv_styler!((Self) -> Self::Output;
                $(($color: $Color) {
                    $set_color
                    $unset_color
                    { $set_rgb $set_ansi }
                    [$(
                        stringify!($Color) "(Color::" stringify!($variant_color) ")",
                        $set_variant_color($Color(Color::$variant_color))
                    )*]
                    stringify!($Color) "(Color::" stringify!($reset_color) ")",
                    $set_reset_color($Color(Color::$reset_color))
                })*
                $(($attr: $Attr) {
                    $set_attr $unset_attr
                    [$(
                        stringify!($Attr) "::" stringify!($variant_attr),
                        $set_variant_attr($Attr::$variant_attr)
                    )*]
                    stringify!($Attr) "::" stringify!($reset_attr),
                    $set_reset_attr($Attr::$reset_attr)
                })*
            );

            priv_styler!((Self) (
                    $($get_color $set_color($Color(Color::$reset_color)))*
                    $($get_attr  $set_attr ($Attr::$reset_attr))*
                )
                style and or xor dedup reset
            );

            /// Formats the CSIs of `self`'s `Some` fields.
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                $(if let Some(t) = self.$get_color() { t.fmt(f)?; })*
                $(if let Some(t) = self.$get_attr () { t.fmt(f)?; })*
                Ok(())
            }
        }

        /// A trait for setting `Option`al attributes on mutable styled types.
        pub trait StylerMut: StylerIndex {
            priv_styler!((&mut Self) -> ();
                $(($color: $Color) {
                    $set_mut_color $unset_mut_color
                    { $set_rgb_mut $set_ansi_mut }
                    [$(
                        stringify!($Color) "(Color::" stringify!($variant_color) ")",
                        $set_variant_mut_color($Color(Color::$variant_color))
                    )*]
                    stringify!($Color) "(Color::" stringify!($reset_color) ")",
                    $set_reset_mut_color($Color(Color::$reset_color))
                })*
                $(($attr: $Attr) {
                    $set_mut_attr $unset_mut_attr
                    [$(
                        stringify!($Attr) "::" stringify!($variant_attr),
                        $set_variant_mut_attr($Attr::$variant_attr)
                    )*]
                    stringify!($Attr) "::" stringify!($reset_attr),
                    $set_reset_mut_attr($Attr::$reset_attr)
                })*
            );

            priv_styler!((&mut Self) (
                    $($get_color $set_mut_color($Color(Color::$reset_color)))*
                    $($get_attr  $set_mut_attr ($Attr::$reset_attr))*
                )
                style_mut and_mut or_mut xor_mut dedup_mut reset_mut
            );
        }
    };
}

macro_rules! priv_styler {
    ($($get:ident($Self:ty) -> $Output:ty)*) => {
        $($crate::doc!("Gets `" stringify!($Output) "`.",
        fn $get(self: $Self) -> $Output;);)*
    };

    (($Self:ty) -> $Output:ty;
        $(($attr:ident: $Attr:ident) {
            $set:ident $unset:ident
            $({ $set_rgb:ident $set_ansi:ident })?
            [$(
                $($doc_variant:expr)*,
                $set_variant:ident($body_variant:expr)
            )*]
            $($doc_reset:expr)*,
            $set_reset:ident($body_reset:expr)
        })*
    ) => {
        $(
            $crate::doc!("Sets `Option<" stringify!($Attr) ">`.",
            fn $set(self: $Self, $attr: impl Into<Option<$Attr>>) -> $Output;);

            $crate::doc!("`None`s `" stringify!($Attr) "`.",
            fn $unset(self: $Self) -> $Output {
                self.$set(None)
            });

            $($crate::doc!("Sets `Some(" $($doc_variant)* ")`.",
            fn $set_variant(self: $Self) -> $Output {
                self.$set(Some($body_variant))
            });)*

            $($crate::doc!("Sets `Some(" stringify!($Attr) "(Color::Rgb(r, g, b)))`.",
            fn $set_rgb(self: $Self, r: u8, g: u8, b: u8) -> $Output {
                self.$set(Some($Attr(Color::Rgb(r, g, b))))
            });)?

            $($crate::doc!("Sets `Some(" stringify!($Attr) "(Color::Ansi(ansi)))`.",
            fn $set_ansi(self: $Self, ansi: u8) -> $Output {
                self.$set(Some($Attr(Color::Ansi(ansi))))
            });)?

            $crate::doc!("Sets `Some(" $($doc_reset)* ")`.",
            fn $set_reset(self: $Self) -> $Output {
                self.$set(Some($body_reset))
            });
        )*
    };
    ((Self) ($($get:ident $set:ident($reset_expr:expr))*)
        $style:ident $and:ident $or:ident $xor:ident $dedup:ident $reset:ident
    ) => {
        /// Applies `styler`'s styles.
        fn $style(self, styler: &impl StylerIndex) -> <Self::Output as Styler>::Output
        where
            Self::Output: Styler<Output = Self::Output>
        {
            let out = self;
            $(let out = out.$set(styler.$get());)*
            out
        }

        priv_styler!((Self) and $and $($get $set)*);
        priv_styler!((Self) or  $or  $($get $set)*);
        priv_styler!((Self) xor $xor $($get $set)*);

        /// Dedups (`None`s if identicals) fields.
        fn $dedup(mut self, before: &impl StylerIndex) -> Self
        where
            Self: Styler<Output = Self>
        {
            $(if self.$get() == before.$get() {
                self = self.$set(None);
            })*
            self
        }

        /// Resets (sets to reset value) fields which are `Some`.
        fn $reset(mut self) -> Self
        where
            Self: Styler<Output = Self>
        {
            $(if let Some(_) = self.$get() {
                self = self.$set(Some($reset_expr));
            })*
            self
        }
    };
    ((Self) $op:ident $fn:ident $($get:ident $set:ident)*) => {
        doc!("`Option::" stringify!($op) "` fields.",
        fn $fn(self, other: &impl StylerIndex) -> <Self::Output as Styler>::Output
        where
            Self::Output: Styler<Output = Self::Output>
        {
            let out = self;
            $(
                let op = out.$get().$op(other.$get());
                let out = out.$set(op);
            )*
            out
        });
    };
    ((&mut Self) ($($get:ident $set:ident($reset_expr:expr))*)
        $style:ident $and:ident $or:ident $xor:ident $dedup:ident $reset:ident
    ) => {
        /// Applies `styler`'s styles.
        fn $style(&mut self, styler: &impl StylerIndex) {
            $(self.$set(styler.$get());)*
        }

        priv_styler!((&mut Self) and $and $($get $set)*);
        priv_styler!((&mut Self) or  $or  $($get $set)*);
        priv_styler!((&mut Self) xor $xor $($get $set)*);

        /// Dedups (`None`s if identicals) fields.
        fn $dedup(&mut self, before: &impl StylerIndex) {
            $(if self.$get() == before.$get() {
                self.$set(None);
            })*
        }

        /// Resets (sets to reset value) fields which are `Some`.
        fn $reset(&mut self) {
            $(if let Some(_) = self.$get() {
                self.$set(Some($reset_expr));
            })*
        }
    };
    ((&mut Self) $op:ident $fn:ident $($get:ident $set:ident)*) => {
        doc!("`Option::" stringify!($op) "` fields.",
        fn $fn(&mut self, other: &impl StylerIndex) {
            $(self.$set(self.$get().$op(other.$get()));)*
        });
    };
}
