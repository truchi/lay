mod attributes;
mod backends;
mod color;
mod no;
mod reset;
mod style;
mod styler;

use crate::*;

impl Generation {
    pub fn generate_style(&self) {
        let dir = &self.style;

        // dir/attributes/
        for ground in &self.grounds {
            write(
                dir,
                &format!("attributes/{}.rs", ground.snake),
                self.ground(ground),
            );
        }
        for attribute in &self.attributes {
            write(
                dir,
                &format!("attributes/{}.rs", attribute.snake),
                self.attr(attribute),
            );
        }
        write(
            dir,
            &format!("attributes/mod.rs"),
            self.mod_style_attributes(),
        );

        // dir/backends/
        write(dir, "backends/crossterm.rs", self.backend_crossterm());
        write(dir, "backends/mod.rs", self.mod_style_backends());

        // dir/styler/
        let (styler_index, styler_index_mut) = self.styler_index();
        let (styler, styler_mut) = self.styler();
        write(dir, "styler/styler_index.rs", styler_index);
        write(dir, "styler/styler_index_mut.rs", styler_index_mut);
        write(dir, "styler/styler.rs", styler);
        write(dir, "styler/styler_mut.rs", styler_mut);
        write(dir, "styler/mod.rs", self.mod_style_styler());

        // dir/
        write(dir, &format!("{}.rs", self.color.snake), self.color());
        write(dir, &format!("{}.rs", self.reset.snake), self.reset());
        write(dir, &format!("{}.rs", self.none.snake), self.no());
        write(dir, "style.rs", self.style());
        write(dir, "styled_impls.rs", self.styled_impls());
        write(dir, "mod.rs", self.mod_style());
    }

    pub fn mod_style(&self) -> TokenStream {
        concat(&[
            quote! {
                pub mod attributes;
                mod backends;
                mod color;
                mod reset;
                mod style;
                mod styled_impls;
                mod styler;
                #LINE_BREAK

                pub use attributes::*;
                pub use color::*;
                pub use reset::*;
                pub use style::*;
                pub use styler::*;
                #LINE_BREAK
            },
            self.import_no(),
        ])
    }

    pub fn styled_impls(&self) -> TokenStream {
        let styled_impls = (
            &Str::new("Styled<T>"),
            &[quote! { T: Display }][..],
            &Str::new("style"),
        );

        concat(&[
            quote! {
                use crate::*;
                use std::fmt::Display;
                // FIXME
                use std::ops::Not;
                #LINE_BREAK
            },
            self.impl_styler_index(styled_impls),
            self.impl_styler_index_mut(styled_impls),
            self.impl_styler(styled_impls),
            self.impl_styler_mut(styled_impls),
            self.impl_styler_ops(styled_impls, false),
            self.impl_styler_ops(styled_impls, true),
        ])
    }
}
