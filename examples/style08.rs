////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

// 💡
// This example is generated from the documentation. Check it out:
// https://truchi.github.io/lay/lay/style/

fn main() {
    use lay::*;
    println!(
        "{}{}, still underlined.",
        Underlined,
        Styled::from("Red").red(),
    );
}
