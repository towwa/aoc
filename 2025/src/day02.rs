use std::fs;
pub fn main() {
    let mut silver = 0;
    let mut gold = 0;
    let input = fs::read_to_string("2");
    let input = input.unwrap();
    let input = input.strip_suffix("\n").unwrap();
    input.split(",").for_each(|pair| {
        let mut nums = pair.split("-").flat_map(|n| n.parse::<u64>());
        let start = nums.next().unwrap();
        let end = nums.next().unwrap();
        for i in start..=end {
            let cur = i.to_string();
            // println!("{}", cur);
            if cur.len() % 2 == 0 && cur[..cur.len() / 2] == cur[cur.len() / 2..] {
                silver += i;
            }
            if (format!("{}{}", i, i))[1..2 * cur.len() - 1].contains(&cur) {
                gold += i;
            }
        }
    });
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
