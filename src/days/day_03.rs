use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mut grid = char_grid(input);

    let neighbors = grid.moore();

    let mut gears: HashMap<Point, Vec<usize>> = HashMap::new();

    for y in 0..grid.h() {
        let mut num: Option<(usize, HashSet<Point>)> = None;
        for x in 0..grid.w() {
            let p = p2(x, y);
            if let Some(added) = grid[p].to_digit(10) {
                let (n, g) = num.get_or_insert_with(|| (0, HashSet::new()));
                *n = *n * 10 + added as usize;
                g.extend(neighbors.get_all_neighbors(p).filter(|p| grid[*p] == '*'));

                if x != grid.w() - 1 {
                    continue;
                }
            }
            // non-digit or end of line
            if let Some((n, g)) = num.take() {
                for p in g {
                    gears.entry(p).or_default().push(n);
                }
            }
        }
    }

    let mut sum = 0;
    for (p, v) in gears {
        if v.len() == 2 {
            sum += v[0] * v[1];
        }
    }
    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mut grid = char_grid(input);

    let neighbors = grid.moore();

    let mut sum = 0;
    for y in 0..grid.h() {
        let mut num: Option<(usize, bool)> = None;
        for x in 0..grid.w() {
            let p = p2(x, y);
            if let Some(added) = grid[p].to_digit(10) {
                let added_valid = neighbors
                    .get_all_neighbors(p2(x, y))
                    .any(|p| !matches!(grid[p], '0'..='9' | '.'));

                let (n, valid) = num.get_or_insert((0, false));
                *n = *n * 10 + added as usize;
                *valid = *valid || added_valid;

                if x != grid.w() - 1 {
                    continue;
                }
            }
            // non-digit or end of line
            if let Some((n, true)) = num.take() {
                sum += n;
            }
        }
    }

    pv!(sum);
}
