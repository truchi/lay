mod cell;

use crate::*;

impl Generation {
    pub fn generate_layer(&self) {
        let dir = &self.layer;

        write(dir, "cell.rs", self.cell_impls());
        write(dir, "fill.rs", self.fill_impls());
        write(dir, "mod.rs", self.mod_layer());
    }

    pub fn mod_layer(&self) -> TokenStream {
        quote! {
            mod cell;
            mod fill;
            #LINE_BREAK

            pub use cell::*;
            pub use fill::*;
            #LINE_BREAK
        }
    }

    pub fn fill_impls(&self) -> TokenStream {
        let fill_impls = (&Str::new("Fill"), &[][..], &Str::new("cell"));

        concat(&[
            quote! {
                use crate::*;
                #LINE_BREAK
            },
            self.impl_styler_index(fill_impls),
            self.impl_styler(fill_impls),
            self.impl_styler_mut(fill_impls),
        ])
    }
}
