#![feature(slice_take)]

use itertools::Itertools;
use advent_of_code_2022::read_from_file;

const REAL_INPUT: &str = "inputs/d5.real";
const TEST_INPUT: &str = "inputs/d5.test";

fn main() {
    let content = read_from_file(REAL_INPUT);
    let (stacks, commands) = content.split_once("\n\n").unwrap();
    let commands: Vec<(usize, usize, usize)> = parse_commands(commands);
    let mut stack = parse_stacks(stacks);
    for command in commands {
        crate_map_9001(&mut stack, command)
    }
    let v = stack.iter().map(|v| v.last().unwrap().clone()).collect::<Vec<String>>();
    let k = v.join(&"")
        .replace("]", "")
        .replace("[", "");
}


fn crate_map_9000(map: &mut Vec<Vec<String>>, (how_many, from_col_offset, to_col_offset): (usize, usize, usize)) {
    let from_col = from_col_offset - 1;
    let to_col = to_col_offset - 1;
    for _ii in 0..how_many {
        let c = map[from_col].pop().unwrap();
        map[to_col].push(c);
    }
}

fn crate_map_9001(map: &mut Vec<Vec<String>>, (how_many, from_col_offset, to_col_offset): (usize, usize, usize)) {
    let from_col = from_col_offset - 1;
    let to_col = to_col_offset - 1;
    let r = map[from_col].len() - how_many;
    let (mut m1, mut m2) = {
        let (m1, m2) = map[from_col].split_at(r);
        (m1.to_vec(), m2.to_vec())
    };
    map[from_col] = m1.to_vec();
    map[to_col].extend(m2.to_vec());
}

fn parse_stacks(stack: &str) -> Vec<Vec<String>> {
    let air_crated = stack
        .replace("    ", "[_]")
        .replace(" ", "")
        .replace("][", "],[")
        ;
    let mut lines = air_crated.lines().collect::<Vec<&str>>();
    lines.pop();
    let mut l = lines.iter().map(|line| {
        let l = line.to_string();
        l.split(",").map(|v| v.to_string()).collect::<Vec<String>>()
    }).collect::<Vec<Vec<String>>>();
    let total_columns = l[0].len();
    let mut out: Vec<Vec<String>> = vec![];
    for kk in 0..total_columns {
        out.push(vec![]);
        for ii in (0..l.len()).rev() {
            if l[ii][kk] != "[_]" {
                out[kk].push(l[ii][kk].clone())
            }
        }
    }
    out
}

fn parse_commands(commands: &str) -> Vec<(usize, usize, usize)> {
    commands.lines().map(|line: &str| {
        line.replace("move", ",")
            .replace("from", ",")
            .replace("to", ",")
            .split(",")
            .filter(|v| v != &"")
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| v.trim().parse::<usize>().unwrap())
            .collect_tuple().unwrap()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_commands() {
        let content = read_from_file(TEST_INPUT);
        let (stacks, commands) = content.split_once("\n\n").unwrap();
        let commands: Vec<(usize, usize, usize)> = parse_commands(commands);
        assert_eq!(commands,
                   vec![
                       (1, 2, 1),
                       (3, 1, 3),
                       (2, 2, 1),
                       (1, 1, 2),
                   ]
        );
    }

    #[test]
    fn test_parse_stacks() {
        let content = read_from_file(TEST_INPUT);
        let (stacks, commands) = content.split_once("\n\n").unwrap();
        let r: Vec<Vec<String>> = parse_stacks(stacks);
        let t = vec![
            vec!["[Z]", "[N]"],
            vec!["[M]", "[C]", "[D]"],
            vec!["[P]"],
        ];
        assert_eq!(r, t)
    }

    #[test]
    fn test_mapping_coordinates() {
        let mut t = vec![
            vec!["[Z]".to_string(), "[N]".to_string()],
            vec!["[M]".to_string(), "[C]".to_string(), "[D]".to_string()],
            vec!["[P]".to_string()],
        ];
        let mut r = vec![
            vec!["[C]".to_string()],
            vec!["[M]".to_string()],
            vec!["[P]".to_string(), "[D]".to_string(), "[N]".to_string(), "[Z]".to_string()],
        ];
        crate_map_9000(&mut t, (1, 2, 1));
        crate_map_9000(&mut t, (3, 1, 3));
        crate_map_9000(&mut t, (2, 2, 1));
        crate_map_9000(&mut t, (1, 1, 2));
        assert_eq!(t, r)
    }

    #[test]
    fn test_mapping_coordinates_9001() {
        let mut t = vec![
            vec!["[Z]".to_string(), "[N]".to_string()],
            vec!["[M]".to_string(), "[C]".to_string(), "[D]".to_string()],
            vec!["[P]".to_string()],
        ];
        let mut r = vec![
            vec!["[M]".to_string()],
            vec!["[C]".to_string()],
            vec!["[P]".to_string(), "[Z]".to_string(), "[N]".to_string(), "[D]".to_string()],
        ];
        crate_map_9001(&mut t, (1, 2, 1));
        crate_map_9001(&mut t, (3, 1, 3));
        crate_map_9001(&mut t, (2, 2, 1));
        crate_map_9001(&mut t, (1, 1, 2));
        assert_eq!(t, r)
    }
}