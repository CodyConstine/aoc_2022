use std::fs;

pub fn main() {
    println!("Day 4");
    part_1();
    part_2();
}

fn part_1() {
    let contents = fs::read_to_string("inputs/day4.txt").expect("Should have been able to read the file");
    
    let fully_contained = contents.lines().fold(0, |acc, schedule| -> i64 {
        let schedule_parsed: Vec<i32> = schedule.split(",").flat_map(|x| x.split("-").map(|y| y.to_string().parse::<i32>().unwrap())).collect();
        
        if schedule_parsed[0] >= schedule_parsed[2] && schedule_parsed[1] <= schedule_parsed[3] {
            acc + 1
        } 
        else if schedule_parsed[2] >= schedule_parsed[0] && schedule_parsed[3] <= schedule_parsed[1] {
            acc + 1
        } 
        else {
            acc + 0
        }
    });

    println!("The number of fully contained schedules is {}", fully_contained);
}

fn part_2() {
    let contents = fs::read_to_string("inputs/day4.txt").expect("Should have been able to read the file");
    
    let fully_contained = contents.lines().fold(0, |acc, schedule| -> i64 {
        let schedule_parsed: Vec<i32> = schedule.split(",").flat_map(|x| x.split("-").map(|y| y.to_string().parse::<i32>().unwrap())).collect();
        
        if schedule_parsed[0] >= schedule_parsed[2] && schedule_parsed[0] <= schedule_parsed[3] {
            acc + 1
        } 
        else if schedule_parsed[2] >= schedule_parsed[0] && schedule_parsed[2] <= schedule_parsed[1] {
            acc + 1
        } 
        else {
            acc + 0
        }
    });

    println!("The number of overlapping schedules is {}", fully_contained);
}
