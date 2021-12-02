use std::fs::File;
use std::io::{self, BufRead};

pub struct Direction {
    depth: i32,
    horizontal: i32,
}

impl Direction {
    fn new(d: i32, h: i32) -> Direction {
        Direction {
            depth: d,
            horizontal: h,
        }
    }

    fn add(&mut self, dir: Direction) {
        self.depth += dir.depth;
        self.horizontal += dir.horizontal;
    }
}

pub fn parse_input(filepath: &str) -> io::Result<Vec<Direction>> {
    let file = File::open(filepath)?;

    let mut input: Vec<Direction> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let elements: Vec<&str> = line.split_whitespace().collect();

            if let Ok(size) = elements[1].parse::<i32>() {
                if let Some(current_direction) = match elements[0] {
                    "forward" => Some(Direction::new(0, size)),
                    "down" => Some(Direction::new(size, 0)),
                    "up" => Some(Direction::new(-size, 0)),
                    _ => None,
                } {
                    input.push(current_direction)
                }
            }
        }
    }
    Ok(input)
}

pub fn part_1(input: Vec<Direction>) -> i32 {
    let mut overall_direction = Direction::new(0, 0);
    for current_direction in input {
        overall_direction.add(current_direction)
    }
    overall_direction.depth * overall_direction.horizontal
}

pub struct DirectionPt2 {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl DirectionPt2 {
    fn new(d: i32, h: i32, a: i32) -> DirectionPt2 {
        DirectionPt2 {
            depth: d,
            horizontal: h,
            aim: a,
        }
    }

    fn progress(&mut self, dir: DirectionPt2) {
        self.aim += dir.aim;
        self.horizontal += dir.horizontal;
        // If the horizontal value is positive, this means
        // that it was a "forward" direction and your depth
        // has to be increased by your aim*X,
        // where x is the same as the dir's horizontal value.
        if dir.horizontal > 0 {
            self.depth += self.aim * dir.horizontal
        }
    }
}

pub fn parse_input_pt2(filepath: &str) -> io::Result<Vec<DirectionPt2>> {
    let file = File::open(filepath)?;

    let mut input: Vec<DirectionPt2> = Vec::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let elements: Vec<&str> = line.split_whitespace().collect();

            if let Ok(size) = elements[1].parse::<i32>() {
                if let Some(current_direction) = match elements[0] {
                    "forward" => Some(DirectionPt2::new(0, size, 0)),
                    "down" => Some(DirectionPt2::new(0, 0, size)),
                    "up" => Some(DirectionPt2::new(0, 0, -size)),
                    _ => None,
                } {
                    input.push(current_direction)
                }
            }
        }
    }
    Ok(input)
}

pub fn part_2(input: Vec<DirectionPt2>) -> i32 {
    let mut overall_direction = DirectionPt2::new(0, 0, 0);
    for current_direction in input {
        overall_direction.progress(current_direction)
    }
    overall_direction.depth * overall_direction.horizontal
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part_1() {
        let filepath = "./day-1.test";
        let test_input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert!(fs::write(filepath, test_input).is_ok());
        match super::parse_input(filepath) {
            Ok(input) => assert_eq!(super::part_1(input), 150),
            Err(_) => assert!(false),
        };
        assert!(fs::remove_file(filepath).is_ok());
    }

    #[test]
    fn part_2() {
        let filepath = "./day-1.test";
        let test_input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert!(fs::write(filepath, test_input).is_ok());
        match super::parse_input_pt2(filepath) {
            Ok(input) => assert_eq!(super::part_2(input), 900),
            Err(_) => assert!(false),
        };
        assert!(fs::remove_file(filepath).is_ok());
    }
}
