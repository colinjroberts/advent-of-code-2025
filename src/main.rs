use std::env;

mod day1part1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Requires a day-part argument e.g. cargo run -- 1-1");
        return
    };

    match &args[2] {
        String::from("1-1") => day1part1::solve(&args[2]),
        _ => panic!("Unknown day/part")
    }
}
