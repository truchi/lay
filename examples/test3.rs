use lay::*;

fn main() {
    println!(
        "{}Hello, {}world{}!{}",
        Bold,
        Foreground(Red),
        Foreground(Reset),
        ResetWeight
    );
}
