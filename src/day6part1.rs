use regex::Regex;

// 4 rows of digits then one row of symbols. I can probably just
// split things up into some vecs then do a little matching and 
// perform the right operations.


pub fn solve(content: &str) {

    let re = Regex::new(r"[ \s]+").unwrap();

    // split the input into vecs
    let mut content_vecs: Vec<Vec<&str>> = content.lines().map(|line| re.split(line.trim()).collect::<Vec<&str>>()).collect();
    let operators: Vec<&str> = content_vecs.pop().unwrap();
    let mut solution = 0;

    for (idx, operator) in operators.iter().enumerate() {

        let init = match *operator {
            "+" => 0,
            "*" => 1,
            _ => panic!("Not an operator I know what to do with")
        };

        let result = match *operator {
            "+" => content_vecs.iter().fold(init, |acc, vec| acc + vec[idx].parse::<usize>().unwrap()),
            "*" => content_vecs.iter().fold(init, |acc, vec| acc * vec[idx].parse::<usize>().unwrap()),
            _ => panic!("Not an operator I know what to do with")
        };

        solution += result
    };

    println!("solution: {}", solution);
}