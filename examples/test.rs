use crossterm::terminal::{Clear, ClearType};
use lay::*;

fn main() {
    // println!(
    // "{} {}",
    // Styled::from("Hello,") * Blue / Yellow + Bold + Underline,
    // Styled::from("world!") * Green / Red + Italic + Slow
    // );

    let mut canvas = Canvas::new(30, 30);
    canvas.above(&(Styled::from('A') * Red / Green + Bold + Underline), 2, 2);
    canvas.below(&(Fill::from((' ', 20, 10)) / Yellow), 1, 1);
    canvas.above(&(Fill::from(('a', 2, 2)) * Red / Black), 5, 5);
    canvas.above(&(Styled::from("LOL") * Red / Magenta + Bold), 3, 0);
    // dbg!(canvas);
    let render = Render {
        layer:  &canvas,
        width:  0,
        height: 0,
        x:      0,
        y:      0,
    };
    // dbg!(render);
    let render2 = Render {
        layer:  &render,
        width:  30,
        height: 30,
        x:      0,
        y:      0,
    };

    println!("{}", Clear(ClearType::All));
    println!("{}", render2);
}
