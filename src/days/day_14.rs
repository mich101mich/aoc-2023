use rayon::iter::plumbing;

use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut grid = char_grid(input);

    let final_grid = detect_loop(1000000000, || {
        for _ in 0..4 {
            for x in 0..grid.w() {
                for y in 0..grid.h() {
                    if grid[p2(x, y)] != 'O' {
                        continue;
                    }
                    let mut last = None;
                    for place in (0..y).rev() {
                        if grid[p2(x, place)] == '#' {
                            break;
                        }
                        if grid[p2(x, place)] == '.' {
                            last = Some(place);
                        }
                    }
                    if let Some(place) = last {
                        grid[p2(x, y)] = '.';
                        grid[p2(x, place)] = 'O';
                    }
                }
            }
            grid.rotate_clockwise();
        }
        grid.clone()
    });

    let load = final_grid
        .grid_iter_index()
        .filter(|(_, c)| **c == 'O')
        .map(|(p, _)| grid.h() - p.y)
        .sum::<usize>();
    pv!(load);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");

    let mut grid = char_grid(input);

    for x in 0..grid.w() {
        for y in 0..grid.h() {
            if grid[p2(x, y)] != 'O' {
                continue;
            }
            let mut last = None;
            for place in (0..y).rev() {
                if grid[p2(x, place)] == '#' {
                    break;
                }
                if grid[p2(x, place)] == '.' {
                    last = Some(place);
                }
            }
            if let Some(place) = last {
                grid[p2(x, y)] = '.';
                grid[p2(x, place)] = 'O';
            }
        }
    }

    let load = grid
        .grid_iter_index()
        .filter(|(_, c)| **c == 'O')
        .map(|(p, _)| grid.h() - p.y)
        .sum::<usize>();

    pv!(load);
}
