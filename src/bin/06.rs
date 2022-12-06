use std::collections::HashSet;
use advent_of_code_2022::read_from_file;

const REAL_INPUT: &str = "inputs/d6.real";

fn main() {
    let input = read_from_file(REAL_INPUT);
    let k = find_marker(input.clone(), 4);
    println!("{}", k);
    let k = find_marker(input, 14);
    println!("{}", k);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finding_first_marker_char() {
        assert_eq!(super::find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4), 5);
        assert_eq!(super::find_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4), 6);
        assert_eq!(super::find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4), 10);
        assert_eq!(super::find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4), 11);
        let input = read_from_file(REAL_INPUT);
        assert_eq!(find_marker(input, 4), 1647);
    }
}

pub(crate) fn find_marker(line: String, marker_length: usize) -> usize {
    let m = marker_length - 1;
    let chars = line.split("").filter(|c| c != &"").collect::<Vec<&str>>();
    for k in m..chars.len() {
        let recent = chars[(k-m)..=k].to_vec();
        let l: HashSet<&str> = HashSet::from_iter(recent);
        if l.len() == marker_length {
            return k + 1;
        }
    }
    0
}