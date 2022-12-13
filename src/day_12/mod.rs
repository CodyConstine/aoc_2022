use std::fs;

pub fn main() {
    println!("Day 12");
    part_1();
}

pub struct DijkstraNode<'a> {
    neighbors: Vec<&'a DijkstraNode<'a>>,
    name: String,
    height: i32,
    dist: i32,
    end: bool,
    prev: Option<&'a DijkstraNode<'a>>,
}

fn part_1() {
    let contents = fs::read_to_string("inputs/day12_dummy.txt").expect("Should have been able to read the file");
    
    let mut start: DijkstraNode;

    let mut nodes: Vec<Vec<&DijkstraNode>> = Vec::new(); 

    for (x, line) in contents.lines().enumerate() {
        nodes.push(Vec::new());
        for (y, c) in line.chars().enumerate() {
            let (height, start, end) = if c == 'S' {
                (0, true, false)
            } else if c == 'E' {
                (26, true, false)
            } else {
                (c as i32 - 97, false, false)
            };
            let mut node = DijkstraNode{
                neighbors: Vec::new(), 
                name: format!("({}, {})", x, y),
                height: height,
                dist: if start { 0 } else { 99999 },
                end: end,
                prev: None};
            nodes[x].push(&node);

            if x > 0 && (nodes[x-1][y].height - height).abs() < 2 {
                node.neighbors.push(&nodes[x-1][y]);
                
            }
            if y > 0 && (nodes[x][y-1].height - height).abs() < 2 {
                node.neighbors.push(&nodes[x][y-1]);
            }

        }
    }
}
