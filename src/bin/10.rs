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
    let mut instructions = Instructions::new(&content);
    let state = instructions.compute();
    let mut strength: i32 = 0;
    for cycle in (20..=220).step_by(40) {
        strength += state.state_at(cycle) * cycle as i32;
    }
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
        self.cycle_states
            .chunks(40)
            .enumerate()
            .for_each(|(ii, v)| {
                let mut crt_position = 0;
                v.iter().for_each(|jj| {
                    let sprite_pos = (jj - 1..=jj + 1);
                    if sprite_pos.contains(&crt_position) {
                        s.push_str("#");
                    } else {
                        s.push_str(".");
                    }
                    crt_position += 1;
                });
                s.push('\n');
            });
        s
    }
}

impl RunState {
    fn new() -> Self {
        RunState {
            cycle_states: vec![],
            last_op: None,
            x: 1,
        }
    }

    pub fn add(&mut self, x: i32) {
        let mut cycles = if let Some(Ops::Add(..)) = self.last_op {
            vec![self.x; 1]
        } else {
            vec![self.x; 2]
        };
        self.x += x;
        cycles.push(self.x);
        self.cycle_states.extend(cycles);
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

struct Instructions {
    op_queue: Vec<Ops>,
}

impl Instructions {
    pub(crate) fn add_x(&mut self, add: i32) {
        self.op_queue.push(Ops::Add(add));
    }

    pub(crate) fn noop(&mut self) {
        self.op_queue.push(Ops::NoOp);
    }
}

impl Instructions {
    pub(crate) fn new(instructions: &str) -> Self {
        let mut ii = Instructions { op_queue: vec![] };
        instructions.lines().for_each(|line| {
            if line.starts_with("noop") {
                ii.noop();
            } else {
                let n = line
                    .split_once(' ')
                    .map(|v| v.1.parse::<i32>().unwrap())
                    .unwrap();
                ii.add_x(n);
            }
        });
        ii
    }

    pub fn compute(&mut self) -> RunState {
        let mut run_state = RunState::new();
        self.op_queue.iter().enumerate().for_each(|(_ii, op)| {
            match op {
                Ops::Add(x) => run_state.add(*x),
                Ops::NoOp => run_state.noop(),
            };
        });
        run_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_from_file;

    #[test]
    fn small_program_from_problem() {
        let instruction = "noop\naddx 3\naddx -5\nnoop\nnoop\naddx 5\naddx 6";
        let mut computer = Instructions::new(&instruction);
        println!("{:?}", computer.op_queue);
        let run_state = computer.compute();
        assert_eq!(run_state.x, 10);
        println!("{:?}", run_state.cycle_states);
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
        let mut computer = Instructions::new(&instruction);
        let state = computer.compute();
        state.cycle_states.chunks(10).enumerate().for_each(|v| {
            println!("{v:3?}");
        });
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
        let mut computer = Instructions::new(&instruction);
        let state = computer.compute();
        let drawn_str = state.draw_image();
        assert_eq!(drawn_str, r);
    }
}
