////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

// 💡
// This example is generated from the documentation. Check it out:
// TODO link

fn main() {
    use lay::*;
    println!("{}Red.{} Not red.", Foreground(Red), Foreground(ResetColor));
    println!(
        "{}On Green{}. Not on green.",
        Background(Green),
        Background(ResetColor)
    );
    println!("{}Bold{}. Not bold.", Bold, ResetWeight);
}
