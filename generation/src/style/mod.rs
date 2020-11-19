mod attributes;
mod backends;
mod color;
mod markers;
mod reset;
mod style;
mod styler;

use crate::*;

impl Generation {
    pub fn generate_style(&self, dir: &str) {
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
        write(dir, "backends/mod.rs", quote! {
            #[cfg(feature = "backend-crossterm")]
            mod crossterm;
        });

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
        write(dir, &format!("{}.rs", self.index.snake), self.i());
        write(dir, &format!("{}.rs", self.none.snake), self.no());
        write(dir, "style.rs", self.style());
        let styled_impls = (
            &Str::new("Styled<T>"),
            &[quote! { T: Display }][..],
            &Str::new("style"),
        );
        write(
            dir,
            "styled_impls.rs",
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
            ]),
        );
        write(
            dir,
            "mod.rs",
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
                self.import_markers(),
            ]),
        );
    }
}
