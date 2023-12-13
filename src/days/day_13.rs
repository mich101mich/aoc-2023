use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

fn find_reflection(grid: &Grid<bool>, not: Option<Reflection>) -> Option<Reflection> {
    let (dead_x, dead_y);
    match not {
        Some(Reflection::Horizontal(y)) => {
            dead_y = y;
            dead_x = grid.w() + 99;
        }
        Some(Reflection::Vertical(x)) => {
            dead_x = x;
            dead_y = grid.h() + 99;
        }
        None => {
            dead_x = grid.w() + 99;
            dead_y = grid.h() + 99;
        }
    };
    for y in 1..grid.h() {
        if y == dead_y {
            continue;
        }
        let mut correct = true;
        for dy in 1..grid.h() {
            let Some(y1) = y.checked_sub(dy) else { break };
            let y2 = y + dy - 1;
            if y2 >= grid.h() {
                break;
            }
            if grid.row(y1).zip(grid.row(y2)).any(|(a, b)| a != b) {
                correct = false;
                break;
            }
        }
        if correct {
            return Some(Reflection::Horizontal(y));
        }
    }

    for x in 1..grid.w() {
        if x == dead_x {
            continue;
        }
        let mut correct = true;
        for dx in 1..grid.w() {
            let Some(x1) = x.checked_sub(dx) else { break };
            let x2 = x + dx - 1;
            if x2 >= grid.w() {
                break;
            }
            if grid.col(x1).zip(grid.col(x2)).any(|(a, b)| a != b) {
                correct = false;
                break;
            }
        }
        if correct {
            return Some(Reflection::Vertical(x));
        }
    }

    None
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut iter = input.lines().peekable();
    let mut result = 0;
    while iter.peek().is_some() {
        let lines = iter.by_ref().take_while(|l| !l.is_empty()).to_vec();
        let input = lines.join("\n");
        let mut grid = hashtag_grid(&input);
        let old_reflection = find_reflection(&grid, None).unwrap();
        'outer: for y in 0..grid.h() {
            for x in 0..grid.w() {
                let p = p2(x, y);
                grid[p] = !grid[p];
                if let Some(reflection) = find_reflection(&grid, Some(old_reflection)) {
                    match reflection {
                        Reflection::Horizontal(y) => {
                            result += 100 * y;
                        }
                        Reflection::Vertical(x) => {
                            result += x;
                        }
                    }
                    break 'outer;
                }
                grid[p] = !grid[p];
            }
        }
    }
    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut iter = input.lines().peekable();
    let mut result = 0;
    while iter.peek().is_some() {
        let lines = iter.by_ref().take_while(|l| !l.is_empty()).to_vec();
        let input = lines.join("\n");
        let grid = hashtag_grid(&input);

        let reflection = find_reflection(&grid, None).unwrap();
        match reflection {
            Reflection::Horizontal(y) => {
                result += 100 * y;
            }
            Reflection::Vertical(x) => {
                result += x;
            }
        }
    }
    pv!(result);
}
