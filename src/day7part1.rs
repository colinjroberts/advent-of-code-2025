use std::collections::HashSet;

// First thoughts: 
// How do I get away from the visual representation
// and into something more concrete/condensed?
// 
// Create a set of beam indexes (starting with the initial beam)
// Have sets of splitter indicies
// Iterate over each row of splitter. If in the previous run
// there is a laser at the same index as the splitter, remove 
// the beam from the set and add beams at i+1 and i-1

pub fn solve(content: &str) {

    // Initialize our vec as well as a variable for marking
    // the starting location
    let mut beam_splitters: Vec<Vec<usize>> = vec![];
    let mut starting_laser_index = 0;

    // extract the splitter indexes
    // we will later iterate over these locations
    for line in content.split("\n").collect::<Vec<&str>>() {
        let mut splitters = vec![];

        for (idx, c) in line.chars().enumerate() {
            if c == 'S' {
                starting_laser_index = idx;
            }

            if c == '^' {
                splitters.push(idx);
            }
        }

        if splitters.len() > 0 {
            beam_splitters.push(splitters);
        }
    }


    // Set up a current set of beam locations and a 
    // next set of beam locations. When we iterate through
    // each set of splitters, we will look at cur beams to 
    // see if there is a laser at the same index a splitter
    // is at. If so, we will remove that index from next_beams
    // and add index + 1 and index - 1 to next_beams. Using
    // a set handles duplicates, and we need two sets to 
    // be able to process one row without messing it up by
    // making changes in place. 
    let mut cur_beams: HashSet<usize> = HashSet::new();
    let mut next_beams: HashSet<usize> = HashSet::new();

    // initialize next beams with the idx of the starting beam
    next_beams.insert(starting_laser_index);

    let mut counter = 0;

    // Now using the indexes of the splitters in order
    // keep track of where the beams are
    for splitter_row in beam_splitters {
        cur_beams = next_beams.clone();

        for splitter_idx in splitter_row {
            // if there is a splitter in a location of the beam,
            // we need to update the beams for the next run
            // as well as the counter
            if cur_beams.contains(&splitter_idx) {
                counter += 1;
                next_beams.remove(&splitter_idx);
                next_beams.insert(splitter_idx - 1);
                next_beams.insert(splitter_idx + 1);
            }
        }
    }

    println!("counter: {:?}", counter);
}