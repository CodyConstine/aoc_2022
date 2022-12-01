use std::fs;

pub fn main() {
    println!("Day 1");

    let contents = fs::read_to_string("inputs/day1.txt").expect("Should have been able to read the file");

    let mut elves_cargo = vec![];

    for (elf_num, cargo) in contents.rsplit("\n\n").enumerate() {
        let cargo_list: Vec<i64> = cargo.lines().map(|x| x.parse::<i64>().unwrap()).collect();
        let total = cargo_list.iter().sum::<i64>();
        elves_cargo.push(total);
    }

    elves_cargo.sort();
    
    println!("Part 1 solution {}", &elves_cargo[elves_cargo.len()-1]);
    println!("Part 2 solution {}", elves_cargo[elves_cargo.len()-3..elves_cargo.len()].iter().sum::<i64>());
}
