macro_rules! styler {
    (
        $(#[$meta_styler:meta])*
        $Styler:ident
        $(#[$meta_styler_mut:meta])*
        $StylerMut:ident
        Colors { $(
            $Color:ident($color:ident) $NoColor:ident [$IdxColor:ident]
            $OpColor:ident($op_color:ident) $OpAssignColor:ident($op_assign_color:ident) {
                $get_color:ident $get_mut_color:ident
                $set_color:ident $set_mut_color:ident
                $unset_color:ident $unset_mut_color:ident
                $reset_color:ident: $set_reset_color:ident $set_reset_mut_color:ident
                $($variant_color:ident: $set_variant_color:ident $set_variant_mut_color:ident)*
            }
        )* }
        Attributes { $(
            $Attr:ident($attr:ident) $NoAttr:ident [$IdxAttr:ident]
            $OpAttr:ident($op_attr:ident) $OpAssignAttr:ident($op_assign_attr:ident) {
                $get_attr:ident $get_mut_attr:ident
                $set_attr:ident $set_mut_attr:ident
                $unset_attr:ident $unset_mut_attr:ident
                $reset_attr:ident: $set_reset_attr:ident $set_reset_mut_attr:ident
                $($variant_attr:ident: $set_variant_attr:ident $set_variant_mut_attr:ident)*
            }
        )* }
    ) => {
        styler!(impl [No, Idx] $($Color $NoColor $IdxColor)* $($Attr $NoAttr $IdxAttr)*);

        $(#[$meta_styler])*
        pub trait $Styler: Sized {
            $(
                styler!(impl [get]     $Color($color) $IdxColor $get_color);
                styler!(impl [set mut] $Color($color) $set_mut_color);
            )*
            $(
                styler!(impl [get]     $Attr($attr) $IdxAttr $get_attr);
                styler!(impl [set mut] $Attr($attr) $set_mut_attr);
            )*

            $(
                styler!(impl [set]   $Color($color) $set_color $set_mut_color);
                styler!(impl [unset] $Color($color) $unset_color $unset_mut_color $set_mut_color);
                $(styler!(impl [variant]
                    stringify!($Color) "(Color::" stringify!($variant_color) ")",
                    $set_variant_color $set_variant_mut_color $set_mut_color($Color(Color::$variant_color))
                );)*
                styler!(impl [variant]
                    stringify!($Color) "(Color::" stringify!($reset_color) ")",
                    $set_reset_color $set_reset_mut_color $set_mut_color($Color(Color::$reset_color))
                );
            )*
            $(
                styler!(impl [set]   $Attr($attr) $set_attr $set_mut_attr);
                styler!(impl [unset] $Attr($attr) $unset_attr $unset_mut_attr $set_mut_attr);
                $(styler!(impl [variant]
                    stringify!($Attr) "::" stringify!($variant_attr),
                    $set_variant_attr $set_variant_mut_attr $set_mut_attr($Attr::$variant_attr)
                );)*
                styler!(impl [variant]
                    stringify!($Attr) "::" stringify!($reset_attr),
                    $set_reset_attr $set_reset_mut_attr $set_mut_attr($Attr::$reset_attr)
                );
            )*

            styler!(impl [op]    $Styler and and_mut $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            styler!(impl [op]    $Styler or  or_mut  $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            styler!(impl [op]    $Styler xor xor_mut $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            styler!(impl [dedup] $Styler             $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            styler!(impl [reset]
                $($get_color $set_mut_color($Color(Color::$reset_color)))*
                $($get_attr  $set_mut_attr(($Attr::$reset_attr)))*
            );
            styler!(impl [fmt] $($get_color)* $($get_attr)*);
        }
    };

    (impl [No, Idx] $($Self:ident $No:ident $Idx:ident)*) => {
        $(
            doc!("Gets `Option<" stringify!($Self) ">`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $Idx;);

            doc!("Sets `Option<" stringify!($Self) ">` to `None`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $No;);
        )*
    };

    (impl [get] $Self:ident($self:ident) $Idx:ident $get:ident) => {
        doc!("Gets `Option<" stringify!($Self) ">`.",
        fn $get(&self) -> Option<$Self>;);
    };

    (impl [set] $Self:ident($self:ident) $set:ident $set_mut:ident) => {
        doc!("Sets `Option<" stringify!($Self) ">`.",
        fn $set(mut self, $self: impl Into<Option<$Self>>) -> Self {
            self.$set_mut($self);
            self
        });
    };
    (impl [set mut] $Self:ident($self:ident) $set_mut:ident) => {
        doc!("Sets `Option<" stringify!($Self) ">` mutably.",
        fn $set_mut(&mut self, $self: impl Into<Option<$Self>>););
    };

    (impl [unset] $Self:ident($self:ident) $unset:ident $unset_mut:ident $set_mut:ident) => {
        doc!("Sets " stringify!($self) " to `None`.",
        fn $unset(mut self) -> Self {
            self.$set_mut(None);
            self
        });

        doc!("Sets " stringify!($self) " to `None` mutably.",
        fn $unset_mut(&mut self) {
            self.$set_mut(None);
        });
    };

    (impl [variant] $($doc:expr)*, $set_variant:ident $set_variant_mut:ident $set_mut:ident($body:expr)) => {
        doc!("Sets `Some(" $($doc)* ")`.",
        fn $set_variant(mut self) -> Self {
            self.$set_mut(Some($body));
            self
        });

        doc!("Sets `Some(" $($doc)* ")` mutably.",
        fn $set_variant_mut(&mut self) {
            self.$set_mut(Some($body));
        });
    };

    (impl [op] $Styler:ident $fn:ident $fn_mut:ident $($get:ident $set_mut:ident)*) => {
        doc!("`Option::" stringify!($fn) "` fields.",
        fn $fn<T: $Styler>(mut self, other: &T) -> Self {
            $(self.$set_mut(self.$get().$fn(other.$get()));)*
            self
        });

        doc!("`Option::" stringify!($fn) "` fields mutably.",
        fn $fn_mut<T: $Styler>(&mut self, other: &T) {
            $(self.$set_mut(self.$get().$fn(other.$get()));)*
        });
    };

    (impl [dedup] $Styler:ident $($get:ident $set_mut:ident)*) => {
        /// Dedups (`None`s if identicals) fields.
        fn dedup<T: $Styler>(mut self, before: &T) -> Self {
            $(if self.$get() == before.$get() {
                self.$set_mut(None);
            })*
            self
        }

        /// Dedups (`None`s if identicals) fields mutably.
        fn dedup_mut<T: $Styler>(&mut self, before: &T) {
            $(if self.$get() == before.$get() {
                self.$set_mut(None);
            })*
        }
    };

    (impl [reset] $($get:ident $set_mut:ident($body:expr))*) => {
        /// Resets (sets to reset value) fields which are `Some`.
        fn reset(mut self) -> Self {
            $(if let Some(_) = self.$get() {
                self.$set_mut(Some($body));
            })*
            self
        }

        /// Resets (sets to reset value) fields which are `Some` mutably.
        fn reset_mut(&mut self) {
            $(if let Some(_) = self.$get() {
                self.$set_mut(Some($body));
            })*
        }
    };

    (impl [fmt] $($get:ident)*) => {
        /// Formats the CSIs of `self`'s `Some` fields.
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            $(if let Some(t) = self.$get() { t.fmt(f)?; })*
            Ok(())
        }
    }
}
