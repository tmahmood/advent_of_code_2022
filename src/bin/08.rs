use advent_of_code_2022::{end_it, read_from_file, start_it};
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

const REAL_DATA: &str = "inputs/d8.real";
const TEST_DATA: &str = "inputs/d8.test";

struct Grid {
    grid: Vec<usize>,
    size: usize,
}

impl Grid {
    pub fn new(content: String) -> Self {
        let grid = content
            .replace('\n', "")
            .chars()
            .map(|v| v.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        let size = content.lines().count();
        Self { grid, size }
    }

    pub fn count_sides(&self) -> usize {
        4 * self.size - 4
    }

    pub(crate) fn find_tree_at(&self, (x, y): (usize, usize)) -> usize {
        let p = y * self.size + x;
        *self.grid.get(p).unwrap()
    }

    pub(crate) fn at_index(&self, index: usize) -> usize {
        *self.grid.get(index).unwrap()
    }

    pub(crate) fn tree_position(&self, l: usize) -> (usize, usize) {
        let y = (l as f64 / self.size as f64).ceil() as usize - 1;
        let x = (l - 1) % self.size;
        (x, y)
    }

    pub fn is_on_edge(&self, (x, y): (usize, usize)) -> bool {
        x == 0 || y == 0 || x == self.size - 1 || y == self.size - 1
    }
}

fn main() {
    let start = start_it();
    let grid = Grid::new(read_from_file(REAL_DATA));
    let mut tree_visible = vec![false; grid.size * grid.size];
    let mut max_score = 0;
    for (ii, current_tree) in grid.grid.iter().enumerate() {
        let pos = grid.tree_position(ii + 1);
        if grid.is_on_edge(pos) {
            tree_visible[ii] = true;
            continue;
        }
        let to_check: Vec<(i32, i32)> = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut scenic_score = 1;
        for nn in to_check {
            let mut start_pos = pos;
            let mut count = 0;
            loop {
                let next_pos = (
                    (start_pos.0 as i32 + nn.0) as usize,
                    (start_pos.1 as i32 + nn.1) as usize,
                );
                let tree = grid.find_tree_at(next_pos);
                count += 1;
                if tree >= *current_tree {
                    break;
                }
                start_pos = next_pos;
                if grid.is_on_edge(start_pos) {
                    if !tree_visible[ii] {
                        tree_visible[ii] = true;
                    }
                    break;
                }
            }
            scenic_score *= count;
        }
        max_score = max(scenic_score, max_score);
    }
    let c = tree_visible.iter().filter(|&v| *v).count();
    println!("{c} {max_score}");
    end_it(start)
}

#[test]
fn test_with_data() {}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_from_file;

    #[test]
    fn test_conversion() {
        let grid = Grid::new(read_from_file(TEST_DATA));
        let p = grid.tree_position(11);
        assert_eq!((0, 2), p);
        let p = grid.tree_position(5);
        assert_eq!((4, 0), p);
        let p = grid.tree_position(25);
        assert_eq!((4, 4), p);
    }

    #[test]
    fn test_check_sides_visible() {
        let grid = Grid::new(read_from_file(TEST_DATA));
        let h = grid.size;
        assert_eq!(h, 5);
        assert_eq!(16, grid.count_sides());
    }

    #[test]
    fn test_building_grid() {
        let grid = Grid::new(read_from_file(TEST_DATA));
        assert_eq!(
            grid.grid,
            vec![3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0]
        )
    }

    #[test]
    fn test_at_position() {
        let grid = Grid::new(read_from_file(TEST_DATA));
        let c = grid.find_tree_at((4, 4));
        assert_eq!(c, 0);
        let c = grid.find_tree_at((4, 3));
        assert_eq!(c, 9);
    }

    #[test]
    fn test_coordinate() {
        let grid = Grid::new(read_from_file(TEST_DATA));
        let c = grid.tree_position(11);
        assert_eq!(c, (0, 2));
        assert_eq!(grid.find_tree_at(c), 6);
        //
        let c = grid.tree_position(25);
        assert_eq!(c, (4, 4));
        assert_eq!(grid.find_tree_at(c), 0);
    }

    #[test]
    fn test_check_on_edge() {
        let grid = Grid::new(read_from_file(TEST_DATA));
        assert!(grid.is_on_edge((4, 3)));
        assert!(!grid.is_on_edge((3, 3)));
    }
}
//3037325512653323354935390
