use std::fs;

enum Throw {
    Rock = 1,
    Paper,
    Scissors,
}

fn get_throw(letter: char) -> Throw {
    match letter {
        'A' | 'X' => Throw::Rock,
        'B' | 'Y' => Throw::Paper,
        'C' | 'Z' => Throw::Scissors,
        _ => panic!("Invalid throw"),
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn part_one(moves: &str) -> i32 {
    let mut score = 0;
    for mv in moves.split('\n') {
        let elf_move = get_throw(mv.chars().next().unwrap()) as i32;
        let my_move = get_throw(mv.chars().nth(2).unwrap()) as i32;

        score += my_move;

        if elf_move == my_move {
            score += 3;
        } else if my_move - elf_move == 1 || (elf_move == 3 && my_move == 1) {
            score += 6;
        }
    }
    score
}

fn part_two(moves: &str) -> i32 {
    let mut score = 0;
    for mv in moves.split('\n') {
        let elf_move = get_throw(mv.chars().next().unwrap()) as i32;
        let expected_move = mv.chars().nth(2).unwrap();

        let my_move = match expected_move {
            'X' => if elf_move == 1 { 3 } else { elf_move - 1 },
            'Z' => if elf_move == 3 { 1 } else { elf_move + 1 },
            'Y' => elf_move,
            _ => panic!("Invalid throw"),
        };

        score += my_move;

        if elf_move == my_move {
            score += 3;
        } else if my_move - elf_move == 1 || (elf_move == 3 && my_move == 1) {
            score += 6;
        }
    }
    score
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("A Y\nB X\nC Z"), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("A Y\nB X\nC Z"), 12);
    }
}
