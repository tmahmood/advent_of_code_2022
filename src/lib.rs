use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::time::Instant;

#[macro_export]
macro_rules! bootstrap {
    ($inp: expr) => {
        use advent_of_code_2022::{end_it, read_from_file, start_it};
        let content = read_from_file(REAL_DATA);
        let start = start_it();
        let r = solve(content, $inp);
        println!("Result is: {:?}", r);
        end_it(start);
    };
}

pub fn read_from_file(input: &str) -> String {
    let mut content = String::new();
    let mut file = File::open(input).unwrap();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn reading_ints_from_line<T: FromStr>(input: &str) -> Vec<T> {
    input
        .split(&[',', ':', ' '])
        .filter_map(|v| v.parse::<T>().ok())
        .collect()
}

#[test]
fn test_reading_ints_from_line() {
    let v = reading_ints_from_line::<i32>("Starting items: 79, 98");
    assert_eq!(v, vec![79, 98]);
    let v = reading_ints_from_line::<i32>("Monkey 0:");
    assert_eq!(v, vec![0]);
}

pub fn start_it() -> Instant {
    Instant::now()
}

pub fn end_it(start: Instant) {
    let duration = start.elapsed();
    println!("Finished in {duration:?}");
}
