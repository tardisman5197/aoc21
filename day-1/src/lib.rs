use std::fs::File;
use std::io::{self, BufRead};

pub fn parse_input(filepath: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filepath)?;

    let mut input: Vec<i32> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if let Ok(height) = line.parse::<i32>() {
                input.push(height)
            } 
        }
    }
    Ok(input)
}

pub fn part_1(input: Vec<i32>) -> i32 {
    let mut no_of_inc = 0;
    for i in 1..input.len() {
        if input[i] > input[i-1] {
            no_of_inc += 1;
        }
    }
    no_of_inc
}

pub fn part_2(input: Vec<i32>) -> i32 {
    let mut no_of_inc = 0;

    for i in 1..input.len()-2 {
        let prev: i32 = input[i-1..i+2].iter().sum();
        let cur: i32 = input[i..i+3].iter().sum();
        if cur > prev  {
            no_of_inc += 1;
        }
    }
    no_of_inc
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part_1() {
        let filepath = "./day-1.test";
        let test_input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert!(fs::write(filepath, test_input).is_ok());
        match super::parse_input(filepath) {
            Ok(input) => assert_eq!(super::part_1(input), 7),
            Err(_) => assert!(false)
        };
        assert!(fs::remove_file(filepath).is_ok());
    }

    #[test]
    fn part_2() {
        let filepath = "./day-1.test";
        let test_input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        assert!(fs::write(filepath, test_input).is_ok());
        match super::parse_input(filepath) {
            Ok(input) => assert_eq!(super::part_2(input), 5),
            Err(_) => assert!(false)
        };
        assert!(fs::remove_file(filepath).is_ok());
    }
}