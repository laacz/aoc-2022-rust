use std::ops::RangeInclusive;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn range_pair_from_str(s: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let mut split = s.split(',');
    let mut rstr = split.next().unwrap().split('-');
    let r1 =
        rstr.next().unwrap().parse::<i32>().unwrap()..=rstr.next().unwrap().parse::<i32>().unwrap();

    rstr = split.next().unwrap().split('-');
    let r2 =
        rstr.next().unwrap().parse::<i32>().unwrap()..=rstr.next().unwrap().parse::<i32>().unwrap();

    (r1, r2)
}

fn part_one(input: &str) -> i32 {
    let mut score = 0;
    for rs in input.lines() {
        let (r1, r2) = range_pair_from_str(rs);
        if (r1.contains(r2.start()) && r1.contains(r2.end())) ||
            (r2.contains(r1.start()) && r2.contains(r1.end())) {
            score += 1;
        }
    }
    score
}

fn part_two(input: &str) -> i32 {
    let mut score = 0;
    for rs in input.lines() {
        let (r1, r2) = range_pair_from_str(rs);
        if r1.contains(r2.start()) || r1.contains(r2.end()) ||
            r2.contains(r1.start()) || r2.contains(r1.end()) {
            score += 1;
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
                "2-4,6-8\n\
                2-3,4-5\n\
                5-7,7-9\n\
                2-8,3-7\n\
                6-6,4-6\n\
                2-6,4-8"
            ),
            2
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(
                "2-4,6-8\n\
                2-3,4-5\n\
                5-7,7-9\n\
                2-8,3-7\n\
                6-6,4-6\n\
                2-6,4-8"
            ),
            4
        );
    }
}
