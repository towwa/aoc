use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Slab {
    start: i64,
    end: i64,
    intervals: Vec<(i64, i64)>,
}

fn find_slab(slabs: &[Slab], x: i64) -> Option<usize> {
    // Slabs are sorted by start. Find the last slab whose start is <= x,
    // then verify x is not in a gap after that slab's end.
    let idx = slabs.partition_point(|slab| slab.start <= x);
    if idx == 0 {
        return None;
    }
    let idx = idx - 1;
    if x <= slabs[idx].end { Some(idx) } else { None }
}

fn slab_contains(slab: &Slab, y1: i64, y2: i64) -> bool {
    // A rectangle column fits in this slab if one inside interval covers
    // the rectangle's full vertical span.
    slab.intervals.iter().any(|&(lo, hi)| lo <= y1 && y2 <= hi)
}

pub fn main() {
    let mut silver = 0;
    let mut gold: u64 = 0;
    let input = fs::read_to_string("9").unwrap();
    // let input = fs::read_to_string("9.1").unwrap();
    let lines = input.lines().collect::<Vec<_>>();
    let points = lines
        .iter()
        .map(|line| {
            let mut pair = line.split(',');
            let a = pair.next().unwrap().parse::<i64>().unwrap();
            let b = pair.next().unwrap().parse::<i64>().unwrap();
            (a, b)
        })
        .collect::<Vec<_>>();

    // Compress the horizontal scanline map into slabs of x columns that share
    // the same active y boundaries.
    let mut add_events: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut remove_events: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut event_xs = Vec::new();

    // Turn every horizontal polygon edge into scanline events.
    // While sweeping left to right, this edge contributes a y-boundary
    // between its left x and right x.
    for (p1, p2) in points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
    {
        if p1.1 != p2.1 {
            continue;
        }
        let x1 = min(p1.0, p2.0);
        let x2 = max(p1.0, p2.0);
        let y = p1.1;
        add_events.entry(x1).or_default().push(y);
        remove_events.entry(x2).or_default().push(y);
        event_xs.push(x1);
        event_xs.push(x2);
    }

    event_xs.sort_unstable();
    event_xs.dedup();

    let mut active_counts: HashMap<i64, usize> = HashMap::new();
    let mut slabs = Vec::new();

    // Build x-slabs between consecutive event x values. Within one slab,
    // the active horizontal boundaries do not change, so the inside y-ranges
    // are constant for every x-column in that slab.
    for window in event_xs.windows(2) {
        let x = window[0];
        let next_x = window[1];

        // Update active boundaries at the left edge of this slab.
        // Removals happen before additions so an edge ending here is not
        // active for the slab starting here, while an edge starting here is.
        if let Some(ys) = remove_events.get(&x) {
            for &y in ys {
                if let Some(count) = active_counts.get_mut(&y) {
                    *count -= 1;
                    if *count == 0 {
                        active_counts.remove(&y);
                    }
                }
            }
        }
        if let Some(ys) = add_events.get(&x) {
            for &y in ys {
                *active_counts.entry(y).or_insert(0) += 1;
            }
        }

        if active_counts.is_empty() || x == next_x {
            continue;
        }

        let mut boundaries = active_counts.keys().cloned().collect::<Vec<_>>();
        boundaries.sort_unstable();
        let mut intervals = Vec::new();

        // Crossing sorted boundaries toggles outside/inside. Pair them up:
        // [bottom, top, bottom, top] becomes inside intervals.
        for chunk in boundaries.chunks(2) {
            if let [y1, y2] = chunk {
                intervals.push((*y1, *y2));
            }
        }

        slabs.push(Slab {
            start: x,
            end: next_x - 1,
            intervals,
        });
    }

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let dx = (p1.0 - p2.0).abs() + 1;
            let dy = (p1.1 - p2.1).abs() + 1;
            let area = dx * dy;
            silver = silver.max(area);

            // Gold only cares about rectangles larger than the best valid
            // rectangle already found.
            if area <= gold as i64 {
                continue;
            }

            let x1 = min(p1.0, p2.0);
            let x2 = max(p1.0, p2.0);
            let y1 = min(p1.1, p2.1);
            let y2 = max(p1.1, p2.1);

            let Some(mut slab_idx) = find_slab(&slabs, x1) else {
                continue;
            };

            let mut current_x = x1;
            let mut valid = true;
            while current_x <= x2 {
                if slab_idx >= slabs.len() {
                    valid = false;
                    break;
                }
                let slab = &slabs[slab_idx];

                // The rectangle is valid only if each slab across its width
                // exists and contains the rectangle's full y-span.
                if slab.start > current_x || !slab_contains(slab, y1, y2) {
                    valid = false;
                    break;
                }
                current_x = slab.end + 1;
                if current_x <= x2 {
                    slab_idx += 1;
                    if slab_idx < slabs.len() && slabs[slab_idx].start > current_x {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                gold = gold.max(area as u64);
            }
        }
    }

    println!("Silver: {}", silver);
    println!("Gold: {}", gold);
}
