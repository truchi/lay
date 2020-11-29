#![allow(unused)]

use lay::*;

const UP: &str = "\x1B[1A";
const RIGHT: &str = "\x1B[1C";
const DOWN: &str = "\x1B[1B";
const LEFT: &str = "\x1B[1D";

fn main() {
    // let style = Style::RESET;
    let style = Style::from(Background(Red)).blink(Slow);
    let fill = Fill::new((2, 20), ('a', style));
    println!("{:#?}", fill);

    println!("12345");
    println!("🚧aa");
    println!("🚧{}aa", LEFT);
    println!("🚧{}{}aa", LEFT, LEFT);
}
