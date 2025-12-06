// I have a distinct memory of doing AoC problems like this in the past
// I'm pretty sure I implemented solutions that reads the input into a
// 2-d array, then did something like walking the array and checking 
// all of the neighbors.
//
// As seems to be my mood this year, I want to try a different approach.
// What if I made a graph? Let me think through the usual approach first.

// Let's see, it is very possible to iterate left->right, top->down and
// and check the items around the current value, then increment a sum
// if it matches our criteria. Sounds like linear time since we are
// touching each step of the input once and performing 8 or fewer checks 
// at each step. 
//   - Read the data into a matrix object that gives me some tools.
//   - Start at the top left. For each @, check the surrounding elements
//     I'm envisioning some method that returns an Option<bool> and 
//     count the number of Trues (meaning there is TP around it).
//   - If the count ever exceeds 4, move to the next @. If all surrounding
//     spaces are checked and it doesn't exceed four, increment the count.

// Thoughts:
// - The corners are always reachable since they only have 3 adjascent spots
// - I haven't been thinking about parallelizing things at all! This one
//   seems tough because it is so relational.
// - Every other way I can think of doing this is more work than just
//   walking the matrix and checking each adjascent space for an @.
//   I guess I'll go with that and see what happens.

// Struct to hold the matrix and 
pub struct Matrix {
    data: Vec<Vec<usize>>,
    width: usize,
    height: usize,
    accessible_tp_count: usize,
}

impl Matrix {
    // Read the input into a 2d array (or in this case a vec of vecs)
    // And at the same time convert the chars to 0 or 1 for simplicity 
    // Also save height and width as I'll need those for math later
    pub fn new(content: &str) -> Self {
        let lines = content.lines();
        let matrix: Vec<Vec<usize>> = lines
            .map( |line| 
                line.chars().map( |c| {
                    match c {
                        '.' => 0,
                        '@' => 1,
                        _ => panic!("input must only contain . or @")
                }})
                .collect()
            )
            .collect();

        let height = matrix.len();
        let width = matrix[0].len();

        Matrix{
            data: matrix,
            width,
            height,
            accessible_tp_count: 0,
        }
    }

    // Just a little helper to print the matrix nice and pretty
    pub fn print(&self) {
        for line in &self.data {
            println!("{:?}", line)
        }
    }

    // Main execution for the solution.
    // It iterates over each item in the matrix, calls tp_neighbor count if the 
    // current space is a @ to see how many TPs are around it, and if it is 
    // fewer than 4, it increments the count for the solution. That is what
    // is returned
    pub fn count_accessible_rolls(&mut self) -> usize {

        // Iterate over the matrix
        for (i, row) in self.data.clone().iter().enumerate() {
            for (j, val) in row.iter().enumerate() {

                // For each cell that has TP in it, check its neighbors
                if *val == 1 {
                    let tp_neighbor_count = self.tp_neighbor_count(i, j);

                    // if it has fewer than 4 let's count it!
                    if tp_neighbor_count < 4 {
                        self.accessible_tp_count += 1;
                    }
                }
            }
        }

        self.accessible_tp_count
    }

    /// Given a a cell in the matrix (identified by a row index and a col index 
    /// aka an x and y position), count and return the sum of all of the 
    /// surrounding rolls. First call the neighbors helper which returns 
    /// the coordinates of all valid neighbors, then , because I change the data
    /// in the matrix from * and @ to 0 and 1, I can just look up those values
    /// at each coordinate in the matrix and sum them!
    fn tp_neighbor_count(&self, row: usize, col: usize) -> usize {
        let neighbors = self.neighbors(row, col);
        neighbors.into_iter().map(|(x,y)| self.data[x][y]).sum()
    } 

    /// Returns a Vec of (row, col) tuples representing all of the neighbors
    /// of this cell. The implementation is a little silly because the indexes
    /// are of type usize which can't be negative. So when subtracting, I need
    /// to use an if let along with checked_sub which returns None if subtracting
    /// leads to an underflow. When adding, I can just do the normal check to see
    /// if it is out of bounds based on the width and height of the matrix. 
    fn neighbors(&self, row: usize, col: usize) -> Vec<(usize, usize)> {

        // A vec to hold all of the valid neighbor coordinates that we 
        // will later return 
        let mut output: Vec<(usize, usize)> = vec![];

        // North
        if let Some(new_row) = row.checked_sub(1) {
            output.push((new_row, col));
        }
        
        // Northeast
        if let Some(new_row) = row.checked_sub(1) {
            if col + 1 < self.width {
                output.push((new_row, col + 1));
            }      
        }

        // East
        if col + 1 < self.width {
            output.push((row, col + 1));
        }

        // Southeast
        if row + 1 < self.height {
            if col + 1 < self.width {
                output.push((row + 1, col + 1));
            }
        }

        // South
        if row + 1 < self.height {
            output.push((row + 1, col));
        }

        // Southwest
        if row + 1 < self.height {
            if let Some(new_col) = col.checked_sub(1) {
                output.push((row + 1, new_col));
            }
        }

        // West
        if let Some(new_col) = col.checked_sub(1) {
            output.push((row, new_col));
        }

        // Northwest
        if let Some(new_row) = row.checked_sub(1) {
            if let Some(new_col) = col.checked_sub(1) {
                output.push((new_row, new_col));
            }      
        }

        output
    }
}



pub fn solve(content: &str) {

    // Set up the problem be reading in the data
    // This ends up converting the matrix from a string of *s and @s to
    // a 2d matrix of 0s and 1s. It also derives and saves the height
    // and width of the matrix for use later. See pub fn new in
    // the impl for Matrix.
    let mut matrix = Matrix::new(content);

    // check the array to make sure I ingested it properly
    // matrix.print();

    // Follow this call chain for the solution!
    let solution = matrix.count_accessible_rolls();

    println!("solution: {}", solution);
}