use crate::*;

mod style;

impl Generation {
    pub fn generate_examples(&self, dir: &str) {
        generate_example(dir, "style", &self.example_style());
    }

    pub fn generate_docs(&self, dir: &str) {
        generate_doc(dir, "style", self.example_style());
    }
}

fn generate_example(dir: &str, file: &str, parts: &[Vec<(String, TokenStream)>]) {
    let parts = parts
        .iter()
        .map(|part| {
            let part = part.iter().map(|(comment, code)| {
                let comment = comment!("{}", comment);
                quote! { #comment #code }
            });

            quote! { #(#part)* }
        })
        .collect::<Vec<_>>();

    let data = parts.iter().enumerate().map(|(i, tokens)| {
        let name = quote::format_ident!("PART_{}", i);

        let formatted = fmt(tokens);
        let formatted = if i == 0 {
            formatted
        } else {
            format!("\n{}", formatted)
        };

        quote! { const #name: &'static str = #formatted; }
    });

    let code = parts.iter().enumerate().map(|(i, tokens)| {
        let comment = comment!(";)");
        let name = quote::format_ident!("PART_{}", i);

        quote! {
            println!("{}", #name);
            #comment #LINE_BREAK
            #tokens #LINE_BREAK
        }
    });

    let data = concat(&data.collect::<Vec<_>>());
    let code = concat(&code.collect::<Vec<_>>());

    let data_file_name = &format!("{}.data.rs", file);
    let file_name = &format!("{}.rs", file);

    write(dir, file_name, quote! {
        include!(#data_file_name);
        use lay::*; #LINE_BREAK

        pub fn main() { #code }
    });

    write(dir, data_file_name, data);
}

fn generate_doc(dir: &str, file: &str, parts: Vec<Vec<(String, TokenStream)>>) {
    let mod_name = Str::new(file);
    let file_name = &format!("{}.doc.rs", file);

    let parts = parts
        .into_iter()
        .map(|part| {
            // let part = part
            // .into_iter()
            // .map(|(comment, code)| {
            // doc!(
            // "{}```
            // # use lay::*;
            // {}
            // ```",
            // comment,
            // pre(code)
            // )
            // })
            // .map(|doc| doc)
            // .collect::<String>();
            panic!("{:#?}", part);
        })
        .collect::<Vec<_>>();

    write(dir, file_name, quote! {
        /// Lalalalla
        pub mod #mod_name;
        pub use #mod_name::*;
    });
}

/// Prints the part in pretty code frame
fn fmt(tokens: &TokenStream) -> String {
    let len = 100;

    let tokens = quote! { fn main() { #LINE_BREAK #tokens } };
    let tokens = pre(tokens);

    let formatted = format(&tokens);
    // Remove enclosing main function
    let formatted = &formatted[12..formatted.len() - 2];

    // Why? :(
    // For some reason, parts with no code blocks
    // come here with an extra opening \n...
    let formatted = if formatted.starts_with("\n") {
        &formatted[1..]
    } else {
        formatted
    };

    let formatted = formatted.lines().enumerate().map(|(i, line)| {
        // Having the tokens in a main function indent the whole thing
        // Deindent bruteforcely
        let line = if line.starts_with("    ") {
            &line[4..]
        } else {
            line
        };

        let spaces = " ".repeat(len - line.len() - 6);
        format!("│{:>width$}│ {}{}│\n", i + 1, line, spaces, width = 2)
    });

    let separator1 = "┌──┬".to_string() + &"─".repeat(len - 5) + "┐";
    let separator2 = "└──┴".to_string() + &"─".repeat(len - 5) + "┘";
    separator1 + "\n" + &formatted.collect::<String>() + &separator2
}
