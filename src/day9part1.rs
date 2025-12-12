use std::fmt;

// A given point can be one corner of a square
// and a connecting point must be its opposite.
// So each point could possibly any corner, and
// we could look for its max matching corner
// Given a point, at point x,y (for now imagine)
// it is at the center of the canvas. For each 
// of the four cases, find its corresponding opposite
// The point is:
//   - the top left corner (min x, min y)
//     - find the bottom right (max x, max y)
//   - the top right corner (max x, min y)
//     - find the bottom left (min x, max y)
//   - the bottom right corner (max x, max y)
//     - find the top left (min x, min y)
//   - the bottom left corner (min x, max y)
//     - find the top right (max x, min y)
// Now of course because it is symmetrical, we'd end
// up finding it twice in both directions. So really
// we only need to do two directions. 
// 
// OOOORRrrrrr I could brute force it. Rust go brrrr!

struct Point {
    x: isize,
    y: isize
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    pub fn area(&self, other: &Point) -> isize {

        let x_diff = (self.x - other.x + 1).abs();
        let y_diff = (self.y - other.y + 1).abs();

        (x_diff) * (y_diff)
    }
}

pub fn solve(content: &str) {

    let points: Vec<Point> = content.lines()
        .into_iter()
        .map(|line| line.split(","))
        .map(|mut split| Point{x: split.next().unwrap().parse().unwrap(), y: split.next().unwrap().parse().unwrap()})
        .collect();

    // How about an ugly n^2 attempt?
    // for each point, check the distance of all other points
    let mut largest_area: isize = 0;
    for p1 in &points {
        for p2 in &points {
            let area = p1.area(p2);
            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("{:?}", largest_area);
}