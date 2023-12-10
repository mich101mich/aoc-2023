use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut grid = char_grid(input);

    let start = grid.grid_iter_index().find(|&(_, c)| *c == 'S').unwrap().0;
    let mut pipe_grid = grid.map(|c| match c {
        'S' | '.' => None,
        //             Up, Right, Down, Left
        'L' => Some([true, true, false, false]),
        '7' => Some([false, false, true, true]),
        'F' => Some([false, true, true, false]),
        'J' => Some([true, false, false, true]),
        '|' => Some([true, false, true, false]),
        '-' => Some([false, true, false, true]),
        _ => panic!("Unknown char: {}", c),
    });

    let mut on_loop = HashSet::new();

    for start_dir in Dir::all() {
        let mut pos = start + start_dir;
        let mut in_dir = start_dir;
        if pipe_grid
            .get(pos)
            .copied()
            .flatten()
            .filter(|pipe| pipe[start_dir.opposite().num()])
            .is_none()
        {
            continue;
        }
        let mut history = vec![start, pos];
        while pos != start {
            let Some(dirs) = pipe_grid.get(pos).copied().flatten() else {
                break;
            };
            let to_prev = in_dir.opposite();
            if !dirs[to_prev.num()] {
                break;
            }
            let Some(out_dir) = Dir::all().find(|&d| d != to_prev && dirs[d.num()]) else {
                break;
            };
            pos += out_dir;
            in_dir = out_dir;
            history.push(pos);
        }
        if pos != start {
            continue;
        }
        on_loop.extend(history);
        // connections to start: start_dir and opposite of in_dir
        let start_dirs = pipe_grid[start].insert([false; 4]);
        start_dirs[start_dir.num()] = true;
        start_dirs[in_dir.opposite().num()] = true;
        break;
    }

    for (pos, v) in pipe_grid.grid_iter_mut_index() {
        if !on_loop.contains(&pos) {
            *v = None;
        }
    }

    let mut scaled_grid = Grid::new_clone(p2(2 * grid.w() - 1, 2 * grid.h() - 1), false);

    for pos in &on_loop {
        let dirs = pipe_grid[pos].unwrap();
        let mut scaled_pos = *pos * 2;
        scaled_grid[scaled_pos] = true;
        for dir in Dir::all() {
            if dirs[dir.num()] {
                scaled_grid[scaled_pos + dir] = true;
            }
        }
    }

    let mut outside = scaled_grid.clone();
    // pipes are automatically outside

    let mut next = vec![];
    for y in 0..scaled_grid.h() {
        if !scaled_grid[p2(0, y)] {
            next.push(p2(0, y));
        }
        if !scaled_grid[p2(scaled_grid.w() - 1, y)] {
            next.push(p2(scaled_grid.w() - 1, y));
        }
    }
    for x in 0..scaled_grid.w() {
        if !scaled_grid[p2(x, 0)] {
            next.push(p2(x, 0));
        }
        if !scaled_grid[p2(x, scaled_grid.h() - 1)] {
            next.push(p2(x, scaled_grid.h() - 1));
        }
    }

    for p in &next {
        outside[*p] = true;
    }

    while let Some(p) = next.pop() {
        for dir in Dir::all() {
            let Some(next_p) = dir.bounded_add(p, scaled_grid.bounds()) else {
                continue;
            };
            if !outside[next_p] {
                outside[next_p] = true;
                next.push(next_p);
            }
        }
    }

    let mut inside = 0;
    for x in 0..pipe_grid.w() {
        for y in 0..pipe_grid.h() {
            let pos = p2(x, y) * 2;
            if !outside[pos] {
                inside += 1;
            }
        }
    }
    pv!(inside);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut grid = char_grid(input);

    let start = grid.grid_iter_index().find(|&(_, c)| *c == 'S').unwrap().0;
    let pipe_grid = grid.map(|c| match c {
        'S' | '.' => None,
        //             Up, Right, Down, Left
        'L' => Some([true, true, false, false]),
        '7' => Some([false, false, true, true]),
        'F' => Some([false, true, true, false]),
        'J' => Some([true, false, false, true]),
        '|' => Some([true, false, true, false]),
        '-' => Some([false, true, false, true]),
        _ => panic!("Unknown char: {}", c),
    });

    for start_dir in Dir::all() {
        let mut pos = start + start_dir;
        let mut in_dir = start_dir;
        if pipe_grid
            .get(pos)
            .copied()
            .flatten()
            .filter(|pipe| pipe[start_dir.opposite().num()])
            .is_none()
        {
            continue;
        }
        let mut history = vec![start, pos];
        while pos != start {
            let Some(dirs) = pipe_grid.get(pos).copied().flatten() else {
                break;
            };
            let to_prev = in_dir.opposite();
            if !dirs[to_prev.num()] {
                break;
            }
            let Some(out_dir) = Dir::all().find(|&d| d != to_prev && dirs[d.num()]) else {
                break;
            };
            pos += out_dir;
            in_dir = out_dir;
            history.push(pos);
        }
        if pos != start {
            continue;
        }
        pv!(history.len() / 2);
        break;
    }
}
