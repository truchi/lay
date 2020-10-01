#[doc(hidden)]
#[macro_export]
macro_rules! doc {
    ($($doc:expr)*, $item:item) => { doc!{impl concat![" ", $($doc,)*], $item} };
    (impl $doc:expr, $item:item) => { #[doc = $doc] $item };
}
