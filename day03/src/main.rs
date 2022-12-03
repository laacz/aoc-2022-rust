use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(input: &str) -> i32 {
    let mut score = 0;
    for rs in input.lines() {
        let rs1 = &rs[..rs.len() / 2];
        let rs2 = &rs[rs.len() / 2..];
        'outer: for c1 in rs1.chars() {
            for c2 in rs2.chars() {
                if c1 == c2 {
                    let val = c1 as i32 - 'a' as i32 + 1;
                    if val < 0 {
                        score += c1 as i32 - 'A' as i32 + 27;
                    } else {
                        score += val
                    }
                    break 'outer;
                }
            }
        }
    }
    score
}

fn part_two(input: &str) -> i32 {
    let mut score = 0;
    let mut group = Vec::new();
    for line in input.lines() {
        group.push(line);
        if group.len() == 3 {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    let val = c as i32 - 'a' as i32 + 1;
                    if val < 0 {
                        score += c as i32 - 'A' as i32 + 27;
                    } else {
                        score += val
                    }
                    break;
                }
            }
            group.clear();
        }
    }
    score
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            157
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(
                "vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            70
        );
    }
}
