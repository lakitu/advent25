mod days;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day number>");
        return;
    }

    let day: u32 = args[1].parse().expect("Day must be a number");

    match day {
        1 => days::day01::main(),
        2 => days::day02::main(),
        3 => days::day03::main(),
        _ => eprintln!("Day {day} not implemented")
    }
}
