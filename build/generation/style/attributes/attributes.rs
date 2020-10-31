use crate::generation::{Attribute, Lay, LINE_BREAK};

impl Lay {
    pub fn attributes(&self) -> Vec<(String, String)> {
        self.attributes
            .iter()
            .map(|attr| (attr.name.to_lowercase(), attribute(attr)))
            .collect()
    }
}

#[allow(non_snake_case)]
fn attribute(attribute: &Attribute) -> String {
    let Variant = attribute
        .variants
        .iter()
        .map(|variant| ident!("{}", variant));

    let variants: String = attribute
        .variants
        .iter()
        .map(|variant| format!("`{}`, ", variant))
        .collect();

    let decl_doc = doc!(
        "`{Attribute}` ({variants}`{ResetAttribute}`).

        Prints the corresponding CSI to the terminal when `Display`ed.
        `Default`s to `{ResetAttribute}`, the unsetting CSI.",
        Attribute = attribute.name,
        variants = variants,
        ResetAttribute = attribute.reset,
    );

    let default_doc = doc!(
        "Returns `{Attribute}::{ResetAttribute}`.",
        Attribute = attribute.name,
        ResetAttribute = attribute.reset,
    );

    quote! {
        Attribute = attribute.name,
        ResetAttribute = attribute.reset, {
            pub use #Attribute::*;

            #LINE_BREAK

            #(#[doc = #decl_doc])*
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #Attribute {
                #(#Variant,)*
                #ResetAttribute
            }

            #LINE_BREAK

            #(#[doc = #default_doc])*
            impl Default for #Attribute {
                #(#[doc = #default_doc])*
                fn default() -> Self {
                    Self::#ResetAttribute
                }
            }
    }}
}
