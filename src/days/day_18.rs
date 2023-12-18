use rayon::iter::plumbing;

use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "{Dir} {usize} (#{isize:x}{char})").unwrap())
        .map(|(_, _, len, dir)| (Dir::from(dir.to_digit(10).unwrap()).clockwise(), len)) // clockwise because 0 => R instead of U
        .to_vec();

    let mut min_pos = p2(0, 0);
    let mut max_pos = p2(0, 0);
    let mut pos = p2(0, 0isize);
    for (dir, len) in &parsed {
        pos += dir.as_delta() * *len;
        min_pos = min_pos.cwise_min(pos);
        max_pos = max_pos.cwise_max(pos);
    }

    let mut inside_count = 0;
    let mut row_ranges = vec![];
    let mut row = min_pos.y;
    while row <= max_pos.y {
        row_ranges.clear();
        let mut next_change = max_pos.y + 1;
        let mut pos = p2(0, 0);
        for i in 0..parsed.len() {
            let (dir, len) = &parsed[i];
            let start = pos;
            pos += dir.as_delta() * *len;
            let end = pos;

            let top_left = start.cwise_min(end);
            let bottom_right = start.cwise_max(end);

            let flips; // indicates if inside/outside flips
            if dir.is_horizontal() {
                if top_left.y != row {
                    if top_left.y > row {
                        next_change = next_change.min(top_left.y);
                    }
                    continue;
                }
                // flip cases:
                //            ^        v
                // +----------+   or   +----------+
                // ^                              v
                //
                // non-flip cases:
                // v          ^
                // +----------+   or   +----------+
                //                     ^          v
                let prev_i = (i + parsed.len() - 1) % parsed.len();
                let next_i = (i + 1) % parsed.len();
                let prev_dir = parsed[prev_i].0;
                let next_dir = parsed[next_i].0;
                assert!(prev_dir.is_vertical());
                assert!(next_dir.is_vertical());
                if (prev_dir == Dir::Up) == (next_dir == Dir::Up) {
                    flips = true;
                } else {
                    flips = false;
                }
                next_change = row + 1; // always continue with next row
            } else {
                if top_left.y >= row || row >= bottom_right.y {
                    // include 'or equal' because the bar itself is added as horizontal

                    if top_left.y > row {
                        next_change = next_change.min(top_left.y);
                    }
                    continue;
                }
                flips = true; // vertical bars always flip
                next_change = next_change.min(bottom_right.y);
            }
            row_ranges.push((top_left.x..=bottom_right.x, flips));
        }
        row_ranges.sort_by_key(|(r, _)| *r.start());

        let mut x = min_pos.x - 1;
        let mut inside = false;
        let mut inside_added = 0;
        for (range, flips) in &row_ranges {
            if inside {
                inside_added += range.start() - x;
            }
            inside_added += range.end() - range.start() + 1;

            x = range.end() + 1;
            if *flips {
                inside = !inside;
            }
        }
        assert!(!inside);
        inside_count += inside_added * (next_change - row);
        if next_change > row {
            row = next_change;
        } else {
            row += 1;
        }
    }
    pv!(inside_count);
    // 70764156 low
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "{Dir} {usize} (#{usize:x})").unwrap());

    let mut min_pos = p2(0, 0);
    let mut max_pos = p2(0, 0);
    let mut pos = p2(0, 0isize);
    for (dir, len, _) in parsed.clone() {
        pos += dir.as_delta() * len as isize;
        min_pos = min_pos.cwise_min(pos);
        max_pos = max_pos.cwise_max(pos);
    }

    let w = (max_pos.x - min_pos.x + 1) as usize + 2;
    let h = (max_pos.y - min_pos.y + 1) as usize + 2;
    let start_x = -min_pos.x as usize + 1;
    let start_y = -min_pos.y as usize + 1;

    let mut grid = Grid::new_clone(p2(w, h), false);
    let mut pos = p2(start_x, start_y);
    for (dir, len, color) in parsed {
        for _ in 0..len {
            pos += dir;
            grid[pos] = true;
        }
    }

    let mut queue = vec![p2(0, 0)];
    let mut outside_count = 1;
    grid[p2(0, 0)] = true;
    while let Some(pos) = queue.pop() {
        for dir in Dir::all() {
            let next = pos + dir;
            let Some(cell) = grid.get_mut(next) else {
                continue;
            };
            if *cell {
                continue;
            }
            *cell = true;
            outside_count += 1;
            queue.push(next);
        }
    }

    let inside_count = w * h - outside_count;
    pv!(inside_count);
}
