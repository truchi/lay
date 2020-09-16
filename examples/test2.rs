use lay::*;

fn main() {
    let style = Style::default() * Blue;
    let styled = Styled::new(' ', style);
    let a = <Styled<char, Style> as Styler>::get_foreground(&styled);
    dbg!(a);

    dbg!(char::default());
    dbg!(std::mem::size_of::<u8>());
    dbg!(std::mem::size_of::<Option<u8>>());
    dbg!(std::mem::size_of::<u16>());
    dbg!(std::mem::size_of::<Option<u16>>());
    dbg!(std::mem::size_of::<u32>());
    dbg!(std::mem::size_of::<Option<u32>>());
}
