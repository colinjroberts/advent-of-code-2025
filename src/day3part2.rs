
// Given a string of digits, return the 12 highest digits in the order they are found e.g. 
// 987654321111111 = 987654321111
// 811111111111119 = 811111111119
// 234234234234278 = 434234234278
// 818181911112111 = 888911112111
// 
// I spent a very long time thining about this one trying to find a better solution
// than what I had for part 1. At some point I started thinking about the problem 
// in terms of ranges. The problem asks us to build a number with 12 digits in it,
// but let's pick a smaller number for easier thinking. Let's say I need to make a
// number made up of 4 digits.
//   - I know that the first digit of my 4 digit number is the most significant. The
//     higher I can make that first digit, the better.
//   - I also know that since it must be 4 digit number, my options for the first digit
//     are the first number through to the 4th to last number (in 123865792) that's 123865.
//     If I picked 9 as my first digit, I wouldn't have enough numbers left in the 
//     sequence to make a 4 digit number.
// Given that, there is a cleaner maybe easier to understand brute force solution.
// o = length of the output I need to make
// i = length of the input
//   - Find the highest digit in the input between 0 and i - o (i.e. the example above
//     has input of length 9 and output length 4 so I'm looking between indexes 0 and 5
//     the whole input was 123865792, so the subrange is 123865).
//     Save the index (3) of that highest digit (8), let's call it x. 
//   - Next, find the highest digit in the input between x+1 and i - o + 1. Put another
//     way, we are now looking for the highest digit to make the second digit in our 5
//     digit output. Since we already have a first digit, we have to start searching 
//     after that one, and we know that the last digit from our input we could possibly
//     use is the 3rd to last one because we are making a 4 digit number. x+1 is index 4
//     and i - o + 1 = 9 - 4 + 1 = 6. The input is 123865792, so the range between indexes
//     4 and 6 is 67. 7 is the highest digit, so we save that index and repeat.

// How efficient is that in the worst case? With an input like 987654321, we are going to
// have to check basically the whole input for each output digit we need. with o for 
// output length and i for input length, we need to check all of i for each o: i*o
// When o is small, we don't need to scan i many times. When o is large (approaching i), 
// the range we search each time is small. E.g. if the input is 100 characters long and 
// the output is 99, then when looking for the first digit, we only have 2 options: the
// first number and the second.

// It really really feels like there is a data structure that would make this algo 
// more efficient. After some searching I think that is a segment tree 
// https://cp-algorithms.com/data_structures/segment_tree.html
// I know it will take O(i log i) to build the tree, then finding the max of a given 
// range should be O(log i) which we need to do o times. So I think overall that will 
// be O(i * log i) + O(o * log i). I'm no expert on this stuff, but if that is right 
// then the dominant one is whichever is bigger, which in this case is i, I guess.
// In any case, I'm behind already, so I'll just implement my scanning algo.

// Oh and I guess a sparse table would work as well and have better retrieval. n log n
// to build and constant time to retreive, but takes n log n space as well.


// Let's generalize the solution. I'd like to see if this works for part 1 as well.
const SOLUTION_LENGTH: usize = 2;

/// Scan the input o times where o is the length of the output.
/// There are definitely some optimizations I could work into here, but
/// for now I just want to see what a straightforward algo looks like
fn find_highest_numbers(line: &str) -> String {

    let input_vec = line.chars().map(|c| c.to_digit(10).unwrap() as i8).collect::<Vec<i8>>();
    let mut output_vec: Vec<i8> = vec![];

    // For each slot in the output we need to build
    for output_index in 0..SOLUTION_LENGTH {
        
        // determine the segment of the array we are searching

        // start: if this is the first loop, it should be 0, 
        // otherwise it should be one more than the most 
        // recently saved index plus 1. This is some Rust
        // nonsense that basically says output_vec.last()
        // has a value, return it plus one, otherwise return 0
        let start: i8 = match output_vec.last() {
            Some(&x) => x + 1,
            None => 0,
        };

        // end: for whichever digit of our output we are 
        // looking for, we need to make sure that we choose 
        // a digit from the input and leave enough room to 
        // fill the rest of our output. That's going to be
        // the length of the input - the length of the solution
        // plus which digit of the solution we are trying to fill
        let end: i8 = (input_vec.len() - SOLUTION_LENGTH + output_index).try_into().unwrap();

        // Now, find the index of the highest digit in this range. 
        // Since 9 is the highest possible digit, one could optimize
        // here to return early on a 9.
        // highest_number_tuple is a tuple of (index, value) 
        let mut highest_number_tuple = (-1, -1);
        for input_index in start..=end {
            if input_vec[input_index as usize] > highest_number_tuple.1 {
                highest_number_tuple = (input_index, input_vec[input_index as usize]);
            }
        }

        // Now that we have the highest digit, save the index in 
        // the output and start again!
        output_vec.push(highest_number_tuple.0);
    }

    // At this point, we should have completely filled the output vec with the 
    // indexes of the numbers we want to save, so we need to extract those. Then 
    // because the next step is to turn these digits into a large number, we need
    // to convert it from a vec of ints back into a string so that string can be
    // turned into a huge number. This is some Rust nonsense that does that all
    // at once and implicitly returns.
    output_vec.into_iter().map(|idx| input_vec[idx as usize].to_string()).collect()
}

pub fn solve(content: &str) {

    let joltages: Vec<String> = content.lines()
        .map(|line| find_highest_numbers(line))
        .collect();

    let solution: i64 = joltages.into_iter().map(|joltage| joltage.parse::<i64>().unwrap()).sum();
    
    println!("solution: {}", solution);
}