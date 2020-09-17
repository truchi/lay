use lay::*;

fn main() {
    dbg!(std::mem::size_of::<Cell>());
    dbg!(std::mem::size_of::<Option<Cell>>());
}
