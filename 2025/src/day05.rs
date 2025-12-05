use std::cmp::max;
use std::collections::HashMap;
use std::fs;

pub fn main() {
    let mut silver = 0;
    let mut gold: u64 = 0;
    let input = fs::read_to_string("5").unwrap();
    // let input = fs::read_to_string("5.1").unwrap();
    let mut ranges = vec![];
    let mut ingredients = vec![];
    for line in input.lines() {
        if line.is_empty() {
            return;
        }
        if let Some((start, end)) = line.split_once("-") {
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            ranges.push((start, end));
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }
    for i in ingredients {
        let covered = ranges.iter().filter(|(s, e)| i >= *s && i <= *e).count();
        if covered >= 1 {
            silver += 1;
        }
    }
    ranges.sort();
    let mut merged: Vec<(u64, u64)> = vec![];
    for (start, end) in ranges {
        if merged.is_empty() || merged.last().unwrap().1 < start {
            merged.push((start, end));
        } else {
            let last = merged.last_mut().unwrap();
            last.1 = max(last.1, end);
        }
    }
    gold = merged.iter().map(|(s, e)| e - s + 1).sum::<u64>();
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
