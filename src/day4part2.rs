// Ahh yes, we are doing a screen renderer! We now need to loop 
// over the matrix, run the counter that we wrote in part one, but 
// now the matrix needs to be updated in the process! Be careful
// though, we can't update as we go since if we change a '@' to 
// a '.' then when we check the next space, it will have one fewer
// roll around it which messes up the count for this iteration. So
// we'll need to add a second data object for rendering. An optimized
// version of this would use actual Arrays since the whole thing is
// of fixed size, but for now I'm going to stick with Vecs.


// Struct to hold the matrix, the solution, and other related info
// needed for calculations (like height and width)
pub struct Matrix {
    data: Vec<Vec<usize>>,
    data2: Vec<Vec<usize>>,
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
            data: matrix.clone(),
            data2: matrix,
            width,
            height,
            accessible_tp_count: 0,
        }
    }

    // Just a little helper to print the matrix nice and pretty
    // and more like the input so it is easier to compare to the
    // example in case I mess it up.
    pub fn print(&self) {
        for row in &self.data {
            for val in row {
                match *val {
                    0 => print!("{}", '.'),
                    1 => print!("{}", '@'),
                    _ => panic!("Woah, how'd that get in there!?")
                };
            }
            print!("\n")
        }
        print!("\n")
    }


    /// Main execution wrapper for the solution
    /// Calls count_and_remove_accessible_rolls repeatedly until we get a
    /// run where we didn't remove any rolls.
    pub fn recursively_count_and_remove_accessible_rolls(&mut self, print: bool) -> usize {

        loop {
            if print {
                self.print();
            }
            // repeatedly count and remove tp until we can't anymore
            if self.count_and_remove_accessible_rolls() == 0 {
                break;
            }
        }

        self.accessible_tp_count
    }

    /// Iterates over each item in the matrix, for each roll (@) it finds,
    /// calls check_tp_accessibility which tells us whether the roll will
    /// be removed for the next run. We keep track of that so that when
    /// 0 rolls are removed, the recursion will stop. 
    fn count_and_remove_accessible_rolls(&mut self) -> usize {

        let mut tps_removed: usize = 0;

        // Iterate over the matrix
        for (i, row) in self.data.clone().iter().enumerate() {
            for (j, val) in row.iter().enumerate() {

                // Only run our check if the current value is a @ 
                // (which in my struct is represented by a 1)
                if *val == 1 {
                    // Changed a little logic here from part 1. Now
                    // check_tp_accessibility handles getting the neighbors,
                    // checking whether this space is accessible,
                    // preps a matrix for the next run, and tells us
                    // if we "removed" a roll or not.
                    tps_removed += self.check_tp_accessibility(i, j);
                }
            }
        }

        // After scanning the whole matrix, getting all the counts, and
        // prepping a new matrix with all of the accessible TPs removed,
        // let's update data to be the new one we have been making.
        self.data = self.data2.clone();

        tps_removed
    }

    /// Given a a cell in the matrix (identified by a row index and a col index 
    /// aka an x and y position), count the number of accessible rolls 
    /// surrounding it. If it is less than 4, it means it is accessible, so
    /// we should add it to our running total of aaccessible rolls and remove it
    /// the map for next time. Finally, return a 1 if we removed a roll and 0 
    /// otherwise. We will be using that number to know when we can't remove 
    /// any more tp.
    fn check_tp_accessibility(&mut self, row: usize, col: usize) -> usize {
        let neighbors = self.neighbors(row, col);
        let tp_neighbor_count: usize = neighbors.into_iter().map(|(x,y)| self.data[x][y]).sum();

        // if it has fewer than 4, it is accessible! That means
        // that we need to both count it and remove it for the
        // next run.
        if tp_neighbor_count < 4 {
            self.data2[row][col] = 0;
            self.accessible_tp_count += 1;
            return 1;
        }

        0
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

    // Follow this call chain for the solution!
    // For part 2, we need to count and remove accessible rolls until
    // we can't any more, so let's do it over and over!
    let solution = matrix.recursively_count_and_remove_accessible_rolls(false);

    println!("solution: {}", solution);
}