use crate::Ops::NoOp;
use advent_of_code_2022::{end_it, read_from_file, start_it};

const REAL_DATA: &str = "inputs/d10.real";
const TEST_DATA: &str = "inputs/d10.test";

fn main() {
    let t = start_it();
    let s = process(REAL_DATA);
    println!("{s}");
    end_it(t);
}

fn process(file_name: &str) -> i32 {
    let content = read_from_file(file_name);
    let state = parse_instructions(&content);
    let strength = state
        .cycle_states
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(cycle, x)| (cycle as i32) * x)
        .sum::<i32>();
    println!("{}", state.draw_image());
    strength
}

#[derive(Debug)]
enum Ops {
    Add(i32),
    NoOp,
}

struct RunState {
    cycle_states: Vec<i32>,
    last_op: Option<Ops>,
    x: i32,
}

impl RunState {
    pub(crate) fn draw_image(&self) -> String {
        let mut s = String::new();
        self.cycle_states.chunks(40).for_each(|v| {
            let mut crt_position = 0;
            v.iter().for_each(|jj| {
                if (jj - 1..=jj + 1).contains(&crt_position) {
                    s.push('#');
                } else {
                    s.push('.');
                }
                crt_position += 1;
            });
            s.push('\n');
        });
        s
    }

    fn new() -> Self {
        RunState {
            cycle_states: vec![],
            last_op: None,
            x: 1,
        }
    }

    pub fn add(&mut self, x: i32) {
        self.cycle_states.push(self.x);
        match self.last_op {
            None | Some(NoOp) => self.cycle_states.push(self.x),
            Some(_) => {}
        }
        self.x += x;
        self.cycle_states.push(self.x);
        self.last_op = Some(Ops::Add(x));
    }

    pub fn noop(&mut self) {
        match self.last_op {
            Some(NoOp) | None => self.cycle_states.push(self.x),
            Some(Ops::Add(..)) => {}
        }
        self.last_op = Some(NoOp);
    }

    pub fn state_at(&self, cycle: usize) -> i32 {
        self.cycle_states[cycle - 1]
    }
}

pub(crate) fn parse_instructions(instructions: &str) -> RunState {
    let mut run_state = RunState::new();
    instructions.lines().for_each(|line| {
        if line.starts_with("noop") {
            run_state.noop();
        } else {
            let n = line
                .split_once(' ')
                .map(|v| v.1.parse::<i32>().unwrap())
                .unwrap();
            run_state.add(n);
        }
    });
    run_state
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_from_file;

    #[test]
    fn small_program_from_problem() {
        let instruction = "noop\naddx 3\naddx -5\nnoop\nnoop\naddx 5\naddx 6";
        let run_state = parse_instructions(&instruction);
        assert_eq!(run_state.x, 10);
        assert_eq!(run_state.state_at(1), 1);
        assert_eq!(run_state.state_at(2), 1);
        assert_eq!(run_state.state_at(3), 1);
        assert_eq!(run_state.state_at(4), 4);
        assert_eq!(run_state.state_at(5), 4);
        assert_eq!(run_state.state_at(6), -1);
        assert_eq!(run_state.state_at(7), -1);
        assert_eq!(run_state.state_at(8), -1);
        assert_eq!(run_state.state_at(9), -1);
        assert_eq!(run_state.state_at(10), 4);
        assert_eq!(run_state.state_at(11), 4);
        assert_eq!(run_state.state_at(12), 10);
    }

    #[test]
    fn test_data_process() {
        let instruction = read_from_file(TEST_DATA);
        let state = parse_instructions(&instruction);
        assert_eq!(state.state_at(20), 21);
        assert_eq!(state.state_at(60), 19);
        assert_eq!(state.state_at(100), 18);
        assert_eq!(state.state_at(140), 21);
        assert_eq!(state.state_at(180), 16);
        assert_eq!(state.state_at(220), 18);
    }

    #[test]
    fn test_drawing_image() {
        let r = r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;
        let instruction = read_from_file(TEST_DATA);
        let state = parse_instructions(&instruction);
        let drawn_str = state.draw_image();
        assert_eq!(drawn_str, r);
    }
}
