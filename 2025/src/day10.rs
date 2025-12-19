use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub fn main() {
    let mut silver = 0;
    let mut gold: u64 = 0;
    let input = fs::read_to_string("10").unwrap();
    // let input = fs::read_to_string("10.1").unwrap();
    input.lines().for_each(|line| {
        //example line pattern [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        // first part is on lights, then buttons with () which toggle those buttons and voltage
        // between {}
        // silver: find minimum presses to turn on all lights
        let mut state: u64 = 0;
        let mut lights = Vec::new();
        let mut voltage = Vec::new();
        let re_state = Regex::new(r"\[([.#]+)\]").unwrap();
        let caps = re_state.captures(line).unwrap();
        let s = &caps[1];
        s.chars().for_each(|ch| {
            if ch == '#' {
                state = (state << 1) | 1;
            } else {
                state = state << 1;
            }
        });
        let re_buttons = Regex::new(r"\(([^)]+)\)").unwrap();
        for cap in re_buttons.captures_iter(line) {
            let btns: Vec<usize> = cap[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let mask = btns
                .iter()
                .fold(0u64, |acc, &b| acc | (1 << (s.len() - 1 - b)));
            lights.push(mask);
        }
        let re_lights = Regex::new(r"\{([0-9,]+)\}").unwrap();
        let caps = re_lights.captures(line).unwrap();
        voltage = caps[1]
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        // BFS to find minimum presses
        // state is a vector of 0/1
        // each button toggles certain lights
        // we want to turn all lights to 1
        // each state can be represented as a bitmask
        let target = 0;
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((state, 0u64, 0u64)); // (state, presses, voltage)
        visited.insert(state);
        while !queue.is_empty() {
            let (cur_state, presses, volt) = queue.pop_front().unwrap();
            if cur_state == target {
                silver += presses;
                break;
            }
            for (i, &btn) in lights.iter().enumerate() {
                let next_state = cur_state ^ btn;
                if !visited.contains(&next_state) {
                    visited.insert(next_state);
                    queue.push_back((next_state, presses + 1, volt));
                }
            }
        }
        // gold: find minimum presses so that each button press indexes add up to voltage array
        // Each press adds one to each index in each button to those voltage indexes
        // We want to reach the voltage array exactly, one button press adds 1 to each index in that button
        // however we can use dfs to explore button press space instead of solution state and prune

        fn dfs(
            lights: &Vec<u64>,
            voltage: &Vec<u64>,
            idx: usize,
            cur_voltage: &mut Vec<u64>,
            presses: u64,
            min_presses: &mut u64,
        ) {
            println!(
                "dfs idx: {}, cur_voltage: {:?}, presses: {}, min_presses: {}",
                idx, cur_voltage, presses, min_presses
            );
            if idx == lights.len() {
                if cur_voltage == voltage {
                    *min_presses = min(*min_presses, presses);
                }
                return;
            }
            // try pressing this button 0 to max times
            let max_press = (0..voltage.len())
                .filter(|&j| (lights[idx] & (1 << (voltage.len() - 1 - j))) != 0)
                .map(|idx| voltage[idx] - cur_voltage[idx])
                .min()
                .unwrap();
            for p in 0..=max_press {
                // apply presses
                for j in 0..voltage.len() {
                    if (lights[idx] & (1 << (voltage.len() - 1 - j))) != 0 {
                        cur_voltage[j] += p;
                    }
                }
                dfs(
                    lights,
                    voltage,
                    idx + 1,
                    cur_voltage,
                    presses + p,
                    min_presses,
                );
                // revert presses
                for j in 0..voltage.len() {
                    if (lights[idx] & (1 << (voltage.len() - 1 - j))) != 0 {
                        cur_voltage[j] -= p;
                    }
                }
            }
        }
        let mut gold_sol = u64::MAX;
        dfs(
            &lights,
            &voltage,
            0,
            &mut vec![0u64; voltage.len()],
            0,
            &mut gold_sol,
        );
        println!("gold solution for line: {}", gold_sol);
        gold += gold_sol;
    });

    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
