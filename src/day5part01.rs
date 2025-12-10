// Let's break this down a little and write a little BST
// just to help me remember how to do it.

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn to_string(&self) -> String {
        return format!("{}", self.val.to_string());
    }
}

// Define a from to make Node creation simpler
impl From<i32> for Box<Node> {
    fn from(val: i32) -> Self {
        Box::new(Node { val, left: None, right: None })
    }
}


struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn insert(&mut self, val: i32) {

        let mut node = &mut self.root;

        while let Some(cur) = node {
            if val <= cur.val {
                node = &mut cur.left;
            } else {
                node = &mut cur.right;
            }
        }
        *node = Some(val.into());
    }

    // Returns true if the value is in the tree and false otherwise.
    pub fn is_in_tree(&mut self, val: i32) -> bool {
        let mut node = &mut self.root;

        while let Some(cur) = node {
            if val == cur.val {
                return true
            }

            if val < cur.val {
                node = &mut cur.left;
            } else {
                node = &mut cur.right;
            }
        }

        return false
    }

    pub fn print(&self) {
        let mut path = vec![];
        if let Some(node) = &self.root {
            self.recursive_print(&node, 0, "", &mut path);
        } else {
            println!("Tree is empty");
        }
    }

    // The goal here is to recursively scan the tree printing at the right time
    // to get good tree printing, I need to pass a vec of all the turns I've made
    //       ┌──20
    //    ┌──15
    // ┌──11
    // │   │      ┌──10
    // │   │   ┌──9
    // │   └──7
    // 5
    // │   ┌──4
    // └──3
    //    └──3
    fn recursive_print(&self, node: &Box<Node>, depth: usize, from_direction: &str, path: &mut Vec<char>){
        // if there is right go right
        if let Some(right_node) = &node.right {
            path.push('R');
            self.recursive_print(&right_node, depth + 1, "right", path);
        }
       
        if depth == 0{
            println!("{}", node.val.to_string());
        } else {

            let mut prefix: Vec<String> = vec![];

            for (i, d) in path.iter().enumerate() {
                if i == 0 {
                    continue;
                } else {
                    if path[i-1] == *d {
                        prefix.push("   ".to_string());
                    } else {
                        prefix.push("│   ".to_string());

                    }
                }
            }

            let elbow = match from_direction {
                "right" => "┌",
                "left" => "└",
                _ => ""
            };

            println!("{}{}{}{}", prefix.join("").to_string(), elbow, "──", node.val.to_string());
        }

        // if there is left go left
        if let Some(left_node) = &node.left {
            path.push('L');
            self.recursive_print(&left_node, depth + 1, "left", path);
        }
        path.pop();
    }

}


pub fn solve(_content: &str) {
    let mut tree = Tree{root: None};
    tree.print();

    tree.insert(5);
    tree.insert(3);
    tree.insert(11);
    tree.insert(15);
    tree.insert(3);
    tree.insert(7);
    tree.insert(4);
    tree.insert(9);
    tree.insert(10);
    tree.insert(20);


    println!("find 8, which is not in the tree: {:?}", tree.is_in_tree(8));
    println!("find 9, which IS in the tree: {:?}", tree.is_in_tree(9));

    tree.print();
}
