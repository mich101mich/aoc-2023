use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let mut grid = char_grid(input);

    let (start, goal) = {
        let start_y = 0;
        let start_x = grid.row(start_y).position(|c| *c == '.').unwrap();
        let goal_y = grid.h() - 1;
        let goal_x = grid.row(goal_y).position(|c| *c == '.').unwrap();

        // close the gaps and move start and goal inside the walls
        grid[p2(start_x, start_y)] = '#';
        grid[p2(goal_x, goal_y)] = '#';
        (p2(start_x, start_y + 1), p2(goal_x, goal_y - 1))
    };

    // nodes are intersections, aka points with 3 or more open neighbors
    let mut nodes = grid
        .grid_iter_index()
        .filter(|(_, c)| **c == '.')
        .filter(|(p, _)| Dir::all().filter(|dir| grid[*p + *dir] != '#').count() > 2)
        .map(|(p, _)| p)
        .to_vec();
    let start_index = nodes.len();
    nodes.push(start);
    let goal_index = nodes.len();
    nodes.push(goal);

    let node_map = nodes.iter().enumerate().map(|(i, p)| (*p, i)).to_map();
    let mut outgoing = vec![HashMap::new(); nodes.len()];

    for (i, pos) in nodes.iter().copied().enumerate() {
        for start_dir in Dir::all() {
            let start = pos + start_dir;
            if grid[start] == '#' {
                continue;
            }
            let mut steps = 1;
            let mut p = start;
            let mut dir = start_dir;
            while let Some(next_dir) = [dir.counter_clockwise(), dir, dir.clockwise()]
                .into_iter()
                .find(|dir| grid[p + *dir] != '#')
            {
                dir = next_dir;
                p += dir;
                steps += 1;
                if let Some(node) = node_map.get(&p) {
                    let entry = outgoing[i].entry(*node).or_default();
                    *entry = steps.max(*entry);
                    break;
                }
            }
        }
    }

    assert!(nodes.len() < 64); // use 64 bit bitset

    let mut max_candidate = 0;
    let mut positions = HashMap::new();
    let start_token = (start_index, 1u64 << start_index);
    positions.insert(start_token, 0);
    let mut next_positions = HashMap::new();

    loop {
        for ((i, visited), cost) in positions.drain() {
            if i == goal_index {
                max_candidate = max_candidate.max(cost);
                continue;
            }
            for (&j, &delta) in &outgoing[i] {
                if visited & (1 << j) != 0 {
                    continue;
                }
                let new_cost = cost + delta;
                let new_token = (j, visited | (1 << j));
                let cost = next_positions.entry(new_token).or_default();
                *cost = new_cost.max(*cost);
            }
        }
        if next_positions.is_empty() {
            break;
        }
        std::mem::swap(&mut positions, &mut next_positions);
    }

    let mut steps = max_candidate;
    steps += 2; // start and goal
    pv!(steps);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let mut grid = char_grid(input);

    let (start, goal) = {
        let start_y = 0;
        let start_x = grid.row(start_y).position(|c| *c == '.').unwrap();
        let goal_y = grid.h() - 1;
        let goal_x = grid.row(goal_y).position(|c| *c == '.').unwrap();

        // close the gaps and move start and goal inside the walls
        grid[p2(start_x, start_y)] = '#';
        grid[p2(goal_x, goal_y)] = '#';
        (p2(start_x, start_y + 1), p2(goal_x, goal_y - 1))
    };

    let can_move = |p: Point, dir: Dir| {
        let next = p + dir;
        grid[next] == '.' || Dir::from_char(grid[next]) == Some(dir)
    };

    // nodes are intersections, aka points with 3 or more open neighbors
    let mut nodes = grid
        .grid_iter_index()
        .filter(|(_, c)| **c == '.')
        .filter(|(p, _)| Dir::all().filter(|dir| grid[*p + *dir] != '#').count() > 2)
        .map(|(p, _)| p)
        .to_vec();
    nodes.push(start);
    nodes.push(goal);

    let node_map = nodes.iter().enumerate().map(|(i, p)| (*p, i)).to_map();
    let mut outgoing = vec![HashMap::new(); nodes.len()];

    for (i, pos) in nodes.iter().copied().enumerate() {
        for start_dir in Dir::all() {
            if !can_move(pos, start_dir) {
                continue;
            }
            let start = pos + start_dir;
            let mut steps = 1;
            let mut p = start;
            let mut dir = start_dir;
            while let Some(next_dir) = [dir.counter_clockwise(), dir, dir.clockwise()]
                .into_iter()
                .find(|dir| can_move(p, *dir))
            {
                dir = next_dir;
                p += dir;
                steps += 1;
                if let Some(node) = node_map.get(&p) {
                    let entry = outgoing[i].entry(*node).or_default();
                    *entry = steps.max(*entry);
                    break;
                }
            }
        }
    }

    let mut longest_to = vec![0; nodes.len()];
    let mut queue = vec![node_map[&start]];
    while let Some(i) = queue.pop() {
        for (j, cost) in &outgoing[i] {
            let new_cost = longest_to[i] + cost;
            if new_cost > longest_to[*j] {
                longest_to[*j] = new_cost;
                queue.push(*j);
            }
        }
    }

    let mut steps = longest_to[node_map[&goal]];
    steps += 2; // start and goal
    pv!(steps);
}
