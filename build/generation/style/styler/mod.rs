mod styler;
mod styler_index;
mod styler_index_mut;
mod styler_mut;

use crate::generation::{Lay, LINE_BREAK};

impl Lay {
    pub fn mod_styler_styler(&self) -> String {
        quote! {{
            mod styler_index;
            pub use styler_index::*;
        }}
    }
}
