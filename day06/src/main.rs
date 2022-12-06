use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn is_distinct(input: &str) -> bool {
    let mut uniq = HashSet::new();
    input.chars().into_iter().all(move |x| uniq.insert(x))
}

fn part_one(input: &str) -> usize {
    for i in 3..input.len() {
        if is_distinct(&input[i - 3..=i]) {
            return i + 1;
        }
    }

    0
}

fn part_two(input: &str) -> usize {
    for i in 13..input.len() {
        if is_distinct(&input[i - 13..=i]) {
            return i + 1;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let tests = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for test in tests {
            assert_eq!(part_one(test.0), test.1);
        }
    }

    #[test]
    fn test_part_two() {
        let tests = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for test in tests {
            assert_eq!(part_two(test.0), test.1);
        }
    }
}
