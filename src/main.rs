use std::env;
use std::fs;

mod day1part1;
mod day1part2;
mod day2part1;
mod day2part2;
mod day3part1;
mod day3part2;
mod day4part1;
mod day4part2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Requires a day-part argument e.g. cargo run -- 1-1");
        return
    };

    let daypart: String = args[1].clone();
    let input_file: String = args[2].clone();

    // Read the file. I feel like I've been burned in the past
    // by different files needing different parsing, so just
    // read for now and let each day do its own parsing. 
    let content: String = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    // Choose the right code and run it!
    match daypart.as_str() {
        "1-1" => day1part1::solve(&content),
        "1-2" => day1part2::solve(&content),
        "2-1" => day2part1::solve(&content),
        "2-2" => day2part2::solve(&content),
        "3-1" => day3part1::solve(&content),
        "3-2" => day3part2::solve(&content),
        "4-1" => day4part1::solve(&content),
        "4-2" => day4part2::solve(&content),
        _ => panic!("Unknown day/part")
    }
}
