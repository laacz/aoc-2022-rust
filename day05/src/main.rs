use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn parse_input(input: &str) -> (Vec<String>, Vec<Vec<usize>>) {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();

    let mut max_len = 0;
    for line in stacks_input.lines() {
        if max_len < line.len() / 4 {
            max_len = line.len() / 4;
        }
    }

    let mut stacks: Vec<String> = vec![String::from(""); max_len + 1];
    let mut moves = Vec::new();

    for line in stacks_input.lines() {
        let mut pos = 0;
        if !line.contains('[') {
            continue;
        }
        while pos < line.len() {
            let ch = line.chars().nth(pos + 1).unwrap();
            if ch != ' ' {
                stacks[pos / 4].push(ch);
            }
            pos += 4;
        }
    }

    for line in moves_input.lines() {
        let mut move_vec = Vec::new();
        for candidate in line.split(" ") {
            if let Ok(num) = candidate.parse::<usize>() {
                move_vec.push(num);
            }
        }
        moves.push(move_vec);
    }

    (stacks, moves)
}

fn part_one(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    for mov in moves {
        for _ in 0..mov[0] {
            let from = mov[1] - 1;
            let to = mov[2] - 1;
            let ch = stacks[from].chars().next().unwrap();
            stacks[from].remove(0);
            stacks[to].insert(0, ch);
        }
    }

    let mut ret = String::from("");
    for stack in stacks {
        ret.push(stack.chars().next().unwrap())
    }

    ret
}

fn part_two(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    for mov in moves {
        for i in 0..mov[0] {
            let from = mov[1] - 1;
            let to = mov[2] - 1;
            let ch = stacks[from].chars().nth(mov[0] - i - 1).unwrap();
            stacks[from].remove(mov[0] - i - 1);
            stacks[to].insert(0, ch);
        }
    }

    let mut ret = String::from("");
    for stack in stacks {
        ret.push(stack.chars().next().unwrap())
    }

    ret
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one(
                "    [D]\n\
                [N] [C]\n\
                [Z] [M] [P]\n\
                 1   2   3\n\
                \n\
                move 1 from 2 to 1\n\
                move 3 from 1 to 3\n\
                move 2 from 2 to 1\n\
                move 1 from 1 to 2"
            ),
            "CMZ"
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(
                "    [D]\n\
                [N] [C]\n\
                [Z] [M] [P]\n\
                 1   2   3\n\
                \n\
                move 1 from 2 to 1\n\
                move 3 from 1 to 3\n\
                move 2 from 2 to 1\n\
                move 1 from 1 to 2"
            ),
            "MCD"
        );
    }
}
