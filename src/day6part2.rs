
// 4 rows of digits then one row of symbols. I can probably just
// split things up into some vecs, mix things around, then perform
//  the right operations... you know, draw the rest of the owl!


/// Taking a matrix, return a matrix of new numbers
/// New numbers consist of all of the same digits found in the input
/// vec, but they have been rearranged. Imagine lining all of the numbers
/// up vertically and left aligned such that each digit is in a column.
/// Starting at the right most column, scan down the list top to bottom.
/// Build a new number left to right using the digits found in that column.
/// [[123, 45, 6, 1],
///  [1, 23, 456, 7890]]
/// becomes
/// [[3, 25, 1461],
///  [0, 69, 258, 1247]]
pub fn decephalapodize(input: Vec<Vec<&str>>) -> Vec<Vec<usize>> {
    let height = input.len();
    let width = input[0].len();

    let mut results: Vec<Vec<usize>> = vec![];
    let mut output_col = 0;

    
    for w in (0..width).rev() {
        let mut new_num: Vec<&str> = vec![];

        if results.len() <= output_col {
            results.push(vec![])
        }

        for h in 0..height {
            new_num.push(input[h][w]);
        };

        let digit = new_num.join("").trim().to_string();

        match digit.as_str() {
            "" => {
                output_col += 1;
            }
            _ => {
                results[output_col].push(digit.parse::<usize>().unwrap());

            }
        }
    }

    results
}


pub fn solve(content: &str) {

    // split the input into vecs. The main difference from part 1 is that
    // I'm splitting each row into all of its characters and not trimming or
    // joining since the spaces are significant here. I did a classic not read
    // the problem fully at first and went down the wrong path for a minute by
    // wanting to transpose the table, when in reality, the numbers are already 
    // all nice and lined up for me! 
    // E.g. the test data 
    //   123 328  51 64 
    //    45 64  387 23 
    //     6 98  215 314
    //   *   +   *   +  
    // 
    // gets transformed into
    // ["1", "2", "3", " ", "3", "2", "8", " ", " ", "5", "1", " ", "6", "4", " "]
    // [" ", "4", "5", " ", "6", "4", " ", " ", "3", "8", "7", " ", "2", "3", " "]
    // [" ", " ", "6", " ", "9", "8", " ", " ", "2", "1", "5", " ", "3", "1", "4"]
    // ["*", " ", " ", " ", "+", " ", " ", " ", "*", " ", " ", " ", "+", " ", " "]
    let mut content_vecs: Vec<Vec<&str>> = content.lines().map(|line| line.split("").filter(|s| s != &"").collect::<Vec<&str>>()).collect();

    // the final row in this parsed data is operators, so pop that out and 
    // while we are at it, get rid of the spaces since we don't need them later
    // ["*", " ", " ", " ", "+", " ", " ", " ", "*", " ", " ", " ", "+", " ", " "]
    // becomes ["*", "+", "*", "+"]
    let operators: Vec<&str> = content_vecs.pop().unwrap().into_iter().filter(|s| *s == "*" || *s == "+").collect();
    
    // Move all the numbers around so that we have all the numbers we need to crunch
    // together AND transpose into the same vec reading right to left
    // ["1", "2", "3", " ", "3", "2", "8", " ", " ", "5", "1", " ", "6", "4", " "]
    // [" ", "4", "5", " ", "6", "4", " ", " ", "3", "8", "7", " ", "2", "3", " "]
    // [" ", " ", "6", " ", "9", "8", " ", " ", "2", "1", "5", " ", "3", "1", "4"]
    // ["*", " ", " ", " ", "+", " ", " ", " ", "*", " ", " ", " ", "+", " ", " "]
    //
    // becomes
    // [[4, 431, 623], 
    //  [175, 581, 32], 
    //  [8, 248, 369], 
    //  [356, 24, 1]]
    // AND they are actual numbers rather than strings
    let cephalopod_numbers: Vec<Vec<usize>> = decephalapodize(content_vecs);    

    // Now that our data is in the right shape, we will iterate over
    // each operator we have and use them on the data. 
    // operators: ["*", "+", "*", "+"]
    // 
    // cephalopod_numbers:
    // [[4, 431, 623], 
    //  [175, 581, 32], 
    //  [8, 248, 369], 
    //  [356, 24, 1]]
    // so as we iterate over the operators, we can use the index to select
    // the correct vec of cephalopod_numbers to us it on
    let mut solution = 0;
    for (idx, operator) in operators.iter().rev().enumerate() {

        // Set our accumulator to be neutral depending on the operator
        let init = match *operator {
            "+" => 0,
            "*" => 1,
            _ => panic!("Not an operator I know what to do with")
        };

        // Compared to part 1, all of the numbers we want are already in the same
        // vec, so we can add/multiply all those numbers and add it to 
        // our running solution
        let result = match *operator {
            "+" => cephalopod_numbers[idx].iter().fold(init, |acc, num| acc + num),
            "*" => cephalopod_numbers[idx].iter().fold(init, |acc, num| acc * num),
            _ => panic!("Not an operator I know what to do with")
        };

        solution += result
    };


    println!("solution: {:?}", solution);
}