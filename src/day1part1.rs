// We have a dial with numbers 0 to 99. It starts at 50.
// An L value subtracts that number, an R adds it. After
// Each rotation, we need to check to see whether the
// value is 0 and count it, since that will be our answer

/// A representation of both the current position of the dial
struct DialValue {
    position: i32,
    password_count: i32
}

/// Define how to turn right and left on the dial and
/// keep track of when the value ends up as 0
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
        }
        if self.position == 0 {
            self.password_count += 1;
        }
    }

    // Same idea, but down.
    fn decrement(&mut self, amount: i32) {
        self.position -= amount;
        while self.position < 0 {
            self.position += 100;   
        }
        if self.position == 0 {
            self.password_count += 1;
        }
    }
}

pub fn solve(content: &str) {
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
    println!("count of stops at 0: {}", dial.password_count);
}
