pub fn part_1(filepath: &str) -> u32 {
    let input = std::fs::read_to_string(filepath).unwrap();
    // Get the number of '1's at each index and number of lines
    let (bit_freq, len): (Vec<usize>, usize) = input
        .lines()
        .map(|line| (u32::from_str_radix(line, 2).unwrap(), line.len()))
        .enumerate()
        .fold((Vec::new(), 0), |(mut bit_freq, _), (i, (num, len))| {
            if len >= bit_freq.len() {
                bit_freq.resize(len, 0)
            }
            for i in 0..len {
                if (num >> i) & 1 == 1 {
                    bit_freq[i] += 1;
                }
            }
            (bit_freq, i + 1)
        });

    // Create the number with the most popular bit selection
    let (gr, er): (u32, u32) =
        bit_freq
            .iter()
            .enumerate()
            .fold((0, 0), |(mut gr, mut er), (i, b)| {
                if b > &(len / 2) {
                    gr |= 1 << i;
                } else {
                    er |= 1 << i;
                }
                (gr, er)
            });
    gr * er
}

fn get_oxygen_rate(numbers: Vec<(u32, usize)>) -> u32 {
    let num_length = numbers[0].1;
    let mut current_numbers = numbers;

    // For each bit
    for bit_pos in (0..num_length).rev() {
        let mut no_of_1s = 0;
        let mut no_of_0s = 0;

        current_numbers.iter().for_each(|(num, _)| {
            if (num >> bit_pos) & 1 == 1 {
                no_of_1s += 1
            } else {
                no_of_0s += 1
            }
        });

        let current_popular_bit = if no_of_1s >= no_of_0s { 1 } else { 0 };

        current_numbers = current_numbers
            .into_iter()
            .filter(|(num, _)| (num >> bit_pos) & 1 == current_popular_bit)
            .collect();

        if current_numbers.len() == 1 {
            return current_numbers[0].0;
        }
    }
    0
}

fn get_co2_rate(numbers: Vec<(u32, usize)>) -> u32 {
    let num_length = numbers[0].1;
    let mut current_numbers = numbers;

    // For each bit
    for bit_pos in (0..num_length).rev() {
        let mut no_of_1s = 0;
        let mut no_of_0s = 0;

        current_numbers.iter().for_each(|(num, _)| {
            if (num >> bit_pos) & 1 == 1 {
                no_of_1s += 1
            } else {
                no_of_0s += 1
            }
        });

        let current_popular_bit = if no_of_1s < no_of_0s { 1 } else { 0 };

        current_numbers = current_numbers
            .into_iter()
            .filter(|(num, _)| (num >> bit_pos) & 1 == current_popular_bit)
            .collect();

        if current_numbers.len() == 1 {
            return current_numbers[0].0;
        }
    }
    0
}

pub fn part_2(filepath: &str) -> u32 {
    let input = std::fs::read_to_string(filepath).unwrap();
    let numbers: Vec<(u32, usize)> = input
        .lines()
        .map(|line| (u32::from_str_radix(line, 2).unwrap(), line.len()))
        .collect();

    get_oxygen_rate(numbers.clone()) * get_co2_rate(numbers.clone())
}

#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn part_1() {
        let filepath = "./day-3.test";
        let test_input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert!(fs::write(filepath, test_input).is_ok());
        assert_eq!(super::part_1(filepath), 198);
        assert!(fs::remove_file(filepath).is_ok());
    }

    #[test]
    fn part_2() {
        let filepath = "./day-3.test";
        let test_input =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert!(fs::write(filepath, test_input).is_ok());
        assert_eq!(super::part_2(filepath), 230);
        assert!(fs::remove_file(filepath).is_ok());
    }
}
