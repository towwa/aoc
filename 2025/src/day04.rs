use std::cmp::max;
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let mut silver = 0;
    let mut gold = 0;
    let input = fs::read_to_string("4").unwrap();
    // let input = fs::read_to_string("4.1").unwrap();
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let dirs: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    fn roll(grid: &mut Vec<Vec<char>>, dirs: &Vec<(i32, i32)>) -> usize {
        let mut changed = 0;
        let mut to_change = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != '@' {
                    continue;
                }
                let mut rolls = 0;
                for (dx, dy) in dirs {
                    let x = i as i32 + dx;
                    let y = j as i32 + dy;
                    if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
                        continue;
                    }
                    if grid[x as usize][y as usize] == '@' {
                        rolls += 1;
                    }
                }
                if rolls < 4 {
                    changed += 1;
                    to_change.push((i, j));
                }
            }
        }
        for (i, j) in to_change {
            grid[i][j] = '.';
        }
        changed
    }
    let mut times = 1;
    loop {
        let changed = roll(&mut grid, &dirs);
        if times == 1 {
            silver += changed;
            times -= 1;
        }
        gold += changed;
        if changed == 0 {
            break;
        }
    }
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
