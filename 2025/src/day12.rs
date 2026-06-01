use std::cmp::Reverse;
use std::collections::HashSet;
use std::fs;

#[derive(Clone)]
struct Variant {
    width: usize,
    height: usize,
    cells: Vec<(usize, usize)>,
    area: usize,
}

#[derive(Clone)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

pub fn main() {
    let use_sample = std::env::var("AOC_USE_SAMPLE").is_ok();
    let filename = if use_sample { "12.1" } else { "12" };
    let input = fs::read_to_string(filename).expect("unable to read input file");

    let (raw_shapes, regions) = parse_input(&input);
    if raw_shapes.is_empty() || regions.is_empty() {
        println!("silver: 0");
        println!("gold: 0");
        return;
    }

    let shape_variants: Vec<Vec<Variant>> = raw_shapes
        .iter()
        .map(|shape| generate_variants(shape))
        .collect();
    let cell_counts: Vec<usize> = shape_variants
        .iter()
        .map(|variants| variants[0].area)
        .collect();
    let mut shape_order: Vec<usize> = (0..shape_variants.len()).collect();
    shape_order.sort_by_key(|&idx| Reverse(cell_counts[idx]));

    let mut silver = 0;
    for region in regions {
        assert_eq!(
            region.counts.len(),
            shape_variants.len(),
            "region counts do not match shape list"
        );
        let mut board = vec![false; region.width * region.height];
        let mut counts = region.counts.clone();
        if can_fill(
            &shape_variants,
            &cell_counts,
            &shape_order,
            region.width,
            region.height,
            &mut board,
            &mut counts,
            region.width * region.height,
        ) {
            silver += 1;
        }
    }

    println!("silver: {}", silver);
    println!("gold: {}", 0);
}

fn parse_input(input: &str) -> (Vec<Vec<String>>, Vec<Region>) {
    let mut shapes = Vec::new();
    let mut current_shape = Vec::new();
    let mut regions = Vec::new();

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.contains('x') {
            if !current_shape.is_empty() {
                shapes.push(current_shape.clone());
                current_shape.clear();
            }
            regions.push(parse_region(trimmed));
        } else if trimmed.ends_with(':') && trimmed.chars().all(|c| c == ':' || c.is_ascii_digit())
        {
            if !current_shape.is_empty() {
                shapes.push(current_shape.clone());
                current_shape.clear();
            }
        } else {
            current_shape.push(trimmed.to_string());
        }
    }

    if !current_shape.is_empty() {
        shapes.push(current_shape);
    }

    (shapes, regions)
}

fn parse_region(line: &str) -> Region {
    let mut parts = line.split(':');
    let size_part = parts.next().expect("missing size");
    let counts_part = parts.next().unwrap_or("").trim();
    let mut size_iter = size_part.split('x');
    let width = size_iter
        .next()
        .expect("missing width")
        .trim()
        .parse::<usize>()
        .expect("invalid width");
    let height = size_iter
        .next()
        .expect("missing height")
        .trim()
        .parse::<usize>()
        .expect("invalid height");
    let counts = counts_part
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|value| value.parse::<usize>().expect("invalid count"))
        .collect::<Vec<_>>();
    Region {
        width,
        height,
        counts,
    }
}

fn generate_variants(shape: &[String]) -> Vec<Variant> {
    let base_grid = shape
        .iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut variants = Vec::new();
    let mut seen = HashSet::new();
    for flip in 0..=1 {
        let mut grid = if flip == 0 {
            base_grid.clone()
        } else {
            flip_horizontal(&base_grid)
        };
        for _ in 0..4 {
            let variant = grid_to_variant(&grid);
            let key = variant_key(&variant);
            if seen.insert(key) {
                variants.push(variant);
            }
            grid = rotate(&grid);
        }
    }
    variants
}

fn rotate(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result = vec![vec!['.'; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            result[c][rows - 1 - r] = grid[r][c];
        }
    }
    result
}

fn flip_horizontal(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    grid.iter()
        .map(|row| row.iter().rev().copied().collect::<Vec<_>>())
        .collect()
}

fn grid_to_variant(grid: &[Vec<char>]) -> Variant {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut min_r = rows;
    let mut max_r = 0;
    let mut min_c = cols;
    let mut max_c = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '#' {
                min_r = min_r.min(r);
                max_r = max_r.max(r);
                min_c = min_c.min(c);
                max_c = max_c.max(c);
            }
        }
    }
    let height = max_r - min_r + 1;
    let width = max_c - min_c + 1;
    let mut cells = Vec::new();
    for r in min_r..=max_r {
        for c in min_c..=max_c {
            if grid[r][c] == '#' {
                cells.push((r - min_r, c - min_c));
            }
        }
    }
    let area = cells.len();
    Variant {
        width,
        height,
        cells,
        area,
    }
}

fn variant_key(variant: &Variant) -> String {
    let mut rows = vec![vec!['.'; variant.width]; variant.height];
    for &(r, c) in &variant.cells {
        rows[r][c] = '#';
    }
    rows.into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("/")
}

fn can_fill(
    shape_variants: &[Vec<Variant>],
    cell_counts: &[usize],
    shape_order: &[usize],
    width: usize,
    height: usize,
    board: &mut [bool],
    counts: &mut [usize],
    remaining_empty: usize,
) -> bool {
    if counts.iter().all(|&c| c == 0) {
        return true;
    }
    let required_cells: usize = counts
        .iter()
        .enumerate()
        .map(|(idx, &cnt)| cnt * cell_counts[idx])
        .sum();
    if required_cells > remaining_empty {
        return false;
    }

    let next_idx = match board.iter().position(|&cell| !cell) {
        Some(idx) => idx,
        None => return false,
    };
    let row = next_idx / width;
    let col = next_idx % width;

    for &shape_idx in shape_order {
        if counts[shape_idx] == 0 {
            continue;
        }
        for variant in &shape_variants[shape_idx] {
            for &(cell_r, cell_c) in &variant.cells {
                if cell_r > row || cell_c > col {
                    continue;
                }
                let top_row = row - cell_r;
                let top_col = col - cell_c;
                if top_row + variant.height > height || top_col + variant.width > width {
                    continue;
                }
                if variant.cells.iter().all(|&(dr, dc)| {
                    let r = top_row + dr;
                    let c = top_col + dc;
                    !board[r * width + c]
                }) {
                    for &(dr, dc) in &variant.cells {
                        let r = top_row + dr;
                        let c = top_col + dc;
                        board[r * width + c] = true;
                    }
                    counts[shape_idx] -= 1;
                    if can_fill(
                        shape_variants,
                        cell_counts,
                        shape_order,
                        width,
                        height,
                        board,
                        counts,
                        remaining_empty - variant.area,
                    ) {
                        return true;
                    }
                    counts[shape_idx] += 1;
                    for &(dr, dc) in &variant.cells {
                        let r = top_row + dr;
                        let c = top_col + dc;
                        board[r * width + c] = false;
                    }
                }
            }
        }
    }
    board[next_idx] = true;
    let can_skip = can_fill(
        shape_variants,
        cell_counts,
        shape_order,
        width,
        height,
        board,
        counts,
        remaining_empty - 1,
    );
    board[next_idx] = false;
    can_skip
}
