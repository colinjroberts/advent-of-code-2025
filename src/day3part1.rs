
// Given a string of digits, return the two highest digits in the order they are found
// e.g. 12345 = 45, 98765 = 98, 10001 = 11, 11111 = 11;
//
// I'd like to think I could do this in a single pass with two variables tracking the
// left and right digits. 
// Other possible approaches:
//   - There is a straightforward approach scanning the list twice:
//     Scan it once from the first digit to the second to last and save the index of the 
//     highest number (to be the left digit of our solution). Then scan again starting 
//     just after the one we found through the to the end of the list, looking the highest
//     digit (to be the right digit).
// 
// Thoughts about the Plan:
// Generally speaking the algo looks like this:
// - Save a left and right digit using the first two numbers in the list
// - Start on the second number in the list and scan the list through the second to last number
// - At each step, if the current digit is higher than our left number, update it.
// - Otherwise, if the current digit is higher than our right number, update that.
// - At the end of the list, check to see if the very last number is higher than our right number.
// 
// There's a little more nuance
// The first possible number we could use are the first two numbers in the sequence
// E.g. The first two digits in 78679123 are 78.
// Scan through the list looking at each number one at a time. If the current digit is
// higher than our left most number (7), update it to be that new higher number, otherwise, 
// we want to check the right number. Why? Because our left digit is in the 10s place, we want
// to prioritize making that as high as possible even if it means we will get a lower right digit.
// E.g. in 78679123, using 9 as our left digit will give us numbers in the 90s even though
// the only options for right digits are 1, 2, and 3. As long as that left digit doesn't need
// updating, we can check whether the right number does.
//
// Why start the scan from the second digit?
// Sometimes, the first number in the list is lower then the second number in the list. When this 
// situation happens, we have initialized our list with a right digit that is higher than our left
// digit and we need to make sure to handle that. Starting our checking algo from that second number
// in the list means we will update the left number.
// An example! I'll represent the current item we are looking at (cur) with ^. We start
// by seetting left to the first digit (7) and right to the second digit (8). Then start our
// algo from that second number.
// 78679123
//  ^
// cur:8, left:7, right:8 
// Right from the get go, we know we want the left digit to change to 8 because that will give
// us higher numbers. So any time(*) we find a digit higher than our last number, we should replace
// left with that new higher digit. Then, once we've updated left, we need to update right. Using
// the same thought as when we started, the next available option for right is to use the number
// after our current number (in this case 6). So at the end of the first step, our state looks like:
// 78679123
//  ^
// cur:8, left:8, right:6
// But why the (*) above? Can we really update our left digit ANY TIME we find a new higher digit?
// There is one exception: the last number in the list. The very last number in our list can ONLY
// be a right digit, so we need to make sure that we handle that exception.
// Let's take another step in the example with the algo:
// 78679123
//   ^
// cur:6, left:8, right:6
// We are not on the last item in the list, so our logic still applies.
// cur is not greater than left, so we don't need to update it. Next we check right. In this case, 
// they are the same, so we shouldn't do anything. Next number!
// 78679123
//    ^
// cur:7, left:8, right:6
// We are not on the last item in the list, so our logic still applies.
// cur is not greater than left, so we don't need to update it. Next we check right. In this case, 
// cur is greater than right, so we update it.
// 78679123
//    ^
// cur:7, left:8, right:7
// Let's do one more step.
// We are not on the last item in the list, so our logic still applies.
// cur is greater than left, so we update it and set right to be the next number in the list.
// 78679123
//     ^
// cur:9, left:9, right:1
// The process continues until the last step. Skipping ahead...
// 78679123
//        ^
// cur:3, left:9, right:2
// This is a special case we need to handle. When on the last number, we should only check
// and possibly update right. Which in this case we do!
// 78679123
//        ^
// cur:3, left:9, right:3
// Our final number is 93


/// Scan the list of digits to make the highest 2-digit number keeping the order of 
/// the digits intact.
fn find_highest_numbers(line: &str) -> String {

    // convert our input string into an iterator of chars that is peekable
    // (meaning I can look at the next item in the list without moving the
    // iterator to it)
    let mut chars = line.chars().peekable();

    // Set left to be the first number in the list. Only left could be this number, so 
    // we can consume it and process the iterator.
    let mut left: i32 = chars.next().unwrap().to_digit(10).unwrap().try_into().unwrap();

    // Set right, but don't consume it. We want to make sure that if there are more 
    // numbers in the sequence and this number is high, we can use it as our left digit.
    let mut right: i32 = chars.peek().unwrap().to_digit(10).unwrap().try_into().unwrap();

    // Now progress the iterator to the 2nd item in the list (aka the next item available 
    // since we've already consumed the first one).
    let mut cur = chars.next();

    // While there are numbers to look at...
    while cur.is_some() {
        
        // Extract the current digit and convert it to an int
        let c_int: i32 = cur.unwrap().to_digit(10).unwrap().try_into().unwrap();
        
        // while we aren't at the second to last number yet
        if chars.peek().is_some() {
            // if this current number is bigger than our current highest left digit
            // replace it with this new higher digit and set right to be the next 
            // number in the sequence by peeking.
            if c_int > left {
                left = c_int as i32;
                right = chars.peek().unwrap().to_digit(10).unwrap().try_into().unwrap();
            } 

            // if it isn't bigger than left, check right
            else if c_int > right {
                right = c_int as i32;
            }
        } 
        // if we are on the last number
        else {
            // only check the right digit
            if c_int > right {
                right = c_int as i32;
            }
        }

        cur = chars.next();
    }

    let output = left.to_string() + &right.to_string();
    output
}

pub fn solve(content: &str) {
    let joltages: Vec<String> = content.lines()
    .map(|line| find_highest_numbers(line))
    .collect();

    let solution: i64 = joltages.into_iter().map(|joltage| joltage.parse::<i64>().unwrap()).sum();
    println!("solution: {}", solution);
}