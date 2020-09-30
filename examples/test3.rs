use lay::*;

fn main() {
    println!(
        "{}Hello, {}world{}!{}",
        Bold,
        Foreground(Red),
        Foreground(ResetColor),
        ResetWeight
    );

    println!("{:#?}", Style::default() * Red);
    println!("{:#?}", Style::default() * Rgb(0, 0, 0) * NoColor);
    println!("{:#?}", Style::default().foreground(Rgb(0, 0, 0)));
    println!("{:#?}", Style::default().weight(NoWeight));
}
