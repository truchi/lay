use lay::*;

fn main() {
    println!(
        "{} {}",
        Styled::from("Hello,") * Blue / Yellow + Bold + Underline,
        Styled::from("world!") * Green / Red + Italic + Slow
    );
}
