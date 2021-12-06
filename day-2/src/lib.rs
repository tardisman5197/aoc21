pub fn part_1(filepath: &str) -> i32 {
    let (horizontal, depth) = std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0), |(horizontal, depth), (cmd, size)| {
            match (cmd, size.parse::<i32>().unwrap()) {
                ("forward", size) => (horizontal + size, depth),
                ("down", size) => (horizontal, depth + size),
                ("up", size) => (horizontal, depth - size),
                _ => unreachable!(),
            }
        });
    horizontal * depth
}

pub fn part_2(filepath: &str) -> i32 {
    let (horizontal, depth, _) = std::fs::read_to_string(filepath)
        .unwrap()
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0, 0), |(horizontal, depth, aim), (cmd, size)| {
            match (cmd, size.parse::<i32>().unwrap()) {
                ("forward", size) => (horizontal + size, depth + aim * size, aim),
                ("down", size) => (horizontal, depth, aim + size),
                ("up", size) => (horizontal, depth, aim - size),
                _ => unreachable!(),
            }
        });
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part_1() {
        let filepath = "./day-2.test";
        let test_input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert!(fs::write(filepath, test_input).is_ok());
        assert_eq!(super::part_1(filepath), 150);
        assert!(fs::remove_file(filepath).is_ok());
    }

    #[test]
    fn part_2() {
        let filepath = "./day-2.test";
        let test_input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        assert!(fs::write(filepath, test_input).is_ok());
        assert_eq!(super::part_2(filepath), 900);
        assert!(fs::remove_file(filepath).is_ok());
    }
}
