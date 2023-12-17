use crate::utils::*;

fn fill_heuristic(grid: &Grid<usize>) -> Grid<usize> {
    use std::cmp::Reverse;
    let goal = p2(grid.w() - 1, grid.h() - 1);
    let mut heuristic = grid.map(|_| usize::MAX);
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), goal.x, goal.y));
    while let Some((Reverse(cost), x, y)) = queue.pop() {
        let p = p2(x, y);
        let Some(added_cost) = grid.get(p) else {
            continue;
        };
        let cell = &mut heuristic[p];
        if *cell != usize::MAX {
            continue;
        }
        *cell = cost;
        for dir in Dir::all() {
            let next_pos = p + dir;
            queue.push((Reverse(cost + added_cost), next_pos.x, next_pos.y));
        }
    }

    assert!(!heuristic.grid_iter().any(|&x| x == usize::MAX));
    heuristic
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let grid = digit_grid(input);
    let goal = p2(grid.w() - 1, grid.h() - 1);

    let heuristic = fill_heuristic(&grid);

    const DUMMY_START: (Point, Dir) = (p2(9999, 1), Dir::Right);
    const DUMMY_GOAL: (Point, Dir) = (p2(9999, 2), Dir::Right);

    let path = a_star_search(
        |(mut pos, dir), out| {
            if pos == goal {
                out.push((DUMMY_GOAL, 0));
                return;
            } else if (pos, dir) == DUMMY_START {
                out.push(((p2(0, 0), Dir::Right), 0));
                out.push(((p2(0, 0), Dir::Down), 0));
                return;
            }

            let mut cost = 0;
            for _ in 1..4 {
                pos += dir;
                let Some(added_cost) = grid.get(pos) else {
                    return;
                };
                cost += added_cost;
            }
            for _ in 4..=10 {
                pos += dir;
                let Some(added_cost) = grid.get(pos) else {
                    return;
                };
                cost += added_cost;
                out.push(((pos, dir.clockwise()), cost));
                out.push(((pos, dir.counter_clockwise()), cost));
            }
        },
        DUMMY_START,
        DUMMY_GOAL,
        |state| {
            if state == DUMMY_START {
                heuristic[p2(0, 0)]
            } else if state == DUMMY_GOAL {
                0
            } else {
                heuristic[state.0]
            }
        },
    );

    pv!(path.unwrap().cost);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let grid = digit_grid(input);
    let goal = p2(grid.w() - 1, grid.h() - 1);

    let heuristic = fill_heuristic(&grid);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct State {
        pos: Point,
        cost: usize,
        heuristic: usize,
        last_dir: Dir,
        num_straights: usize,
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    {
        let mut start_state = State {
            pos: p2(0, 0),
            cost: 0,
            heuristic: heuristic[p2(0, 0)],
            last_dir: Dir::Down,
            num_straights: 0,
        };
        queue.push(start_state);
        visited.insert(start_state);
        start_state.last_dir = Dir::Right;
        queue.push(start_state);
        visited.insert(start_state);
    }

    while let Some(state) = queue.pop() {
        if state.pos == goal {
            pv!(state.cost);
            break;
        }

        for dir in Dir::all() {
            if dir == state.last_dir.opposite() {
                continue;
            } else if dir == state.last_dir && state.num_straights >= 2 {
                continue;
            }

            let next_pos = state.pos + dir;
            let Some(added_cost) = grid.get(next_pos) else {
                continue;
            };

            let next_state = State {
                pos: next_pos,
                cost: state.cost + added_cost,
                heuristic: heuristic[next_pos],
                last_dir: dir,
                num_straights: if dir == state.last_dir {
                    state.num_straights + 1
                } else {
                    0
                },
            };

            if !visited.insert(next_state) {
                continue;
            }

            queue.push(next_state);
        }
    }
}
