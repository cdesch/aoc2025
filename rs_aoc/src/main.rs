use rs_aoc::days;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        eprintln!("       cargo run -- all");
        eprintln!("Example: cargo run -- 1");
        std::process::exit(1);
    }

    if args[1] == "all" {
        for day in 1..=25 {
            days::run_day(day);
        }
    } else {
        let day: u32 = args[1]
            .parse()
            .expect("Day must be a number between 1 and 25");
        if !(1..=25).contains(&day) {
            eprintln!("Day must be between 1 and 25.");
            std::process::exit(1);
        }
        days::run_day(day);
    }
}
