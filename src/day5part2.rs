use std::fmt;

// Pretty much the same as part 1. I got that working even though it isn't
// the pretty, fancy range BST I wanted to make. Part 2 was a just a 
// small adjustment, so I figured I'd do it just to do it. Hopefully
// I'll come back and update one of these to be the cool fancy BST
// I want it to be. I think it will either need to be something crazy like
// a B+ tree where each leaf has a pointer to the next left or
// if all I'm looking for is logarithmic lookup, I can imnplement 
// a binary search 

// a range is two numbers (that can be the same number)
// where start <= end
struct Range {
    start: usize,
    end: usize,
}

// Pretty printing for Range
impl fmt::Display for Range {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}
// More pretty printing for Range
impl fmt::Debug for Range {
     // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

// return how many items are in the range (inclusive)
// e.g. 2-2 has len 1, 2-5 has len 4, 1-10 has len 10
impl Range {
    fn len(&self) -> usize {
        self.end -  self.start + 1
    }
}

// Easily convert a string in the format "1-5" to a Range with start and end
impl From<&str> for Range {
    fn from(val: &str) -> Self {
        let split: Vec<&str> = val.split("-").into_iter().collect::<Vec<&str>>();
        
        if split.len() != 2 {
            panic!("a range can only be made from a string like xx-yy")
        }
    
        let results: Vec<usize> = split.iter().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let min = results.iter().min().unwrap();
        let max = results.iter().max().unwrap();

        Range{start: *min, end: *max}
    }
}

// A collection of non-overlapping Ranges that implements range merging on insert.
struct RangeList {
    ranges: Vec<Range>
}

impl RangeList {

    // Inserting values into the list should handle merges
    // so that by the time insert is finished, the RangeList 
    // has only mutually exclusive ranges. At rest, ranges
    // should be sorted by the start value.
    // e.g. existing list [1-3, 4-6, 7-9, 11-13] and we add 5-12
    // First we position the new item
    // [1-3, 4-6, 7-9, 11-13]
    //           ^
    //           5-12
    //
    // there is overlap with previous item, so the end of that gets updated
    // and we move cur back one so it is like we just inserted this new range
    // list now looks like this
    // [1-3, 4-12, 7-9, 11-13]
    //       ^
    //       cur_idx
    // 
    // we check the number in front of it and see that there is full overlap
    // so we remove that number in the list and see that there is partial overlap
    // so we update cur and remove the next number
    // [1-3, 4-12, 11-13]
    //       ^
    //       cur_idx
    // 
    // [1-3, 4-13, 11-13]
    // [1-3, 4-13]
    //
    // This method could really use some optimizing. I know there is a way
    // to make this more elegant, but I haven't thought of it yet
    pub fn insert(&mut self, val: &str){

        // For ranges that are mutually exclusive,
        // Iterate over the list until you find a range start that
        // is greater than this one, then insert it.
        let new_range: Range = val.into();
    
        // // A working way to insert items at the right position
        // let mut new_idx = self.ranges.len();
        // for (idx, item) in self.ranges.iter().enumerate() {
        //     if new_range.start < item.start {
        //         if new_range.end < item.start {
        //             // they are mutually exclusive, so add it
        //             // right before this item.
        //             new_idx = idx;
        //             break;
        //         } else {
        //             // This means that the new item bleeds into the next item
        //             // if the new range's end is greater than this item's 
        //             // start, then we can merge these items by inserting
        //             // an updated 
        //             // [1-3, 4-7] adding 1-3 or 3-10
                    
        //             // If new.end < next.end, that is it 
        //             item.start = new_range.start;
        //         }
                
        //     }
        // }
        // self.ranges.insert(new_idx, new_range);
        

        let mut cur_idx = 0;

        // println!("insert: about to loop, cur_ids: {}, self.ranges.len(): {}", cur_idx, self.ranges.len() );
        // Scan to get the correct index for inserting.
        while cur_idx < self.ranges.len() && self.ranges[cur_idx].start < new_range.start {
            cur_idx += 1;
        }

        // println!("insert: Ended initial range scan on index: {}", cur_idx);

        // if the new items comes after another item, we need to check for overlap
        if cur_idx != 0 {

            // if prev.end >= new.start - 1 it means there is left overlap
            // e.g. prev: 2-5 and new: 2-5 or 4-7 or 6-8
            if self.ranges[cur_idx - 1].end >= new_range.start - 1 {

                // prev.end >= new.end it is complete overlap 
                // e.g. prev: 2-5 and new: 3-4 or 2-5
                if self.ranges[cur_idx - 1].end >= new_range.end {
                    // no other action at all is needed
                    // we don't need to insert the new range or anything
                } else {

                    // otherwise prev.end < new.end, so we should update the
                    // end on prev
                    // e.g.  prev: 2-5 and new: 2-7
                    self.ranges[cur_idx - 1].end = new_range.end;
                    cur_idx -= 1;
                }
            } else {
                // if there is no left overlap, we insert a new item
                self.ranges.insert(cur_idx, new_range);
            }

        } else {

            // if the new item we want to insert goes at the beginning, insert it
            self.ranges.insert(cur_idx, new_range);
        }

        // Now we need to check for overlap in the next ranges
        // If there is right overlap we will remove the item or update to make sure
        // everything is mutually exclusive.
        // Only scan to the second to last item, because if it the last item, there 
        // is nothing in front of it to modify

        // if new.end > next.start and new.end > next.end there is some overlap
        // extending to the right. 
        while cur_idx < self.ranges.len() - 1 && self.ranges[cur_idx].end >= self.ranges[cur_idx + 1].start - 1 {

            // if new.end < next.end there is partial overlap
            // e.g. new: 5-8 and next: 5-9 or 8-10
            if self.ranges[cur_idx].end < self.ranges[cur_idx + 1].end {
                // extend the end of the new range
                self.ranges[cur_idx].end = self.ranges[cur_idx + 1].end
            }

            // for both partial overlap above and complete overlap 
            // (e.g. new: 5-8 and next: 4-7 or 5-8)
            // we will remove the next item in the list
            self.ranges.remove(cur_idx + 1);

            // This process should repeat until the item we added does not overlap 
            // the next item OR if the item we added becomes the last item in the list
        }
    }

