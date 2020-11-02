use crate::generation::*;

impl Lay {
    pub fn attributes(&self) -> Vec<(String, TokenStream)> {
        self.attributes
            .iter()
            .map(|attr| (attr.name.to_lowercase(), attribute(attr)))
            .collect()
    }
}

fn attribute(attribute: &Attribute) -> TokenStream {
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

    let Attribute = ident!(attribute.name);
    let ResetAttribute = ident!(attribute.reset);

    quote! {
            pub use #Attribute::*;
            #LINE_BREAK

            #decl_doc
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum #Attribute {
                #(#Variant,)*
                #ResetAttribute
            }
            #LINE_BREAK

            #default_doc
            impl Default for #Attribute {
                #default_doc
                fn default() -> Self {
                    Self::#ResetAttribute
                }
            }
    }
}
