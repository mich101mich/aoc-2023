use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let mut grid = hashtag_grid(input);

    let mut expand_col = vec![false; grid.w()];
    let mut expand_row = vec![false; grid.h()];

    for x in 0..grid.w() {
        expand_col[x] = grid.col(x).all(|b| !b);
    }
    for y in 0..grid.h() {
        expand_row[y] = grid.row(y).all(|b| !b);
    }

    let mut galaxies = vec![];
    let mut y_offset = 0;
    for y in 0..grid.h() {
        if expand_row[y] {
            y_offset += 1000000 - 1;
            continue;
        }
        let mut x_offset = 0;
        for x in 0..grid.w() {
            if expand_col[x] {
                x_offset += 1000000 - 1;
                continue;
            }
            if !grid[p2(x, y)] {
                continue;
            }
            galaxies.push(p2(x + x_offset, y + y_offset));
        }
    }

    let neighborhood = ManhattanNeighborhood::new(0, 0);

    let mut sum = 0;
    for (i, pos) in galaxies.iter().enumerate() {
        for other in &galaxies[i + 1..] {
            sum += neighborhood.heuristic(*pos, *other);
        }
    }
    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let mut grid = hashtag_grid(input);

    let mut expand_col = vec![false; grid.w()];
    let mut expand_row = vec![false; grid.h()];

    for x in 0..grid.w() {
        expand_col[x] = grid.col(x).all(|b| !b);
    }
    for y in 0..grid.h() {
        expand_row[y] = grid.row(y).all(|b| !b);
    }

    let mut galaxies = vec![];
    let mut y_offset = 0;
    for y in 0..grid.h() {
        if expand_row[y] {
            y_offset += 1;
            continue;
        }
        let mut x_offset = 0;
        for x in 0..grid.w() {
            if expand_col[x] {
                x_offset += 1;
                continue;
            }
            if !grid[p2(x, y)] {
                continue;
            }
            galaxies.push(p2(x + x_offset, y + y_offset));
        }
    }

    let neighborhood = ManhattanNeighborhood::new(0, 0);

    let mut sum = 0;
    for (i, pos) in galaxies.iter().enumerate() {
        for other in &galaxies[i + 1..] {
            sum += neighborhood.heuristic(*pos, *other);
        }
    }
    pv!(sum);
}
