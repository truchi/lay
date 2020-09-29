use lay::*;

#[derive(Debug)]
struct A(i8);

impl std::ops::Not for A {
    type Output = Self;

    fn not(mut self) -> Self {
        println!("Own");
        self.0 = -self.0;
        self
    }
}

impl std::ops::Not for &mut A {
    type Output = ();

    fn not(mut self) {
        println!("mut");
        self.0 = -self.0;
    }
}

fn test<'a>() -> &'a Option<u8> {
    &None
}

fn main() {
    let a = A(1);
    let mut b = A(2);
    let c = &mut A(3);

    println!("{:?}", !a);
    println!("{:?}", !&mut b);
    println!("{:?}", b);
    println!("{:?}", !&mut *c);
    println!("{:?}", c);
    println!("{:?}", test());

    println!(
        "{}Hello, {}world{}!{}",
        Bold,
        Foreground(Red),
        Foreground(Reset),
        ResetWeight
    );
}
