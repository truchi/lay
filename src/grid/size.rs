use super::*;

macro_rules! coords {
    ($($(#[$TMeta:meta])* $type:ident: $Type:ident $(#[$XMeta:meta])* $x:ident $(#[$YMeta:meta])* $y:ident)*) => { $(
        $(#[$TMeta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct $Type<T> {
            $(#[$XMeta])*
            pub $x: T,
            $(#[$YMeta])*
            pub $y: T,
        }

        /// ### Methods
        impl<T> $Type<T> {
            /// Converts to another unit.
            pub fn to<U>(&self) -> $Type<U>
            where
                T: Into<U> + Clone,
            {
                let ($x, $y) = self.clone().into();
                $Type { $x: $x.into(), $y: $y.into() }
            }
        }

        impl<T: Clone> From<T> for $Type<T> {
            fn from(value: T) -> Self {
                Self {
                    $x: value.clone(),
                    $y: value,
                }
            }
        }

        impl<T> From<(T, T)> for $Type<T> {
            fn from(($x, $y): (T, T)) -> Self {
                Self { $x, $y }
            }
        }

        impl<T> From<$Type<T>> for (T, T) {
            fn from($type: $Type<T>) -> Self {
                ($type.$x, $type.$y)
            }
        }

        impl<T: Clone> Coord<T> for $Type<T> {
            type Checked = $Type<Option<T>>;
        }
    )* };
}

coords!(
    /// A `x`, `y` [`Point`](crate::Point).
    point: Point
        /// X axis `x` component.
        x
        /// Y axis `y` component.
        y
    /// A `width`, `height` [`Size`](crate::Size).
    size: Size
        /// X axis `width` component.
        width
        /// Y axis `height` component.
        height
);
