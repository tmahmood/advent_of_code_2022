use std::fs::File;
use std::io::Read;
use crate::read_from_file;

fn get_elf_with_maximum_calories(input: &str) -> i32 {
    let content = read_from_file(input);
    let lines = content.split("\n").collect::<Vec<&str>>();
    let mut elf = 0;
    let mut max_calories = 0;
    let mut elvs = vec![];
    lines.iter().for_each(|line| {
        if line == &"" {
            elvs.push(elf);
            if elf > max_calories { max_calories = elf }
            elf = 0;
        } else {
            let n = line.parse::<i32>();
            elf += n.unwrap();
        }
    });
    elvs.sort();
    println!("{}", &elvs[elvs.len() - 3..elvs.len()].iter().sum::<i32>());
    max_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_data() {
        let i: i32 = get_elf_with_maximum_calories("inputs/day_one/test");
        assert_eq!(24000, i);
    }

    #[test]
    fn calculate_first_three() {
        let i: i32 = get_elf_with_maximum_calories("inputs/day_one/test");
        assert_eq!(24000, i);
    }
}
