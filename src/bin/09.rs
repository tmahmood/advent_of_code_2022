use advent_of_code_2022::{end_it, read_from_file, start_it};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

const REAL_DATA: &str = "inputs/d9.real";
const TEST_DATA: &str = "inputs/d9.test";
const TEST_DATA_L: &str = "inputs/d9.2.test";

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, dir: &str) {
        match dir {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y -= 1,
            "D" => self.y += 1,
            _ => {}
        }
    }

    fn translate(&mut self, dir: &Point) {
        self.x += dir.x;
        self.y += dir.y;
    }
}

fn dist(p1: &Point, p2: &Point) -> i32 {
    (((p2.x - p1.x).pow(2) + (p2.y - p1.y).pow(2)) as f64).sqrt() as i32
}

fn comp(val: &i32) -> i32 {
    match 0.cmp(val) {
        Ordering::Greater => -1,
        Ordering::Less => 1,
        Ordering::Equal => 0,
    }
}

fn direction(p: &Point, target: &Point) -> Point {
    let x = target.x - p.x;
    let y = target.y - p.y;
    let x = comp(&x);
    let y = comp(&y);
    Point::new(x, y)
}

fn main() {
    let timer = start_it();
    let moves = read_from_file(REAL_DATA);
    let unique_moves = follow_head(moves.clone(), 2);
    println!("{unique_moves}");
    let unique_moves = follow_head(moves, 10);
    println!("{unique_moves}");
    end_it(timer);
}

fn follow_head(moves: String, keep_track: i32) -> usize {
    let mut knots = vec![Point::new(0, 0); keep_track as usize];
    let mut went_to = HashSet::new();
    went_to.insert(knots.last().unwrap().clone());
    moves
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(p1, c)| (p1, c.parse::<i32>().unwrap()))
                .unwrap()
        })
        .for_each(|(p1, c)| {
            for _ in 0..c {
                knots.get_mut(0).unwrap().move_to(p1);
                for ii in 1..keep_track as usize {
                    let last = knots[ii - 1].clone();
                    let current = &mut knots[ii];
                    if dist(&last, current) > 1 {
                        let next = direction(current, &last);
                        current.translate(&next);
                        if ii == (keep_track - 1) as usize {
                            went_to.insert(knots.last().unwrap().clone());
                        }
                    }
                }
            }
        });
    went_to.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_direction() {
        let a = Point::new(4, -2);
        let b = Point::new(3, 0);
        let dir = direction(&b, &a);
        assert_eq!(dir, Point::new(1, -1));

        let a = Point::new(4, -3);
        let b = Point::new(4, -1);
        let dir = direction(&b, &a);
        assert_eq!(dir, Point::new(0, -1));
    }

    #[test]
    fn test_grid_walking() {
        let moves = read_from_file(TEST_DATA);
        let positions = follow_head(moves, 2);
        assert_eq!(positions, 13);

        let moves = read_from_file(TEST_DATA_L);
        let positions = follow_head(moves, 10);
        assert_eq!(positions, 36);

        let moves = read_from_file(REAL_DATA);
        let positions = follow_head(moves, 10);
        assert_eq!(positions, 2630);
    }

    #[test]
    fn test_dist() {
        let p1 = Point::new(4, 0);
        let p2 = Point::new(4, -1);
        assert_eq!(dist(&p1, &p2), 1);
    }

    #[test]
    fn test_moving_head() {
        let mut head = Point::new(0, 0);
        head.move_to("R");
        assert_eq!(head, Point::new(1, 0))
    }
}
