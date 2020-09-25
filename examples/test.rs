use crossterm::terminal::{Clear, ClearType};
use lay::*;

fn main() {
    // println!(
    // "{} {}",
    // Styled::from("Hello,") * Blue / Yellow + Bold + Underline,
    // Styled::from("world!") * Green / Red + Italic + Slow
    // );

    let mut canvas = Canvas::new(30, 30);
    canvas >>= (
        (Cell::from('A') * Red / Green + Bold + Underline + Italic + Slow),
        2,
        2,
    );
    canvas <<= ((Fill::from((' ', 20, 10)) / Yellow), 1, 1);
    canvas >>= ((Fill::from(('a', 2, 2)) * Red / Black), 5, 5);
    canvas >>= (&mut (Fill::from(('b', 2, 2)) * Green / Red), 7, 7);
    canvas >>= ((Styled::from("LOL") * Red / Magenta + Bold), 3, 0);
    let render = Render::new(canvas, 0, 0);
    let render2 = Render::new(&render, 10, 10);

    println!("{}", Clear(ClearType::All));
    println!("{}", render2);
}
