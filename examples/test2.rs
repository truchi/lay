// use lay::*;

fn main() {
    dbg!(char::default().is_whitespace());
    dbg!(std::mem::size_of::<u8>());
    dbg!(std::mem::size_of::<Option<u8>>());
    dbg!(std::mem::size_of::<u16>());
    dbg!(std::mem::size_of::<Option<u16>>());
    dbg!(std::mem::size_of::<u32>());
    dbg!(std::mem::size_of::<Option<u32>>());
}
