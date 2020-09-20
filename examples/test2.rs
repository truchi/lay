use crossterm::terminal::{size, Clear, ClearType};
use lay::*;

fn main() {
    println!(":{}: {}", '\u{2003}', "EM");
    println!(":{}: {}", '\u{3000}', "IDEOGRAPHIC");
    println!(":🦀:");
    println!(":{}: {}", '\u{2007}', "FIGURE");
    println!(":{}: {}", '\u{2002}', "EN");
    println!(":{}: {}", '\u{2008}', "PUNCTUATION");
    println!(":{}: {}", '\u{2004}', "3 PER EM");
    println!(":{}: {}", '\u{2005}', "4 PER EM");
    println!(":{}: {}", '\u{2006}', "6 PER EM");
    println!(":{}: {}", ' ', "NORMAL");
}
