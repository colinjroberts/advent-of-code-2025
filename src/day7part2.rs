use std::collections::HashMap;
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

// Last thoughts:
// Man...I could really use some dynamic programming practice.
// I hope I can revist this one, becuase it is a bit of a mess
// 
// The general approach is to ingest the input and build a representation
// of the graph by keeping a HashMap with a beam location as the key and
// the locations of child beams as the values. Items are added to map
// whenever we split a beam. Then once the graph is built, we can do a 
// memoized recursive search of the paths/


// Recursively search the graph to count the number of paths from 
// start to end there are through it. 
pub fn recursive_traverse_graph(paths: HashMap<(usize, usize), HashSet<(usize, usize)>>,  path_counts: &mut HashMap<(usize, usize), usize>, cur: (usize, usize)) -> usize {
    
    // for keeping track of child counts since I'm doing it in a for loop
    let mut child_path_counts = 0;

    if let Some(children) = paths.get(&cur) {
        // if this node has children, 
       
        // return the counts if we have already calcuated it
        if let Some(existing_counts) = path_counts.get(&cur) {
            return *existing_counts
        } 
        else {
            // otherwise, calculate the counts
            for child in children {
                child_path_counts += recursive_traverse_graph(paths.clone(), path_counts, *child);
            }
            path_counts.insert(cur, child_path_counts);

            return child_path_counts
        }
    } else {
        // if there are no children, there is only one path
        return 1
    }
}

// This solution is a little gross. It is all in one big method here.
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

    // Same general idea as in part 1, but instead
    // of using a set to just track whether a beam exists
    // at a given x location, we keep track of the location
    // where the beam started. This way, when we are splitting
    // a beam, we know all of the locations it came from (since
    // it is totally possible for a beam to com from two parents
    // one of which was several layers above the split.
    // So instead of sets, I'm using maps.

    // Maps for ingesting the graph and making sure we split at the right place
    let mut cur_beams: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
    let mut next_beams: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();

    // Map for building a representation of the graph
    let mut paths: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut path_joins = 0;

    // initialize next beams with the idx of the starting beam
    next_beams.insert(starting_laser_index, HashSet::from([(starting_laser_index, 0 as usize)]));


    // Now using the indexes of the splitters in order
    // keep track of where the beams are
    for (y, splitter_row) in beam_splitters.into_iter().enumerate() {
        cur_beams = next_beams.clone();

        for splitter_idx in splitter_row {

            // if there is a splitter in a location of the beam,
            // we need to update the beams for the next run
            // as well as the path_split_counter
            if cur_beams.contains_key(&splitter_idx) {

                
                let nodes = next_beams.remove(&splitter_idx).unwrap();

                // In addition to processing the input, we are building a representation
                // of the graph. Whenever we split a beam, it means we are making two children
                // so add an entry to our map to track that this beam location has two children
                for (parent_x, parent_y) in nodes {
                    paths.entry((parent_x, parent_y)).and_modify(|set| {set.insert((splitter_idx - 1, y+1));} ).or_insert(HashSet::from([(splitter_idx - 1, y+1)]));
                    paths.entry((parent_x, parent_y)).and_modify(|set| {set.insert((splitter_idx + 1, y+1));} ).or_insert(HashSet::from([(splitter_idx + 1, y+1)]));
                }

                next_beams.entry(splitter_idx - 1).and_modify(|set| {set.insert((splitter_idx - 1, y+1));} ).or_insert(HashSet::from([(splitter_idx - 1, y+1)]));
                next_beams.entry(splitter_idx + 1).and_modify(|set| {set.insert((splitter_idx + 1, y+1));} ).or_insert(HashSet::from([(splitter_idx + 1, y+1)]));
            }
        }
    }

    // With the graph built, we can search it recursively for path options
    let mut path_counts:  HashMap<(usize, usize), usize> = HashMap::new();
    let result = recursive_traverse_graph(paths, &mut path_counts, (starting_laser_index,0));

    println!("result: {:?}", result);
}