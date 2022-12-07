use advent_of_code_2022::read_from_file;
use itertools::Itertools;
use std::collections::HashMap;

const TEST_INPUT: &str = "inputs/d7.test";
const REAL_INPUT: &str = "inputs/d7.real";

fn main() {
    let content = read_from_file(REAL_INPUT);
    let hash_map = parse_commands(content);
    println!("{}", calculate_dir_sizes(&hash_map));
    println!("{}", find_smallest_d_to_delete(&hash_map));
}

fn calculate_dir_sizes(hash_map: &HashMap<String, Vec<i32>>) -> i32 {
    hash_map
        .values()
        .map(|v| v.iter().sum::<i32>())
        .filter(|v| v <= &100000)
        .sum()
}

fn find_smallest_d_to_delete(hash_map: &HashMap<String, Vec<i32>>) -> i32 {
    let mut v = hash_map
        .values()
        .map(|k| k.iter().sum())
        .sorted()
        .collect::<Vec<i32>>();
    let used = v.pop().unwrap();
    for (k, v) in hash_map.iter() {
        let sum = v.iter().sum::<i32>();
        if 70000000 - used + sum > 30000000 {
            return sum;
        }
    }
    0
}

fn parse_i32(cmd: &str) -> i32 {
    cmd.split_once(' ').unwrap().0.parse::<i32>().unwrap()
}

fn update_hash_map(hash_map: &mut HashMap<String, Vec<i32>>, cur_dir: &str, size: i32) {
    hash_map
        .entry(cur_dir.to_string())
        .and_modify(|v| v.push(size))
        .or_insert(vec![size]);
}

fn parse_commands(commands: String) -> HashMap<String, Vec<i32>> {
    let mut stack = vec![];
    let mut cur_dir = String::from("");
    let mut hash_map: HashMap<String, Vec<i32>> = HashMap::new();
    let binding = commands
        .lines()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let mut lines = binding.iter();
    loop {
        let cmd_opt = lines.next();
        let cmd = if let Some(..) = cmd_opt {
            cmd_opt.unwrap()
        } else {
            "$ cd .."
        }
        .to_string();
        if cmd.starts_with("$ cd ..") {
            stack.pop();
            let size = hash_map.get(&cur_dir).unwrap().iter().sum();
            cur_dir = stack.join("/").replace("//", "/");
            update_hash_map(&mut hash_map, &cur_dir, size);
        } else if cmd.starts_with("$ cd") {
            stack.push(cmd.split(' ').skip(2).join(""));
        } else if cmd.starts_with("$ ls") {
            cur_dir = stack.join("/").replace("//", "/");
        } else if cmd.starts_with("dir") {
            continue;
        } else {
            let size = parse_i32(&cmd);
            update_hash_map(&mut hash_map, &cur_dir, size);
        }
        if stack.is_empty() {
            break;
        }
    }
    hash_map.remove("");
    hash_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_from_file;

    #[test]
    fn test_parsing_commands() {
        let content = read_from_file(TEST_INPUT);
        let result = parse_commands(content);
        assert_eq!(95437, calculate_dir_sizes(&result));
        let content = read_from_file(REAL_INPUT);
        let result = parse_commands(content);
        assert_eq!(1555642, calculate_dir_sizes(&result));
    }

    #[test]
    fn test_which_directory_to_delete() {
        let content = read_from_file(TEST_INPUT);
        let hash_map = parse_commands(content);
        let d = find_smallest_d_to_delete(&hash_map);
        assert_eq!(d, 24933642);
    }
}
