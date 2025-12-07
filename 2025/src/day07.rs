use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub fn main() {
    let mut silver = 0;
    let mut gold: u64 = 0;
    let mut beam = (0, 0);
    let mut splitters = HashSet::new();
    let input = fs::read_to_string("7").unwrap();
    // let input = fs::read_to_string("7.1").unwrap();
    let (n, m) = (input.lines().count(), input.lines().next().unwrap().len());
    input.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, ch)| {
            if ch == 'S' {
                beam.0 = i;
                beam.1 = j;
            } else if ch == '^' {
                splitters.insert((i, j));
            }
        });
    });
    // simulate beam falling and splitting
    // each time it hits a splitter, it splits into two beams
    let mut queue = VecDeque::new();
    queue.push_back(beam);
    let mut visited = HashSet::new();
    visited.insert(beam);
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        if x + 1 < n {
            if splitters.contains(&(x, y)) {
                // split beam
                silver += 1;
                if !visited.contains(&(x + 1, y - 1)) {
                    visited.insert((x + 1, y - 1));
                    queue.push_back((x + 1, y - 1));
                }
                if !visited.contains(&(x + 1, y + 1)) {
                    visited.insert((x + 1, y + 1));
                    queue.push_back((x + 1, y + 1));
                }
            } else if !visited.contains(&(x + 1, y)) {
                visited.insert((x + 1, y));
                queue.push_back((x + 1, y));
            }
        }
    }
    // part 2 dp solution, many worlds solution, count num ways to reach bottom if each time can
    // split left or split right
    // dp[i][j] = dp[i-1][j] + dp[i-1][j-1] + dp[i-1][j+1]

    let mut dp: Vec<Vec<u64>> = vec![vec![0; m]; n];
    dp[beam.0][beam.1] = 1;
    for i in beam.0..n - 1 {
        for j in 0..m {
            if dp[i][j] == 0 {
                continue;
            }
            if splitters.contains(&(i, j)) {
                if j > 0 {
                    dp[i + 1][j - 1] += dp[i][j];
                }
                if j + 1 < m {
                    dp[i + 1][j + 1] += dp[i][j];
                }
            } else {
                dp[i + 1][j] += dp[i][j];
            }
        }
    }
    gold = dp[n - 1].iter().sum();
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
