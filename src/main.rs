use std::env;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("No day provided");
        return;
    }

    let day = args[1].parse::<i32>().unwrap();

    println!("Running day {}", day);

    match day {
        1 => day_1::main(),
        2 => day_2::main(),
        3 => day_3::main(),
        4 => day_4::main(),
        _ => println!("Day not reconized")
    }
}
