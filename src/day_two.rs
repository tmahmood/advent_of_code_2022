use crate::read_from_file;

const REAL_INPUT: &str = "inputs/day_two/input";
const TEST_INPUT: &str = "inputs/day_two/test";
const A: i32 = 1;
const B: i32 = 2;
const C: i32 = 3;

fn score(char: &str) -> i32 {
    // "X > Y > Z > X"
    //  1 > 2 > 3 > 1
    // Rock defeats Scissors, AX > ZC
    // Paper defeats Rock     BY > XA
    // Scissors defeats Paper CZ > YB
    match char {
        "A Y" => 6 + B,
        "B X" => 0 + A,
        "C Z" => 3 + C,

        "A Z" => 0 + C,
        "B Y" => 3 + B,
        "C X" => 6 + A,

        "A X" => 3 + A,
        "B Z" => 6 + C,
        "C Y" => 0 + B,
        _ => 0
    }
}

fn score_2nd_part(char: &str) -> i32 {
    // Rock defeats Scissors, A > C
    // Paper defeats Rock     B > A
    // Scissors defeats Paper C > B
    // "X > Y > Z > X"
    //  1 > 2 > 3 > 1
    // the second column says how the round needs to end:
    // X means you need to lose,
    // Y means you need to end the round in a draw, and
    // Z means you need to win
    match char {
        "A Y" => 3 + A,
        "B X" => 0 + A,
        "C Z" => 6 + A,

        "A Z" => 6 + B,
        "B Y" => 3 + B,
        "C X" => 0 + B,

        "A X" => 0 + C,
        "B Z" => 6 + C,
        "C Y" => 3 + C,
        _ => 0
    }
}

fn follow_strategy(file_name: &str) -> i32 {
    let content = read_from_file(file_name);
    let content = content.trim();
    let mut my_score = 0;
    content.lines().for_each(|v: &str| {
        my_score += score_2nd_part(v);
        println!("{} => {}", v, my_score);
    });
    my_score
}

fn main() {
    let k = follow_strategy(REAL_INPUT);
    println!("{}", k);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_follow_strategy_party_two() {
        let total_score = follow_strategy(REAL_INPUT);
        assert_eq!(total_score, 12)
    }
}


