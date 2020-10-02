macro_rules! styler {
    (
        $(#[$meta_reset:meta])*
        $Reset:ident
        Colors { $(
            $Color:ident($color:ident) $NoColor:ident {
                $get_color:ident
                $set_color:ident $set_mut_color:ident
                $unset_color:ident $unset_mut_color:ident
                Rgb: $set_rgb:ident $set_rgb_mut:ident
                Ansi: $set_ansi:ident $set_ansi_mut:ident
                $reset_color:ident: $set_reset_color:ident $set_reset_mut_color:ident
                $($variant_color:ident: $set_variant_color:ident $set_variant_mut_color:ident)*
            }
        )* }
        Attributes { $(
            $Attr:ident($attr:ident) $NoAttr:ident {
                $get_attr:ident
                $set_attr:ident $set_mut_attr:ident
                $unset_attr:ident $unset_mut_attr:ident
                $reset_attr:ident: $set_reset_attr:ident $set_reset_mut_attr:ident
                $($variant_attr:ident: $set_variant_attr:ident $set_variant_mut_attr:ident)*
            }
        )* }
    ) => {
        styler!(impl [Unit] $(#[$meta_reset])* $Reset
            Colors { $($Color $NoColor $reset_color)* }
            Attributes { $($Attr $NoAttr $reset_attr)* }
        );

        /// A trait for styled types.
        pub trait Styler: Sized {
            $(
                styler!(impl [get] $Color($color) $get_color);
                styler!(impl [set mut] $Color($color) $set_mut_color);
                styler!(impl [set] $Color($color) $set_color $set_mut_color);
                styler!(impl [unset] $Color($color) $unset_color $unset_mut_color $set_mut_color);
                $(styler!(impl [variant]
                    stringify!($Color) "(Color::" stringify!($variant_color) ")",
                    $set_variant_color $set_variant_mut_color $set_mut_color($Color(Color::$variant_color))
                );)*
                styler!(impl [rgb] $set_rgb $set_rgb_mut $set_mut_color($Color));
                styler!(impl [ansi] $set_ansi $set_ansi_mut $set_mut_color($Color));
                styler!(impl [variant]
                    stringify!($Color) "(Color::" stringify!($reset_color) ")",
                    $set_reset_color $set_reset_mut_color $set_mut_color($Color(Color::$reset_color))
                );
            )*
            $(
                styler!(impl [get] $Attr($attr) $get_attr);
                styler!(impl [set mut] $Attr($attr) $set_mut_attr);
                styler!(impl [set] $Attr($attr) $set_attr $set_mut_attr);
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

            styler!(impl [op]    and and_mut $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            styler!(impl [op]    or  or_mut  $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            styler!(impl [op]    xor xor_mut $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            styler!(impl [dedup]             $($get_color $set_mut_color)* $($get_attr $set_mut_attr)*);
            styler!(impl [reset]
                $($get_color $set_mut_color($Color(Color::$reset_color)))*
                $($get_attr  $set_mut_attr(($Attr::$reset_attr)))*
            );
            styler!(impl [fmt] $($get_color)* $($get_attr)*);
        }
    };


    (impl [Unit] $(#[$meta_reset:meta])* $Reset:ident
        Colors { $($Color:ident $NoColor:ident $reset_color:ident)* }
        Attributes { $($Attr:ident $NoAttr:ident $reset_attr:ident)* }
    ) => {
        $(styler!(impl [No] $Color $NoColor);)*
        $(styler!(impl [No] $Attr $NoAttr);)*

        $(#[$meta_reset])*
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
        pub struct $Reset;

        /// Prints the "Reset"/"Normal" csi to the terminal.
        impl Display for $Reset {
            /// Prints the "Reset"/"Normal" csi to the terminal.
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                f.write_str("\x1B[0m")
            }
        }

        styler!(impl [Reset From Color] $Reset $($reset_color)*);
        $(styler!(impl [Reset From $Color] $Reset
            stringify!($Color) "(Color::" stringify!($reset_color) ")",
            $Color(Color::$reset_color)
        );)*
        $(styler!(impl [Reset From $Attr] $Reset
            stringify!($Attr) "::" stringify!($reset_attr),
            $Attr::$reset_attr
        );)*
    };
    (impl [No] $Self:ident $No:ident) => {
        $crate::doc!("Sets `Option<" stringify!($Self) ">` to `None`.",
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
        pub struct $No;);

        $crate::doc!("Returns `None`.",
        impl From<$No> for Option<$Self> {
            $crate::doc!("Returns `None`.",
            fn from(_: $No) -> Self {
                None
            });
        });
    };
    (impl [Reset From Color] $Reset:ident $reset_color:ident $($_:ident)+) => {
        $crate::doc!("Returns `Color::" stringify!($reset_color) "`.",
        impl From<$Reset> for Color {
            $crate::doc!("Returns `Color::" stringify!($reset_color) "`.",
            fn from(_: $Reset) -> Self {
                Self::$reset_color
            });
        });
    };
    (impl [Reset From $Self:ident] $Reset:ident $($doc:expr)*, $body:expr) => {
        $crate::doc!("Returns `" $($doc)* "`.",
        impl From<$Reset> for $Self {
            $crate::doc!("Returns `" $($doc)* "`.",
            fn from(_: $Reset) -> Self {
                $body
            });
        });
        $crate::doc!("Returns `Some(" $($doc)* ")`.",
        impl From<$Reset> for Option<$Self> {
            $crate::doc!("Returns `Some(" $($doc)* ")`.",
            fn from(_: $Reset) -> Self {
                Some($body)
            });
        });
    };

    (impl [get] $Self:ident($self:ident) $get:ident) => {
        $crate::doc!("Gets `Option<" stringify!($Self) ">`.",
        fn $get(&self) -> Option<$Self>;);
    };

    (impl [set] $Self:ident($self:ident) $set:ident $set_mut:ident) => {
        $crate::doc!("Sets `Option<" stringify!($Self) ">`.",
        fn $set(mut self, $self: impl Into<Option<$Self>>) -> Self {
            self.$set_mut($self);
            self
        });
    };
    (impl [set mut] $Self:ident($self:ident) $set_mut:ident) => {
        $crate::doc!("Sets `Option<" stringify!($Self) ">` mutably.",
        fn $set_mut(&mut self, $self: impl Into<Option<$Self>>););
    };

    (impl [unset] $Self:ident($self:ident) $unset:ident $unset_mut:ident $set_mut:ident) => {
        $crate::doc!("Sets " stringify!($self) " to `None`.",
        fn $unset(mut self) -> Self {
            self.$set_mut(None);
            self
        });

        $crate::doc!("Sets " stringify!($self) " to `None` mutably.",
        fn $unset_mut(&mut self) {
            self.$set_mut(None);
        });
    };

    (impl [variant] $($doc:expr)*, $set_variant:ident $set_variant_mut:ident $set_mut:ident($body:expr)) => {
        $crate::doc!("Sets `Some(" $($doc)* ")`.",
        fn $set_variant(mut self) -> Self {
            self.$set_mut(Some($body));
            self
        });

        $crate::doc!("Sets `Some(" $($doc)* ")` mutably.",
        fn $set_variant_mut(&mut self) {
            self.$set_mut(Some($body));
        });
    };

    (impl [rgb] $set_rgb:ident $set_rgb_mut:ident $set_mut:ident($Self:ident)) => {
        $crate::doc!("Sets `Some(" stringify!($Self) "(Color::Rgb(r, g, b)))`.",
        fn $set_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
            self.$set_mut(Some($Self(Color::Rgb(r, g, b))));
            self
        });

        $crate::doc!("Sets `Some(" stringify!($Self) "(Color::Rgb(r, g, b)))` mutably.",
        fn $set_rgb_mut(&mut self, r: u8, g: u8, b: u8) {
            self.$set_mut(Some($Self(Color::Rgb(r, g, b))));
        });
    };

    (impl [ansi] $set_ansi:ident $set_ansi_mut:ident $set_mut:ident($Self:ident)) => {
        $crate::doc!("Sets `Some(" stringify!($Self) "(Color::Ansi(ansi)))`.",
        fn $set_ansi(mut self, ansi: u8) -> Self {
            self.$set_mut(Some($Self(Color::Ansi(ansi))));
            self
        });

        $crate::doc!("Sets `Some(" stringify!($Self) "(Color::Ansi(ansi)))` mutably.",
        fn $set_ansi_mut(&mut self, ansi: u8) {
            self.$set_mut(Some($Self(Color::Ansi(ansi))));
        });
    };

    (impl [op] $fn:ident $fn_mut:ident $($get:ident $set_mut:ident)*) => {
        $crate::doc!("`Option::" stringify!($fn) "` fields.",
        fn $fn(mut self, other: &impl Styler) -> Self {
            $(self.$set_mut(self.$get().$fn(other.$get()));)*
            self
        });

        $crate::doc!("`Option::" stringify!($fn) "` fields mutably.",
        fn $fn_mut(&mut self, other: &impl Styler) {
            $(self.$set_mut(self.$get().$fn(other.$get()));)*
        });
    };

    (impl [dedup] $($get:ident $set_mut:ident)*) => {
        /// Dedups (`None`s if identicals) fields.
        fn dedup(mut self, before: &impl Styler) -> Self {
            $(if self.$get() == before.$get() {
                self.$set_mut(None);
            })*
            self
        }

        /// Dedups (`None`s if identicals) fields mutably.
        fn dedup_mut(&mut self, before: &impl Styler) {
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

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn conversion() {
        // From Reset
        assert_eq!(Color::from(Reset), ResetColor);
        assert_eq!(Foreground::from(Reset), Foreground(ResetColor));
        assert_eq!(Background::from(Reset), Background(ResetColor));
        assert_eq!(Weight::from(Reset), ResetWeight);
        assert_eq!(Slant::from(Reset), ResetSlant);
        assert_eq!(Blink::from(Reset), ResetBlink);
        assert_eq!(Invert::from(Reset), ResetInvert);
        assert_eq!(Strike::from(Reset), ResetStrike);
        assert_eq!(Underline::from(Reset), ResetUnderline);
        assert_eq!(Overline::from(Reset), ResetOverline);
        assert_eq!(Border::from(Reset), ResetBorder);

        // Option From Reset
        assert_eq!(
            Option::<Foreground>::from(Reset),
            Some(Foreground(ResetColor))
        );
        assert_eq!(
            Option::<Background>::from(Reset),
            Some(Background(ResetColor))
        );
        assert_eq!(Option::<Weight>::from(Reset), Some(ResetWeight));
        assert_eq!(Option::<Slant>::from(Reset), Some(ResetSlant));
        assert_eq!(Option::<Blink>::from(Reset), Some(ResetBlink));
        assert_eq!(Option::<Invert>::from(Reset), Some(ResetInvert));
        assert_eq!(Option::<Strike>::from(Reset), Some(ResetStrike));
        assert_eq!(Option::<Underline>::from(Reset), Some(ResetUnderline));
        assert_eq!(Option::<Overline>::from(Reset), Some(ResetOverline));
        assert_eq!(Option::<Border>::from(Reset), Some(ResetBorder));

        // Option From No
        assert_eq!(Option::<Foreground>::from(NoForeground), None);
        assert_eq!(Option::<Background>::from(NoBackground), None);
        assert_eq!(Option::<Weight>::from(NoWeight), None);
        assert_eq!(Option::<Slant>::from(NoSlant), None);
        assert_eq!(Option::<Blink>::from(NoBlink), None);
        assert_eq!(Option::<Invert>::from(NoInvert), None);
        assert_eq!(Option::<Strike>::from(NoStrike), None);
        assert_eq!(Option::<Underline>::from(NoUnderline), None);
        assert_eq!(Option::<Overline>::from(NoOverline), None);
        assert_eq!(Option::<Border>::from(NoBorder), None);
    }
}
