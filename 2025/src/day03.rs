use std::cmp::max;
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let mut silver = 0;
    let mut gold = 0;
    let input = fs::read_to_string("3");
    // let input = fs::read_to_string("3.1");
    let input = input.unwrap();
    let input = input.strip_suffix("\n").unwrap();
    input.lines().for_each(|line| {
        let cur = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<u64>>();
        let mut biggest = 0;
        let mut so_far = cur[cur.len() - 1];
        for i in cur.iter().rev().skip(1) {
            let dig = i.to_owned();
            biggest = max(biggest, dig * 10 + so_far);
            so_far = max(so_far, dig);
        }
        fn twelve_largest_dp(cur: &Vec<u64>) -> u64 {
            let n = cur.len();
            let mut dp: Vec<Vec<u64>> = vec![vec![0; 13]; n + 1];
            for i in 1..=n {
                let dig = cur[i - 1];
                for j in 1..=12 {
                    dp[i][j] = dp[i - 1][j];
                    if dp[i - 1][j - 1] * 10 + dig > dp[i][j] {
                        dp[i][j] = dp[i - 1][j - 1] * 10 + dig;
                    }
                }
            }
            dp[n][12]
        }
        silver += biggest;
        gold += twelve_largest_dp(&cur) as u64;
    });
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
