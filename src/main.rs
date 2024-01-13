use std::env;

fn main() {
    const SPACING: usize = 2;
    const BOAR: &str = include_str!("boar.txt");

    let text = {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);

        if args.len() == 0 {
            "Hello. I am the Boar.".to_string()
        } else {
            args.join(" ")
        }
    };

    let display = format!("| {text} |");
    let line_len = display.len() - SPACING;
    
    println!("");
    println!("/{}\\", "-".repeat(line_len));
    println!("{display}");
    println!("\\{}/", "-".repeat(line_len));

    let pt_start = line_len / 2;
    for i in pt_start..=(pt_start + 2) {
        println!("{}\\", " ".repeat(i))
    }

    println!("");
    println!("{BOAR}");
}
