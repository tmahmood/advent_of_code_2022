use itertools::Itertools;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub fn read_from_file(input: &str) -> String {
    let mut content = String::new();
    let mut file = File::open(input).unwrap();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn reading_lines_to_int(input: &str) {
    let input: Vec<(usize, usize, usize, usize)> = read_from_file(input)
        .lines()
        .map(|l| {
            l.split(['-', ','])
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap()
        })
        .filter(|(s1, e1, s2, e2)| (s1 <= s2 && e1 >= e2) || (s2 <= s1 && e2 >= e1))
        .collect();
}

pub fn start_it() -> Instant {
    Instant::now()
}

pub fn end_it(start: Instant) {
    let duration = start.elapsed();
    println!("Finished in {duration:?}");
}
