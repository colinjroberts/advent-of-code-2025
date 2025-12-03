// Part 2
// Similar to 1 in that we are given a range of numbers and need


// There's got to be a better way to do this along the line of just 
// calculating the number of invalid numbers based on how many integers
// there are. The first half and second half of the numbers have to be 
// able to match...or something. I'm going to brute force first.


/// checks whether a number is "invalid" meaning it is even and the
/// first half of the number matches the second half of the number
fn check_number(num: &str) -> bool {
    let num_len = num.len();

    // skip odd numbered ones since those can't be repeated sequences
    return num_len % 2 == 0 && num[0..num_len/2] == num[num_len/2..]
}

/// My original solution, before I tried to make things a little more Rusty
// pub fn solve(content: &str) {
//     let mut running_sum: u64 = 0;
//     // split the numbers
//     for s in content.split(',') {
//             let range = s.split('-').collect::<Vec<&str>>();

//             // iterate over each number in the range
//             for n in (std::ops::Range { start: range[0].parse::<u64>().unwrap(), end: range[1].parse::<u64>().unwrap() + 1 }) {

//                 // add it to our count if it is "invalid"
//                 if check_number(&n.to_string()) {
//                     running_sum += n;
//                 }
//             }
//         };

//     println!("final count: {}", running_sum);
// }

pub fn solve(content: &str) {
    let total: u64 = content
        .split(',')
        .flat_map(|range_string| {
            let (start, end) = range_string
                .split_once('-')
                .expect("Format should be start-end");

            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();

            start..=end 
        })
        .filter(|&n| check_number(&n.to_string()))
        .sum();

    println!("final count: {}", total);
}

