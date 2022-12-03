use std::fs;
use regex::Regex;

pub fn main() {
    println!("Day 3");
    part_1();
    part_2();
}

fn part_1() {
    let contents = fs::read_to_string("inputs/day3.txt").expect("Should have been able to read the file");
    
    let priority = contents.lines().fold(0, |acc, pack| -> i32 {
        let (first, second) = split_string(pack.to_string());
        let same_char = find_common_char(first, second);
        let p = score_char(same_char);
        acc+p
    });

    println!("The priority is {}", priority);
}

fn part_2() {
    let contents = fs::read_to_string("inputs/day3.txt").expect("Should have been able to read the file");
    let re =  Regex::new(r"(.*\n.*\n.*\n)").expect("Unable to create regex pattern");
    let priority = re.find_iter(&contents).fold(0, |acc, group| -> i32 {
        let groups = group.as_str().lines().collect::<Vec<_>>();
        let first = groups[0];
        let second = groups[1];
        let third = groups[2];
        let same_char = first.chars().fold('a', |acc, c| -> char {
            if second.contains(c) && third.contains(c) {
                c
            } else {
                acc
            }
        });
        println!("{} {} {}", first, second, third);
        println!("{}", same_char);
        let p = score_char(same_char);
        acc + p
    });
    println!("The priority is {}", priority);
}

fn split_string(s: String) -> (String, String) {
    let middle = s.len()/2;
    let first = s[..middle].to_string();
    let second = s[middle..].to_string();
    (first, second)
}

fn find_common_char(first: String, second: String) -> char {
    first.chars().fold('a', |acc, c| if second.contains(c) { c } else { acc })
}

fn score_char(c: char) -> i32 {
    let value = c as i32;
    if value > 91 { 
        value - 96 
    } else { 
        value - 38
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        let s = "aaabbb";
        let (first, second) = split_string(s.to_string());

        assert_eq!(first, "aaa");
        assert_eq!(second, "bbb");
    }

    #[test]
    fn test_find_common_char() {
        let result = find_common_char("aBeDe".to_string(), "gTlFB".to_string());

        assert_eq!(result, 'B');
    }

    #[test]
    fn test_score_char() {
        assert_eq!(score_char('a'), 1);
        assert_eq!(score_char('z'), 26);
        assert_eq!(score_char('A'), 27);
        assert_eq!(score_char('Z'), 52);
    }
}
