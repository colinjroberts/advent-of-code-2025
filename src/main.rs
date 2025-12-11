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
mod day5part01;
mod day5part1;
mod day5part2;
mod day6part1;
mod day6part2;
mod day7part1;

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
        "5-01" => day5part01::solve(&content),
        "5-1" => day5part1::solve(&content),
        "5-2" => day5part2::solve(&content),
        "6-1" => day6part1::solve(&content),
        "6-2" => day6part2::solve(&content),
        "7-1" => day7part1::solve(&content),
        _ => panic!("Unknown day/part")
    }
}
