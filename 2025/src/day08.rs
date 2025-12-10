use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub fn main() {
    let mut silver = 0;
    let mut gold: u64 = 0;
    let input = fs::read_to_string("8").unwrap();
    let input = fs::read_to_string("8.1").unwrap();
    let mut boxes = Vec::new();
    for line in input.lines() {
        let fields = line
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let (j1, j2, j3) = (fields[0], fields[1], fields[2]);
        boxes.push((j1, j2, j3));
    }
    // idea; we need to repeatedly connect the two shortest euclidean distance boxes 1000 times
    // after that whatever groups are left we multiply their sizes together
    // we can use a union find data structure to keep track of connected components
    struct UnionFind {
        parent: Vec<usize>,
        size: Vec<u64>,
    }
    impl UnionFind {
        fn new(n: usize) -> Self {
            UnionFind {
                parent: (0..n).collect(),
                size: vec![1; n],
            }
        }
        fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                self.parent[x] = self.find(self.parent[x]);
            }
            self.parent[x]
        }
        fn union(&mut self, x: usize, y: usize) {
            let root_x = self.find(x);
            let root_y = self.find(y);
            if root_x != root_y {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
        fn get_size(&mut self, x: usize) -> u64 {
            let root_x = self.find(x);
            self.size[root_x]
        }
    }
    let n = boxes.len();
    let mut uf = UnionFind::new(n);
    let mut edges = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1, z1) = boxes[i];
            let (x2, y2, z2) = boxes[j];
            let dist = (x1 as i64 - x2 as i64).pow(2)
                + (y1 as i64 - y2 as i64).pow(2)
                + (z1 as i64 - z2 as i64).pow(2);
            edges.push((dist, i, j));
        }
    }
    edges.sort_by_key(|k| k.0);
    let max_connections = 1000;
    for i in 0..min(max_connections, edges.len()) {
        let (_, u, v) = edges[i];
        uf.union(u, v);
    }
    let mut group_sizes = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *group_sizes.entry(root).or_insert(0) += 1;
        println!("Box {} is in group {}", i, root);
    }
    // silver is product of sizes of 3 largest groups
    let mut group_vals = group_sizes.values().cloned().collect::<Vec<u64>>();
    group_vals.sort();
    silver = group_vals.iter().rev().take(3).product();
    // continue connecting until all boxes are connected, gold is the product of x coordinate of
    // final two boxes needed to connect all boxes
    let mut last_two = Vec::new();
    for i in max_connections..edges.len() {
        let (_, u, v) = edges[i];
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            if uf.get_size(u) == n as u64 {
                last_two.push((u, v));
                break;
            }
        }
    }
    println!("silver: {}", silver);
    println!(
        "gold: {}",
        last_two
            .iter()
            .fold(1u64, |acc, &(u, v)| { acc * (boxes[u].0 * boxes[v].0) })
    );
}
