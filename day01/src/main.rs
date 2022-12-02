use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let elves = parse_input(input);
    println!("Part 1: {}", part_one(&elves));
    println!("Part 2: {}", part_two(&elves));
}

fn parse_input(input: String) -> Vec<usize> {
    let mut elves = Vec::new();
    for line in input.split("\n\n") {
        let mut elf = 0;
        for line in line.lines() {
            elf += line.parse::<usize>().unwrap();
        }
        elves.push(elf);
    }
    elves.sort_by(|a, b| b.cmp(a));
    elves
}

fn part_one(elves: &[usize]) -> usize {
    elves[0]
}

fn part_two(elves: &[usize]) -> usize {
    elves[0] + elves[1] + elves[2]
}

#[cfg(test)]
mod tests {
    use super::{parse_input, part_one, part_two};

    fn get_elves() -> Vec<usize> {
        let input =
            String::from("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000");
        parse_input(input)
    }

    #[test]
    fn test_parse_input() {
        let elves = get_elves();
        assert_eq!(elves, vec![24000, 11000, 10000, 6000, 4000]);
    }

    #[test]
    fn test_part_one() {
        let elves = get_elves();
        assert_eq!(part_one(&elves), 24000);
    }

    #[test]
    fn test_part_two() {
        let elves = get_elves();
        assert_eq!(part_two(&elves), 45000);
    }
}
