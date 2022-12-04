use std::collections::HashMap;
use lazy_static::lazy_static;
use advent_of_code_2022::read_from_file;

const REAL_INPUT: &str = "inputs/day_three/input";
const TEST_INPUT: &str = "inputs/day_three/test";

lazy_static! {
    static ref HASHMAP: HashMap<String, usize> = {
        let c = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut m = HashMap::new();
        for (i, c) in c.char_indices() {
            m.insert(c.to_string(), i + 1);
        }
        m
    };
}

fn main() {
    let a1 = calculate_total_priorities(TEST_INPUT);
    let a2 = calculate_total_priorities(REAL_INPUT);
    println!("{} {}", a1, a2);

}

fn sort_and_dedup(a: &str) -> Vec<usize> {
    let mut v = a.split("")
        .filter(|c| c != &"")
        .map(|c| *HASHMAP.get(&c.to_string()).unwrap())
        .collect::<Vec<usize>>();
    v.sort();
    v.dedup();
    v
}

fn calculate_total_priorities(input: &str) -> i32 {
    let content = read_from_file(input);
    let mut total_priorities = 0;
    content.lines().for_each(|line: &str| {
        let (first, second) = line.split_at((line.len() / 2));
        let f = sort_and_dedup(first);
        let s = sort_and_dedup(second);
        let mut calc = HashMap::new();
        update_count(&mut calc, &f, 2);
        let m = update_count(&mut calc, &s, 2);
        total_priorities += m;
    });
    total_priorities as i32
}

fn calculate_total_group_priorities(file_name: &str) -> i32 {
    let content = read_from_file(file_name);
    let lines = content.lines().collect::<Vec<&str>>();
    let mut total_priorities = 0;
    for ii in (0..lines.len()).step_by(3) {
        let [l1, l2, l3] = lines[ii..ii + 3] else { todo!() };
        println!("{} {} {}", l1, l2, l3);
        let l1 = sort_and_dedup(l1);
        let l2 = sort_and_dedup(l2);
        let l3 = sort_and_dedup(l3);
        let mut calc = HashMap::new();
        update_count(&mut calc, &l1, 3);
        update_count(&mut calc, &l2, 3);
        let m = update_count(&mut calc, &l3, 3);
        total_priorities += m;

    }
    total_priorities as i32
}

fn update_count(calc: &mut HashMap<usize, usize>, f: &Vec<usize>, range: usize) -> usize {
    let mut more_than_one = 0;
    f.iter().for_each(|v| {
        calc.entry(*v)
            .and_modify(|n| {
                *n += 1;
                if *n >= range {
                    more_than_one = *v;
                }
            })
            .or_insert(1);
    });
    more_than_one
}

#[cfg(test)]
mod tests {
    use crate::read_from_file;
    use super::*;

    #[test]
    fn calculate_sum_of_the_priorities() {
        let k = calculate_total_priorities(TEST_INPUT);
        assert_eq!(157, k);
        let k = calculate_total_priorities(REAL_INPUT);
        assert_eq!(8401, k)
    }

    #[test]
    fn calculate_sum_of_priorities_for_groups() {
        let k = calculate_total_group_priorities(TEST_INPUT);
        assert_eq!(70, k);
        let k = calculate_total_group_priorities(REAL_INPUT);
        assert_eq!(8401, k)
    }
}