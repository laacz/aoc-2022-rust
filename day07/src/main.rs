use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

fn build_map(input: &str) -> HashMap<String, usize> {
    let mut result = HashMap::new();

    let mut curr_vec: Vec<String> = Vec::new();
    let mut curr = String::from("/");
    for line in input.lines() {
        if line.starts_with("$ cd ") {
            let dir = String::from(line.split(' ').nth(2).unwrap());

            match dir.as_str() {
                "/" => {
                    curr_vec.clear();
                }
                ".." => {
                    curr_vec.pop();
                }
                _ => {
                    curr_vec.push(dir.clone());
                }
            };
            curr = curr_vec.join("/");

            if !curr.starts_with('/') {
                curr = format!("/{}", curr);
            }

            result.entry(curr.clone()).or_insert(0);
        } else if line.chars().next().unwrap().is_ascii_digit() {
            let size = line.split(' ').next().unwrap();
            let size = size.parse::<usize>().unwrap();

            for (key, value) in result.iter_mut() {
                if curr.starts_with(key) {
                    *value += size;
                }
            }
        }
    }

    result
}

fn part_one(input: &str) -> usize {
    let map = build_map(input);
    let mut result : usize = 0;
    
    for (_key, value) in map.iter() {
        if *value <= 100000 {
            result += value;
        }
    }

    result
}

fn part_two(input: &str) -> usize {
    let map = build_map(input);
    let required = 30000000 - (70000000 - map["/"]);
    
    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_by_key(|k| k.1);

    for (_key, value) in sorted.iter() {
        if value >= &&required {
            return **value;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::{part_one, part_two};

    static INPUT: &str = "$ cd /\n\
    $ ls\n\
    dir a\n\
    14848514 b.txt\n\
    8504156 c.dat\n\
    dir d\n\
    $ cd a\n\
    $ ls\n\
    dir e\n\
    29116 f\n\
    2557 g\n\
    62596 h.lst\n\
    $ cd e\n\
    $ ls\n\
    584 i\n\
    $ cd ..\n\
    $ cd ..\n\
    $ cd d\n\
    $ ls\n\
    4060174 j\n\
    8033020 d.log\n\
    5626152 d.ext\n\
    7214296 k";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 95437);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 24933642);
    }
}
