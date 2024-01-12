use std::env;
use std::fs;
use std::process;

fn main() {
    let res = fs::read_to_string("src/boar.txt");

    if let Ok(boar) = res {
        const SPACING: usize = 2;

        let text = {
            let mut args: Vec<String> = env::args().collect();
            args.remove(0);
            args.join(" ")
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
        println!("{boar}");
    } else {
        eprintln!("the boar is a lie");
        process::exit(1);
    }
}