    /// Scan the list and return true if the number is in a range in the list
    /// and false otherwise. Even though I don't have a BST, I can still 
    /// do a binary search! 
    pub fn find(&self, num: usize) -> bool {

        // lame boring linear search
        // for range in &self.ranges {

        //     // break early if we exceed the range 
        //     if num < range.start {
        //         return false
        //     } 

        //     if num >= range.start && num <= range.end {
        //         return true
        //     }
        // }

        // return false


        // Fun exciting binary search!
        let mut min = 0;
        let mut max = self.ranges.len();

        while min < max {
            let mid = min + (max - min) / 2;
            let range = &self.ranges[mid];

            // on each iteration, check to see if this is the range that fits
            if num >= range.start && num <= range.end {
                return true
            }

            if num < self.ranges[mid].start {
                max = mid
            } else {
                min = mid + 1
            }

            if min == max {
                return false;
            }
        }


        return false
    }

    pub fn nums_in_ranges(&self) -> usize {
        let mut count = 0;
        for range in &self.ranges {
            count += range.len()
        }
        count
    }

    pub fn print(&self){
        println!("RangeList: {:?}", self.ranges);
    }
}



// The super simplist version is to just put everything
// into a list that we will scan every time. Let's do that 
// for now.
pub fn solve(content: &str) {

    let split_content: Vec<&str> = content.split("\n\n").collect::<Vec<&str>>();
    let ranges = split_content[0];
    let mut range_list = RangeList{ranges: vec![]};

    for range in ranges.lines() {
        range_list.insert(range);
    }

    let fresh_count = range_list.nums_in_ranges();

    println!("fresh count: {:?}", fresh_count);
}


// Some tests:
// let test_mutually_exclusice_str_vec: Vec<&str> = vec!["1-5",  "20-25", "10-15", "25-30"];
// let test_expand_right_str_vec: Vec<&str> = vec!["1-4", "3-7", "2-8", "6-10"];
// let test_expand_left_str_vec: Vec<&str> = vec!["10-15", "8-13", "10-17", "16-20"];
// let test_superset_str_vec: Vec<&str> = vec!["5-6", "5-6", "4-7", "3-8", "2-9", "2-9"];
// let test_subset_str_vec: Vec<&str> = vec!["2-9", "3-8", "4-7", "5-6"];

// let tests:Vec<Vec<&str>> = vec![test_mutually_exclusice_str_vec, test_expand_right_str_vec, test_expand_left_str_vec, test_superset_str_vec, test_subset_str_vec];

// for test in tests {
//     println!("Testing with list: {:?}", test);
//     let mut range_list = RangeList{ranges: vec![]};
//     for input in test {
//         // println!("inserting: {}", input);
//         range_list.insert(input);
//         range_list.print();

//         println!("Is num 7 in range?: {}", range_list.find(7));
//     }
//     range_list.print();

//     println!();
// }
