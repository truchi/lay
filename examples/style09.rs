////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

// 💡
// This example is generated from the documentation. Check it out:
// https://truchi.github.io/lay/lay/style/

fn main() {
    use lay::*;
    /// Makes a list
    fn list(title: &str, items: &[&str], selected: usize) -> String {
        let mut list = String::from(title) + ":\n";

        for (i, item) in items.iter().enumerate() {
            if i == selected {
                // Red is flashy, right?
                list = format!("{}‣ {}\n", list, Styled::from(item).red());
            } else {
                list = format!("{}• {}\n", list, Styled::from(item));
            }
        }

        list
    }

    // Print list
    println!(
        "{}",
        Styled::from(list("List", &vec!["Item", "Item", "Item"], 1))
         .red() // Red can be fancy too!
         .on_dark_yellow()
         .bold(),
    );

    // Wait, what?
}
