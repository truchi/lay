#![allow(unused)]

use lay::*;

const UP: &str = "\x1B[1A";
const RIGHT: &str = "\x1B[1C";
const DOWN: &str = "\x1B[1B";
const LEFT: &str = "\x1B[1D";

fn main() {
    // let style = Style::RESET;
    // let style = Style::from(Background(Red)).blink(Slow);
    // let fill = Fill::new((2, 20), ('a', style));
    // println!("{:#?}", fill);

    // let stdout = &mut std::io::stdout();
    // fill.fmt(stdout, (10, 0));
    // fill.fmt_at_cursor(stdout);
    // println!();

    // println!("12345");
    // println!("🚧aa");
    // println!("🚧{}aa", LEFT);
    // println!("🚧{}{}aa", LEFT, LEFT);

    // let canvas = Canvas::with_default(2);
    // println!("{:#?}", canvas);
    // unsafe {
    // println!("{:#?}", canvas.get_unchecked(1100));
    // }

    let mut canvas = Canvas::with_cells((3, 2), vec![
        'a'.into(),
        'b'.into(),
        'c'.into(),
        '1'.into(),
        '2'.into(),
        '3'.into(),
    ])
    .unwrap();

    for cell in canvas.cells_mut(0, 0, 3, 2) {
        dbg!(&cell);
        *cell = Cell::from('Z');
    }
    dbg!(&canvas);

    // println!("Second row");
    // for cell in canvas.row_mut(1, 0, 3) {
    // dbg!(cell);
    // }
}
