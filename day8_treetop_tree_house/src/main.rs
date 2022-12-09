#![feature(is_sorted)]

use std::{collections::HashMap, fs, vec};

struct Grid {
    inner: Vec<Vec<usize>>,
}

impl Grid {
    fn new(input: String) -> Self {
        Self {
            inner: input
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|ch| ch.to_string().parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>(),
        }
    }

    fn is_visible(&self, row_index: usize, col_index: usize, height: &usize) -> bool {
        let mut sight: HashMap<&str, Vec<usize>> = HashMap::new();

        let mut is_visible = false;

        let mut top: Vec<usize> = vec![];
        let mut bottom: Vec<usize> = vec![];
        let mut left: Vec<usize> = vec![];
        let mut right: Vec<usize> = vec![];
        for (row_num, row) in self.inner.iter().enumerate() {
            {
                if row_num < row_index {
                    top.push(*row.get(col_index).unwrap());
                }
            }
            {
                if row_num > row_index {
                    bottom.push(*row.get(col_index).unwrap())
                }
            }
            {
                if row_num == row_index {
                    for (col_num, col) in row.iter().enumerate() {
                        if col_num < col_index {
                            left.push(*col)
                        }
                        if col_num > col_index {
                            right.push(*col)
                        }
                    }
                    sight.insert("left", left);
                    sight.insert("right", right);
                    left = vec![];
                    right = vec![];
                }
            }
        }
        sight.insert("top", top);
        sight.insert("bottom", bottom);

        for (_, v) in sight {
            let mut temp_v = v.clone();
            temp_v.sort();
            if temp_v.last().unwrap() < height {
                is_visible |= true;
            } else {
                is_visible |= false;
            }
        }

        is_visible
    }

    fn get_scenic_score(&self, row_index: usize, col_index: usize, height: &usize) -> usize {
        let mut sight: HashMap<&str, Vec<usize>> = HashMap::new();

        let mut top: Vec<usize> = vec![];
        let mut bottom: Vec<usize> = vec![];
        let mut left: Vec<usize> = vec![];
        let mut right: Vec<usize> = vec![];
        for (row_num, row) in self.inner.iter().enumerate() {
            {
                if row_num < row_index {
                    top.push(*row.get(col_index).unwrap());
                }
            }
            {
                if row_num > row_index {
                    bottom.push(*row.get(col_index).unwrap())
                }
            }
            {
                if row_num == row_index {
                    for (col_num, col) in row.iter().enumerate() {
                        if col_num < col_index {
                            left.push(*col)
                        }
                        if col_num > col_index {
                            right.push(*col)
                        }
                    }
                    left.reverse();
                    sight.insert("left", left);
                    sight.insert("right", right);
                    left = vec![];
                    right = vec![];
                }
            }
        }
        top.reverse();
        sight.insert("top", top);
        sight.insert("bottom", bottom);

        let mut total_score = 1_usize;

        for (_, view) in sight {
            let mut temp_score = 0_usize;
            for t in view.iter() {
                if t < height {
                    temp_score += 1;
                    continue;
                }
                if t >= height {
                    temp_score += 1;
                    break;
                }
            }
            total_score *= temp_score
        }

        total_score
    }
}

fn main() {
    let input = get_input("src/input.txt");
    let grid = Grid::new(input);

    let mut visible_egde = grid.inner.first().unwrap().len()
        + grid.inner.last().unwrap().len()
        + ((grid.inner.len() - 2) * 2);

    let mut scores: Vec<usize> = vec![];

    for (row_num, row) in grid.inner.iter().enumerate() {
        if row_num != 0 && row_num != grid.inner.len() - 1 {
            for (col_num, col) in row.iter().enumerate() {
                if col_num != 0 && col_num != row.len() - 1 {
                    if grid.is_visible(row_num, col_num, col) {
                        visible_egde += 1
                    }
                    let score = grid.get_scenic_score(row_num, col_num, col);
                    scores.push(score);
                }
            }
        }
    }

    println!("visible edges: {}", visible_egde);

    scores.sort();
    println!("max score: {}", scores.last().unwrap())
}

fn get_input(path: &str) -> String {
    let input = fs::read_to_string(path);
    match input {
        Ok(input) => input,
        Err(err) => panic!("err reading input: {}", err),
    }
}
