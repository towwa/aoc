use std::cmp::max;
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let mut silver = 0;
    let mut gold: u64 = 0;
    let mut input = fs::read_to_string("6").unwrap();
    // let mut input = fs::read_to_string("6.1").unwrap();
    let mut numbers = vec![];
    let mut numbers_raw = vec![];
    let mut max_col_len = vec![];
    let mut operations = vec![];
    input.lines().for_each(|line| {
        if "*+".contains(line.chars().next().unwrap()) {
            operations = line
                .split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
        } else {
            // leave one space gap between
            let cur = line
                .split_whitespace()
                .map(|c| c.parse::<String>().unwrap())
                .collect::<Vec<String>>();
            if max_col_len.is_empty() {
                max_col_len = cur.iter().map(|x| x.len()).collect::<Vec<usize>>();
            } else {
                for (idx, val) in cur.iter().enumerate() {
                    max_col_len[idx] = max(max_col_len[idx], val.len());
                }
            }
            numbers.push(cur);
            numbers_raw.push(line.to_string());
        }
    });
    let mut col = 0;
    operations.iter().enumerate().for_each(|(idx, op)| {
        let end_cur = col + max_col_len[idx];
        let mut cur_nums = Vec::new();
        // generate numbers by using columns from numbers_raw
        for j in col..end_cur {
            let mut cur_num = 0;
            for row in &numbers_raw {
                let ch = row.chars().nth(j).unwrap();
                if ch != ' ' {
                    cur_num = cur_num * 10 + ch.to_digit(10).unwrap() as u64;
                }
            }
            cur_nums.push(cur_num);
        }
        col = end_cur + 1; // plus one for space
        if op.eq("+") {
            // add the column of grid[idx]
            // println!("adding column {}", idx);
            let col_sum: u64 = numbers
                .iter()
                .map(|row| row[idx].parse::<u64>().unwrap())
                .sum();
            silver += col_sum;
            gold += cur_nums.iter().sum::<u64>();
        } else if op.eq("*") {
            // multiply the column of grid[idx]
            let col_prod: u64 = numbers
                .iter()
                .map(|row| row[idx].parse::<u64>().unwrap())
                .product();
            silver += col_prod;
            gold += cur_nums.iter().product::<u64>();
        }
    });
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
