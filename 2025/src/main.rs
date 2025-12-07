mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

// mod day19;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: i32 = args
        .get(1)
        .or_else(|| panic!("Usage: <day>"))
        .and_then(|s| s.parse::<i32>().map_err(|e| panic!("{:?}", e)).ok())
        .unwrap_or_else(|| panic!("Use a number for the day"));

    match day {
        1 => day01::main(),
        2 => day02::main(),
        3 => day03::main(),
        4 => day04::main(),
        5 => day05::main(),
        6 => day06::main(),
        7 => day07::main(),
        // 10 => day10::main(),
        // 19 => day19::main(),
        _ => eprintln!("Day {} not implemented", day),
    }
}
