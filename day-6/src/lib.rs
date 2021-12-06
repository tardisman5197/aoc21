pub fn part_1(filepath: &str) -> u32 {
    0
}

pub fn part_2(filepath: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part_1() {
        let filepath = "./day-6.test";
        let test_input = "3,4,3,1,2";
        assert!(fs::write(filepath, test_input).is_ok());
        assert_eq!(super::part_1(filepath), 5934);
        assert!(fs::remove_file(filepath).is_ok());
    }

    // #[test]
    // fn part_2() {
    //     let filepath = "./day-1.test";
    //     let test_input =
    //         "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
    //     assert!(fs::write(filepath, test_input).is_ok());
    //     assert_eq!(super::part_2(filepath), 230);
    //     assert!(fs::remove_file(filepath).is_ok());
    // }
}
