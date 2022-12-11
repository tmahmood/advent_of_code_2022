use advent_of_code_2022::{bootstrap, reading_ints_from_line};
use itertools::Itertools;
use std::collections::BTreeMap;

const REAL_DATA: &str = "inputs/d11.real";

fn solve(input: String, inp: Vec<(i32, bool)>) -> Vec<u64> {
    let mut r = Vec::new();
    for (c, d) in inp {
        let mut monkeys = parse_monkeys(input.clone());
        let lcm = monkeys.values().map(|m| m.test).product::<u64>();
        println!("{lcm}");
        let mut inspected = BTreeMap::new();
        let keys = monkeys.keys().copied().collect::<Vec<i32>>();
        for ii in 0..c {
            for mkey in keys.iter() {
                monkey_business(&mut monkeys, *mkey, &mut inspected, d, lcm);
                println!("-------");
            }
            println!("######");
        }
        let v = inspected
            .values()
            .copied()
            .sorted()
            .rev()
            .take(2)
            .collect::<Vec<u64>>();
        r.push(v.iter().product::<u64>())
    }
    r
}

fn main() {
    bootstrap!(vec![(20, true), (10000, false)]);
}

#[derive(Clone, Debug)]
struct Monkey {
    id: i32,
    items: Vec<u64>,
    operation: (String, String),
    test: u64,
    when_true: i32,
    when_false: i32,
}

impl Monkey {
    pub fn take_all(&mut self) -> Vec<u64> {
        let v = self.items.clone();
        self.items.clear();
        v
    }
}

fn monkey_business(
    monkeys: &mut BTreeMap<i32, Monkey>,
    monkey_index: i32,
    inspected: &mut BTreeMap<i32, u64>,
    reduce_worry: bool,
    lcm: u64,
) {
    let mut items = { monkeys.get_mut(&monkey_index).unwrap().take_all() };
    let monkey = { monkeys.get(&monkey_index).unwrap().clone() };
    while !items.is_empty() {
        inspected
            .entry(monkey_index)
            .and_modify(|v| *v += 1)
            .or_insert(1);
        let item = items.pop().unwrap();
        let op_with = if monkey.operation.1 == "old" {
            item
        } else {
            monkey.operation.1.parse::<u64>().unwrap()
        };
        let mut new_level = if monkey.operation.0 == "+" {
            item + op_with
        } else {
            item * op_with
        };

        let new_level = if reduce_worry {
            new_level /= 3;
            new_level
        } else {
            new_level % lcm
        };
        let to_monkey = if new_level % monkey.test == 0 {
            monkey.when_true
        } else {
            monkey.when_false
        };
        monkeys.entry(to_monkey).and_modify(|v| {
            v.items.push(new_level);
        });
    }
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<&str>>();
        let id = reading_ints_from_line(lines.first().unwrap())
            .pop()
            .unwrap();
        let items = reading_ints_from_line::<u64>(lines.get(1).unwrap()).to_vec();
        let operation = lines
            .get(2)
            .unwrap()
            .split_once("old ")
            .map(|(_, v)| {
                let (sign, n) = v.split_once(' ').unwrap();
                (sign.to_string(), n.to_string())
            })
            .unwrap();
        let test = reading_ints_from_line(lines.get(3).unwrap()).pop().unwrap();
        let when_true = reading_ints_from_line(lines.get(4).unwrap()).pop().unwrap();
        let when_false = reading_ints_from_line(lines.get(5).unwrap()).pop().unwrap();
        Monkey {
            id,
            items,
            operation,
            test,
            when_true,
            when_false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_from_file;
    const TEST_DATA: &str = "inputs/d11.test";

    #[test]
    fn test_reading_input() {
        let monkey_string = r#"Monkey 0:
            Starting items: 79, 98
        Operation: new = old * 19
        Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3"#;
        let m = Monkey::from(monkey_string);
        assert_eq!(m.id, 0);
        assert_eq!(m.items, vec![79, 98]);
        assert_eq!(m.operation, ("*".to_string(), "19".to_string()));
        assert_eq!(m.when_true, 2);
        assert_eq!(m.when_false, 3);
    }

    #[test]
    fn test_monkeys() {
        let input = read_from_file(TEST_DATA);
        let p = solve(input, vec![(20, true)]);
        assert_eq!(p.first().unwrap(), &10605u64);
    }

    #[test]
    fn test_monkeys_too_much_worry() {
        let input = read_from_file(TEST_DATA);
        let p = solve(input, vec![(10000, false)]);
        assert_eq!(p.first().unwrap(), &2713310158);
    }
}

pub(crate) fn parse_monkeys(content: String) -> BTreeMap<i32, Monkey> {
    content
        .split("\n\n")
        .enumerate()
        .map(|(ii, v)| (ii as i32, Monkey::from(v)))
        .collect::<BTreeMap<i32, Monkey>>()
}
