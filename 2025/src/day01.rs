use std::fs;
pub fn main() {
    let lines = fs::read_to_string("1").unwrap();
    const MOD: i64 = 100;
    let mut start: i64 = 50;
    let mut silver = 0;
    let mut gold = 0;
    lines.lines().for_each(|line| {
        let (cmd, amt) = (
            line.chars().next().unwrap(),
            line[1..].parse::<i64>().unwrap(),
        );
        let laps = amt / MOD;
        let amt = amt % MOD;
        gold += laps;

        if start == 0 && amt % MOD == 0 {
            gold -= 1; // if we land on zero exactly, count one less lap
        }
        match cmd {
            'L' => {
                if start != 0 && start - amt < 0 {
                    gold += 1;
                }
                start = (start + MOD - amt % MOD) % MOD;
            }
            'R' => {
                if start != 0 && start + amt > MOD {
                    gold += 1;
                }
                start = (start + amt) % MOD;
            }
            _ => panic!("Unknown command {}", cmd),
        }
        if start == 0 {
            silver += 1;
            gold += 1;
        }
        println!("gold: {}", gold);
    });
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
