use regex::Regex;
use std::fs;

const EPS: f64 = 1e-7;

fn parse(
    line: &str,
    re_state: &Regex,
    re_buttons: &Regex,
    re_voltage: &Regex,
) -> (u64, Vec<u64>, Vec<Vec<usize>>, Vec<u64>) {
    let lights = &re_state.captures(line).unwrap()[1];
    let state = lights
        .chars()
        .fold(0, |acc, ch| (acc << 1) | u64::from(ch == '#'));

    let mut masks = Vec::new();
    let mut buttons = Vec::new();
    for cap in re_buttons.captures_iter(line) {
        let button = cap[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        masks.push(
            button
                .iter()
                .fold(0, |mask, &idx| mask | (1 << (lights.len() - 1 - idx))),
        );
        buttons.push(button);
    }

    let voltage = re_voltage.captures(line).unwrap()[1]
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    (state, masks, buttons, voltage)
}

fn silver_for(state: u64, masks: &[u64]) -> u64 {
    (0..(1u64 << masks.len()))
        .filter_map(|subset| {
            let end = masks.iter().enumerate().fold(state, |state, (i, &mask)| {
                if subset & (1 << i) == 0 {
                    state
                } else {
                    state ^ mask
                }
            });
            (end == 0).then_some(subset.count_ones() as u64)
        })
        .min()
        .unwrap()
}

fn gold_for(buttons: &[Vec<usize>], target: &[u64]) -> u64 {
    let n = buttons.len();
    let m = target.len();

    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by_key(|&button| {
        std::cmp::Reverse(buttons[button].iter().map(|&c| target[c]).min().unwrap())
    });
    let bounds = order
        .iter()
        .map(|&button| buttons[button].iter().map(|&c| target[c]).min().unwrap())
        .collect::<Vec<_>>();

    let mut a = vec![vec![0.0; n + 1]; m];
    for (col, &button) in order.iter().enumerate() {
        for &counter in &buttons[button] {
            a[counter][col] = 1.0;
        }
    }
    for (row, &value) in target.iter().enumerate() {
        a[row][n] = value as f64;
    }

    let mut row = 0;
    let mut pivots = Vec::new();
    for col in 0..n {
        let Some(pivot) = (row..m).find(|&r| a[r][col].abs() > EPS) else {
            continue;
        };
        a.swap(row, pivot);

        let divisor = a[row][col];
        for c in col..=n {
            a[row][c] /= divisor;
        }

        let pivot_row = a[row].clone();
        for r in 0..m {
            if r == row || a[r][col].abs() <= EPS {
                continue;
            }
            let factor = a[r][col];
            for c in col..=n {
                a[r][c] -= factor * pivot_row[c];
            }
        }

        pivots.push(col);
        row += 1;
        if row == m {
            break;
        }
    }

    if a.iter()
        .any(|row| row[..n].iter().all(|value| value.abs() <= EPS) && row[n].abs() > EPS)
    {
        return u64::MAX;
    }

    let free = (0..n)
        .filter(|col| !pivots.contains(col))
        .collect::<Vec<_>>();
    let mut free_values = vec![0; free.len()];
    let mut best = u64::MAX;

    loop {
        let mut presses = vec![0; n];
        for (i, &col) in free.iter().enumerate() {
            presses[col] = free_values[i];
        }

        let mut ok = true;
        for (r, &col) in pivots.iter().enumerate() {
            let value = free
                .iter()
                .enumerate()
                .fold(a[r][n], |value, (i, &free_col)| {
                    value - a[r][free_col] * free_values[i] as f64
                });
            let rounded = value.round();
            if rounded < 0.0 || (value - rounded).abs() > EPS || rounded as u64 > bounds[col] {
                ok = false;
                break;
            }
            presses[col] = rounded as u64;
        }

        let total = presses.iter().sum();
        if ok && total < best && voltage_matches(buttons, target, &order, &presses) {
            best = total;
        }

        let mut i = 0;
        while i < free.len() {
            free_values[i] += 1;
            if free_values[i] <= bounds[free[i]] {
                break;
            }
            free_values[i] = 0;
            i += 1;
        }
        if i == free.len() {
            break;
        }
    }

    best
}

fn voltage_matches(
    buttons: &[Vec<usize>],
    target: &[u64],
    order: &[usize],
    presses: &[u64],
) -> bool {
    let mut actual = vec![0; target.len()];
    for (col, &button) in order.iter().enumerate() {
        for &counter in &buttons[button] {
            actual[counter] += presses[col];
        }
    }
    actual == target
}

pub fn main() {
    let input = fs::read_to_string("10").unwrap();
    // let input = fs::read_to_string("10.1").unwrap();

    let re_state = Regex::new(r"\[([.#]+)\]").unwrap();
    let re_buttons = Regex::new(r"\(([^)]+)\)").unwrap();
    let re_voltage = Regex::new(r"\{([0-9,]+)\}").unwrap();

    let mut silver = 0;
    let mut gold = 0;
    for line in input.lines() {
        let (state, masks, buttons, voltage) = parse(line, &re_state, &re_buttons, &re_voltage);
        silver += silver_for(state, &masks);
        gold += gold_for(&buttons, &voltage);
    }

    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
