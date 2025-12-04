use std::collections::HashSet;

// Part 2
// Similar to 1 in that we are given a range of numbers and need
// identify numbers in that range with certain properties.
// The "invalid" numbers are now any number that is a repeating
// pattern of any length e.g. 1111, 1212, 123123123

// Instead of iterating through all of the numbers in the range
// and checking whether it matches some rules about what makes
// it invalid, I want to write code to generate all of the numbers 
// that I want within a given range.
// E.g. given a range of 0 - 100, I know that I can generate
// repeating pattern numbers. 100 is the only 3 digit number, 
// and single digit numbers definitionally can't be repeating.

// I've checked my input and I see only numbers made of 1 digit
// to 10 digits. But since a 1 digit number can't have repeats
// I only need to care about 2 digit to 10 digit numbers

// 2 digit:
// - one number repeated e.g. 22

// 3 digit:
// - one number repeated e.g. 333

// 4 digit:
// - one number repeated e.g. 4444
// - two repeating numbers e.g. 1212

// 5 digit:
// - one number repeated e.g. 55555

// 6 digit
// - one number repeated e.g. 666666
// - two repeating numbers e.g. 121212
// - three repeating numbers e.g. 123123

// 7 digit
// - one number repeated e.g. 7777777

// 8 digit
// - one number repeated e.g. 88888888
// - two repeating numbers e.g. 12121212
// - four repeating numbers e.g. 12341234

// 9 digit
// - one number repeated e.g. 999999999
// - 3 repeating numbers e.g. 123123123

// 10 digit
// - one number repeated e.g. 1111111111
// - two repeating numbers e.g. 12121212
// - five repeating numbers e.g. 1234512345

// The kinds of patterns a number can have is determined by its factors
// (not including the digit length itself). Numbers of length 
// greater than 2 will always be able to have a repeating pattern of 1 digit
// and 1 is always a factor. 

// I couldn't quite see the general solution at first, so I ended up
// implementing the 2 digit repeating pattern then the 3 digit one.
// They looked basically the same, so it was only a few tweaks to generalize.



/// Returns a vec of number ranges with start and end of the same length
/// It was easier for me to think through the solution with ranges
/// of numbers of equal length (e.g. 123-456 (3 digits), 12345-56789 
/// (5 digits)). This function is used to help when the start and end of a 
/// problem's input range are of different length (e.g. 1-234, 134-23456). 
/// It returns all ranges of the same length e.g. 4-234 returns 
//  (4,9), (10,99), (100,234).
fn number_ranges_by_length(start: &str, end: &str) -> Vec<(String,String)> {

    // First, build a vec of tuples for the ranges in each
    // power of 10. E.g. for the range 4-234, this would
    // yield [(1,9), (10,99), (100,999)]
    let mut result = vec![];
    let start_len = start.len();
    let end_len = end.len();

    for n in start_len..=end_len {
        let min = u64::pow(10, (n - 1) as u32);
        let max = u64::pow(10, n as u32) - 1;
        result.push((min.to_string(), max.to_string()));
    }

    // then update the first item in the first tuple to be the 
    // start argument e.g. yielding [(4,9), (10,99), (100,999)]
    let result_len = result.len();
    result[0] = (start.to_string(), result[0].1.clone());

    // and last item of the last tuple with the end argument
    // e.g. yielding [(4,9), (10,99), (100,234)]
    result[result_len - 1] = (result[result_len - 1].0.clone(), end.to_string());

    return result
}


