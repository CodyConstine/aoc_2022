use std::fs;

pub fn main() {
    println!("Day 2");
    part_1();
    part_2();
}

fn part_1() {
    let contents = fs::read_to_string("inputs/day2.txt").expect("Should have been able to read the file");
    
    let score = contents.lines().fold(0, |acc, round| -> i64 {
        let mut split = round.split(" ");
        let opp_play = split.next().unwrap();
        let my_play = split.next().unwrap();
        let result = match (opp_play, my_play) {
            ("A", "X") => 4,
            ("A", "Y") => 8,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 7,
            ("C", "Y") => 2,
            ("C", "Z") => 6,
            _ => 0
        };
        acc+result
    });

    println!("The score is {}", score);
}

fn part_2() {
    let contents = fs::read_to_string("inputs/day2.txt").expect("Should have been able to read the file");
    
    let score = contents.lines().fold(0, |acc, round| -> i64 {
        let mut split = round.split(" ");
        let opp_play = split.next().unwrap();
        let my_play = split.next().unwrap();
        let result = match (opp_play, my_play) {
            ("A", "X") => 3,
            ("A", "Y") => 4,
            ("A", "Z") => 8,
            ("B", "X") => 1,
            ("B", "Y") => 5,
            ("B", "Z") => 9,
            ("C", "X") => 2,
            ("C", "Y") => 6,
            ("C", "Z") => 7,
            _ => 0
        };
        acc+result
    });

    println!("The score is {}", score);
}
