use std::fs;

pub fn main() {
    println!("Day 5");
    part_1();
}

fn part_1() {
    let contents = fs::read_to_string("inputs/day5.txt").expect("Should have been able to read the file");
    
    let stacks: Vec<Vec<char>> = contents
        .lines()
        .take(8)
        .map(|level| -> Vec<char> {
            level.chars().collect::<Vec<char>>().chunks(4).map( |x| x[1] ).collect()
        }).collect();

    let output_stacks = contents.lines().skip(10).fold(stacks, |acc, instruction| -> Vec<Vec<char>> {
        println!("{}", instruction);
        acc
    });

    for stack in output_stacks {
        for s in stack {
            print!("{} ", s)
        }
        println!("")
    }
}