/// Returns all possible patterns of repeated numbers of size n between start and end
/// E.g. given the range 200-400 (which are length 3) and an n of 1, this would generate 
/// all of the possible numbers of length 3 that are repeating patterns of 1 digit.
/// In this example, that would be 222 and 333. Patterns of n=2 look like 1212; patterns
/// n=3 look like 123123.
/// It works by first finding the smallest repeatable number of size n, then the highest.
/// Then it generates all repeatable numbers between those two.
fn generate_n_repeated_numbers(start: &str, end: &str, n: usize) -> Vec<String> {
    let number_length = start.len();

    // First find the lowest pattern (aka "invalid") number we can make

    // Split the lower bound (the start argument) into chunks of size n
    // E.g. 123456 with n=2 -> (12, 34, 56)
    // E.g. 123456 with n=3 -> (123, 456)
    let start_chars: Vec<char> = start.chars().collect();
    let start_n_digit_pairs: Vec<String> = start_chars.chunks(n).map(|pair| pair.iter().collect()).collect();

    // Find the smallest number that could be repeated and be within in range by
    // taking the lower bound (our start arguemnt) and checking each of the chunks we 
    // made above. With a number like 123456 as the lower bound and n=2, the first two 
    // digits are 12, if we repeated that we'd get 121212. But since our lower bound is
    // 123456, we can't actually use that 121212 because it is less than the lower bound.
    // The first number we'd be able to repeat with that lower bound is 13 giving 131313.
    let mut smallest_repeatable_number: u64 = start_n_digit_pairs[0].parse().unwrap();
    for c in start_n_digit_pairs {
        let i: u64 = c.parse().unwrap();

        if i < smallest_repeatable_number {
            break;
        }

        if i > smallest_repeatable_number {
            smallest_repeatable_number += 1;
            break;
        }
    }


    // Then find the highest pattern (aka "invalid") number we can make

    // Split the upper bound (the end argument) into chunks of size n
    // E.g. 123456 with n=2 -> (12, 34, 56)
    // E.g. 123456 with n=3 -> (123, 456)
    let end_chars: Vec<char> = end.chars().collect();
    let end_n_digit_pairs: Vec<String> = end_chars.chunks(n).map(|pair| pair.iter().collect()).collect();
    
    // Find the highest number we can possibly make given the upper bound of "end" and
    // the type of pattern we are repeating. E.g. with an end of 456789 and an n of 2
    // the highest 2 digit repeating pattern is 454545 (as the next highest of 464646)
    // would be out of range.
    let mut highest_repeatable_number: u64 = end_n_digit_pairs[0].parse().unwrap();
    for c in end_n_digit_pairs {
        let i: u64 = c.parse().unwrap();

        // if the current number is bigger than the last number we checked, stop.
        // In the 456789 example, we have 45, 67, 89. 67 is greater than 45 
        // which means 454545 will be the highest repeated number we can make.
        if i > highest_repeatable_number {
            break;
        }
        
        // if it is the same, we need to keep counting
        if i == highest_repeatable_number {
            continue
        }

        // if it is less, then the highest number is 1 minus
        // E.g. with 654321, we have 65, 43, 21. We start with 65, then
        // compare 43 to it. Since 43 is smaller, the highest repeatble 
        // pattern we could make with a max of 654321 is 646464.
        else {
            highest_repeatable_number -= 1;
            break;
        }
    }

    // Now that we know the smallest repeatble number and the highest one,
    // we can generate all possible repeated numbers.
    // E.g. if we are making numbers of length 4 and n of 2 where the smallest
    // repeatble number is 14 and the highest is 17, we will make the following:
    // 1414, 1515, 1616, and 1717
    let mut result = Vec::<String>::new();
    for val in smallest_repeatable_number..=highest_repeatable_number {
        result.push(val.to_string().repeat(number_length / n));
    }

    result
}

/// For a given range (where start and end are of the same length),
/// generate all of the possible "invalid" numbers (those made of
/// repeating patterns). This is method maps the length of numbers
/// in the range to the kinds of patterns we can generate. See the
/// long comment at the top of the file for an explanation. There
/// is probably a more generalize way to do this, but I like seeing 
/// it here in its expanded form.
fn generate_all_numbers(start: String, end: String) -> Vec<String> {
    let range_length = start.len();

    // A hash is necessary so that we don't get repeats e.g. for 8-digit
    // numbers, many of the 2-digit and 4 digit patterns will be the same
    // E.g. In 11111111 - 22222222, will have 2 digit patterns like
    // 11_11_11_11, 12_12_12_12, and 4 digit patterns like
    // 1111_1111 and 1234_1234. We don't want to double count 11111111.
    let mut output = HashSet::new();

    match range_length {
        1 => {
            // digits of length 1 can't have a repeated pattern
        }, 
        2 => {
            // println!("matching for length 2");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
        },
        3 => { 
            // println!("matching for length 3");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
        },
        4 => { 
            // println!("matching for length 4");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
            output.extend(generate_n_repeated_numbers(&start, &end, 2));
        },
        5 => { 
            // println!("matching for length 5");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
        },
        6 => { 
            // println!("matching for length 6");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
            output.extend(generate_n_repeated_numbers(&start, &end, 2));
            output.extend(generate_n_repeated_numbers(&start, &end, 3));

        },
        7 => { 
            // println!("matching for length 7");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
        },
        8 => { 
            // println!("matching for length 8");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
            output.extend(generate_n_repeated_numbers(&start, &end, 2));
            output.extend(generate_n_repeated_numbers(&start, &end, 4));

        },
        9 => { 
            // println!("matching for length 9");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
            output.extend(generate_n_repeated_numbers(&start, &end, 3));
        },
        10 => { 
            // println!("matching for length 10");
            output.extend(generate_n_repeated_numbers(&start, &end, 1));
            output.extend(generate_n_repeated_numbers(&start, &end, 2));
            output.extend(generate_n_repeated_numbers(&start, &end, 5));

        },
        _ => println!("Length needs to be between 1 and 10")
    }

    return output.into_iter().collect()
}


/// Take the input content, break it up into its ranges (split and split_once)
/// for each range provided by the input text, break that up into ranges by size (number_ranges_by_length)
/// for each of the ranges of same size, generate all possible number patterns (generate_all_numbers)
/// Then sum up all of the generated numbers.
pub fn solve(content: &str) {

    // Does most of the work to transform the ranges into number patterns ("invalid numbers")
    let invalid_numbers: Vec<String> = content.split(',')
        .map(|content_range| content_range.split_once('-').expect("Format should be start-end"))
        .flat_map(|pair| number_ranges_by_length(pair.0, pair.1))
        .flat_map(|sub_range| generate_all_numbers(sub_range.0, sub_range.1))
        .collect();

    // Take the numbers we generated as strings, converts them to integers, and sums them
    println!("final sum: {}", invalid_numbers.into_iter().map(|i| i.parse::<u64>().unwrap()).sum::<u64>())
}

