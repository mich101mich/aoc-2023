use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let mut src_grid = hashtag_grid(input);

    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, l)| {
            l.chars()
                .position(|c| c == 'S')
                .map(|x| p2(x as isize, y as isize))
        })
        .unwrap();

    src_grid[start] = false;

    let w = src_grid.w() as isize;
    let h = src_grid.h() as isize;

    let mut all_grids = HashMap::new();

    let mut positions = vec![start];
    let mut next_positions = vec![];
    let mut odd_reachable = 0;
    let mut last_num = 0;
    let mut prev_velocity = 0;
    let mut acceleration_history = vec![];
    let mut loop_start = 0;
    let measure_end = 6000;
    for i in 1.. {
        if i % 1000 == 0 {
            pv!(i);
        }
        next_positions.clear();
        for &pos in &positions {
            for dir in Dir::all() {
                let next = pos + dir;
                let meta_pos = p2(next.x.div_euclid(w), next.y.div_euclid(h));
                let inner_pos = p2(next.x.rem_euclid(w), next.y.rem_euclid(h));
                let grid = all_grids
                    .entry(meta_pos)
                    .or_insert_with(|| src_grid.clone());

                if !grid[inner_pos] {
                    grid[inner_pos] = true;
                    next_positions.push(next);
                }
            }
        }

        std::mem::swap(&mut positions, &mut next_positions);

        if i % 2 != 1 {
            continue; // only track odd_reachable
        }
        odd_reachable += positions.len();

        if i < measure_end {
            continue; // wait for system to stabilize
        }
        let velocity = odd_reachable as isize - last_num;
        last_num = odd_reachable as isize;
        let acceleration = velocity - prev_velocity;
        prev_velocity = velocity;
        if i < measure_end + 10 {
            continue; // no valid last_num and prev_velocity yet
        }
        acceleration_history.push(acceleration);
        if acceleration_history
            .iter()
            .filter(|&&a| a == acceleration)
            .count()
            > 3
        {
            loop_start = i;
            break;
        }
    }

    let target_acceleration = *acceleration_history.last().unwrap();
    let loop_len = acceleration_history
        .iter()
        .rev()
        .skip(1)
        .position(|&a| a == target_acceleration)
        .unwrap()
        + 1;

    let current_loop = &acceleration_history[acceleration_history.len() - loop_len..];
    let prev_loop = &acceleration_history
        [acceleration_history.len() - loop_len * 2..acceleration_history.len() - loop_len];

    let differences = prev_loop
        .iter()
        .zip(current_loop.iter())
        .map(|(&a, &b)| b - a)
        .to_vec();
    let mut accels = current_loop.to_vec();
    let mut accel_index = 0;

    let mut velocity = prev_velocity;

    for i in loop_start + 1..=26501365 {
        if i % 2 != 1 {
            continue; // only track odd_reachable
        }
        accels[accel_index] += differences[accel_index];
        let acceleration = accels[accel_index];
        accel_index = (accel_index + 1) % loop_len;
        velocity += acceleration;
        odd_reachable += velocity as usize;
    }

    pv!(odd_reachable);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let mut grid = hashtag_grid(input);

    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, l)| l.chars().position(|c| c == 'S').map(|x| p2(x, y)))
        .unwrap();

    grid[start] = false;

    let neighborhood = grid.manhattan();

    let mut positions = [start].into_iter().to_set();
    let mut next_positions = HashSet::new();
    for _ in 0..64 {
        next_positions.clear();
        for &pos in &positions {
            for next in neighborhood.get_all_neighbors(pos) {
                if !grid[next] {
                    next_positions.insert(next);
                }
            }
        }
        std::mem::swap(&mut positions, &mut next_positions);
    }
    pv!(positions.len());
}
