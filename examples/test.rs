use lay::*;

fn main() {
    // let style = Style::RESET;
    let style = Style::from(Background(Red)).blink(Slow);
    let fill = Fill::new(2, 20, ('a', style));
    println!("{:#?}", fill);
}
