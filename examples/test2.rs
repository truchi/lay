use crossterm::terminal::{Clear, ClearType};
use lay::*;

struct Style {
    foreground: Option<Foreground>,
}

fn main() {
    dbg!(std::mem::size_of::<u8>());
    dbg!(std::mem::size_of::<Option<u8>>());
    dbg!(std::mem::size_of::<u16>());
    dbg!(std::mem::size_of::<Option<u16>>());
    dbg!(std::mem::size_of::<u32>());
    dbg!(std::mem::size_of::<Option<u32>>());
}
