// Same as part 1, but now we need to make a note every time
// the dial ever pointed at 0, not just if it stopped there
// after a turn.

// My first thought was that instead of incrementing password_count
// after resolving a turn instruction, pull the count up into the
// while loop that handles the number adjustment if the value is >99
// or <0. How convenient that I decided to do it the way I did! 
// But doing only that ended up giving a too high number. It turns
// out that I was missing 2 important edge cases: when turning left,
// if we landed exactly on 0 or if we start on 0.

// Alternate approaches I considered or played around with:
//   - literally increment the position value for every 
//     "tick" of the turn. Real brute force. 
//   - more math! For a turn, add the total e.g. R100 == 50 + 100
//     150 then divide by 100 to get the # of times we passed 0
//     and mod 100 to get the new number.

/// A representation of both the current position of the dial
struct DialValue {
    position: i32,
    password_count: i32
}

/// Define how to turn right and left on the dial and
/// keep track of when the dial points at 0.
impl DialValue {

    // Add the new value to the current position. It can only
    // go up to 99, but the instructions could give us a 
    // crazy number like 250. If that were our first input,
    // we'd have 300 which is off the charts, so we need to subtrct
    // 99 until we are in range for the dial.
    fn increment(&mut self, amount: i32) {
        self.position += amount;

        while self.position > 99 {
            self.position -= 100;
            self.password_count += 1;
        }
    }

    // Same idea, but down.
    // If we turn left and land on 0, make sure we count it.
    // If you start at 0 then overflow left, we shouldn't count it.
    fn decrement(&mut self, amount: i32) {
        // if we are decrementing starting from 0, 
        // we won't pass it again, but the approach below will
        // over count by one, so subtract one to keep it even
        if self.position == 0 {
            self.password_count -= 1;
        }
        
        self.position -= amount;

        // each time we've over shot 0, count it
        while self.position < 0 {
            self.position += 100;  
            self.password_count += 1;
        }

        // if the new dial position is 0
        if self.position == 0 {
            self.password_count += 1;
        }
    }
}

pub fn solve(content: String) {
    let mut dial = DialValue{position:50, password_count: 0};

    // Iterate over each line of input
    for line in content.split("\n") {
        // Extract the direction and magnitude from each line
        let Some(direction) = line.get(0..1) else {
            panic!("Can't parse instruction direction");
        };

        let Some(magnitude_string) = line.get(1..) else {
            panic!("Can't parse instruction magnitude");
        };

        let Ok(magnitude) = magnitude_string.parse::<i32>() else {
            panic!("Can't convert instruction magnitude to int");
        };

        // Now that everything is legit, actually update the values
        // based on the instruction.
        match direction {
            "L" => dial.decrement(magnitude),
            "R" => dial.increment(magnitude),
            _ => panic!("dial direction not L or R")
        }
    };

    println!("final dial position: {}", dial.position);
    println!("count of clicks at 0: {}", dial.password_count);
}