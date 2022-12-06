use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn main() {
    println!("Day 6");
    let contents = fs::read_to_string("inputs/day6.txt").expect("Should have been able to read the file");
    
    let window = find_first_unique_window(&contents, 4);
    println!("The first unqiue group of 4 is {}", window + 4);
    let window2 = find_first_unique_window(&contents, 14);
    println!("The first unqiue group of 4 is {}", window2 + 14);
}

fn find_first_unique_window(input: &String, window_size: usize) -> usize {
    for (i, slice) in input.chars().collect::<Vec<_>>().windows(window_size).enumerate() {
        let hash: HashSet<char> = HashSet::from_iter(slice.iter().cloned());
        if hash.len() == window_size {
            println!("{}: {}{}{}{}: {}", i, slice[0],slice[1],slice[2],slice[3], hash.len());
            return i;
        }
    }
    return 0;
}

