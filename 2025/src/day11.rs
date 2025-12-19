use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub fn main() {
    let mut silver = 0;
    let mut gold = 0;
    let input = fs::read_to_string("11").unwrap();
    // let input = fs::read_to_string("11.1").unwrap();
    // let input = fs::read_to_string("11.2").unwrap();

    let mut graph = HashMap::new();
    // count number of paths from you to out

    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(":").collect();
        let from = parts[0].trim();
        let tos: Vec<&str> = parts[1].trim().split(" ").map(|s| s.trim()).collect();
        graph.insert(
            from.to_string(),
            tos.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
        );
    });
    fn dfs(node: &String, graph: &HashMap<String, Vec<String>>) -> usize {
        if node == "out" {
            return 1;
        }
        let mut total = 0;
        for neighbor in graph.get(node).unwrap() {
            total += dfs(neighbor, graph);
        }
        total
    }
    // check visited fft and dac
    let mut cache = HashMap::new();
    fn dfs2(
        node: &String,
        graph: &HashMap<String, Vec<String>>,
        mut state: u64,
        cache: &mut HashMap<(String, u64), usize>,
    ) -> usize {
        // println!("at node {} with state {}", node, state);
        // check cache
        if cache.contains_key(&(node.clone(), state)) {
            return *cache.get(&(node.clone(), state)).unwrap();
        }
        if node == "out" {
            // println!("reached out with state {}", state);
            return if state == 1001 { 1 } else { 0 };
        }
        if node == "fft" {
            state += 1;
        }
        if node == "dac" {
            state += 1000;
        }

        let mut total = 0;
        // return 0 if node not in graph
        if !graph.contains_key(node) {
            return 0;
        }
        for neighbor in graph.get(node).unwrap() {
            total += dfs2(neighbor, graph, state, cache);
        }
        cache.insert((node.clone(), state), total);
        total
    }
    silver = dfs(&"you".to_string(), &graph) as i32;
    gold = dfs2(&"svr".to_string(), &graph, 0, &mut cache) as u64;
    println!("silver: {}", silver);
    println!("gold: {}", gold);
}
