use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let grid = char_grid(input);
    let mut energized = grid.map(|_| false);
    let mut walked = grid.map(|_| [false; 4]);
    let mut states = vec![];

    let mut count_energized = |start_pos: Point, start_dir: Dir| -> usize {
        energized.fill(false);
        walked.fill([false; 4]);
        states.clear();
        let mut should_add = |(pos, dir): (Point, Dir)| {
            let Some(cell) = walked.get_mut(pos) else {
                return false;
            };
            let has_walked = &mut cell[dir as usize];
            if *has_walked {
                false
            } else {
                *has_walked = true;
                true
            }
        };

        states.push((start_pos, start_dir));
        while let Some((mut pos, mut dir)) = states.pop() {
            let Some(c) = grid.get(pos) else { continue };
            energized[pos] = true;

            match c {
                '.' => {}
                '|' if dir.is_vertical() => {}
                '|' => {
                    let split = (pos + Dir::Up, Dir::Up);
                    if should_add(split) {
                        states.push(split);
                    }
                    dir = Dir::Down;
                }
                '-' if dir.is_horizontal() => {}
                '-' => {
                    let split = (pos + Dir::Left, Dir::Left);
                    if should_add(split) {
                        states.push(split);
                    }
                    dir = Dir::Right;
                }
                '\\' => {
                    dir = match dir {
                        Dir::Up => Dir::Left,
                        Dir::Down => Dir::Right,
                        Dir::Left => Dir::Up,
                        Dir::Right => Dir::Down,
                    }
                }
                '/' => {
                    dir = match dir {
                        Dir::Up => Dir::Right,
                        Dir::Down => Dir::Left,
                        Dir::Left => Dir::Down,
                        Dir::Right => Dir::Up,
                    }
                }
                _ => panic!("Unexpected char: {}", c),
            }
            let next_state = (pos + dir, dir);
            if should_add(next_state) {
                states.push(next_state);
            }
        }

        energized.count()
    };

    let mut best_count = 0;
    for x in 0..grid.w() {
        let count = count_energized(p2(x, 0), Dir::Down);
        if count > best_count {
            best_count = count;
        }
        let count = count_energized(p2(x, grid.h() - 1), Dir::Up);
        if count > best_count {
            best_count = count;
        }
    }
    for y in 0..grid.h() {
        let count = count_energized(p2(0, y), Dir::Right);
        if count > best_count {
            best_count = count;
        }
        let count = count_energized(p2(grid.w() - 1, y), Dir::Left);
        if count > best_count {
            best_count = count;
        }
    }

    pv!(best_count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");

    let mut grid = char_grid(input);
    let mut energized = grid.map(|_| false);

    let mut walked = grid.map(|_| [false; 4]);
    let mut should_add = |(pos, dir): (Point, Dir)| {
        let Some(cell) = walked.get_mut(pos) else {
            return false;
        };
        let has_walked = &mut cell[dir as usize];
        if *has_walked {
            false
        } else {
            *has_walked = true;
            true
        }
    };

    let mut states = vec![(p2(0, 0), Dir::Right)];
    while let Some((mut pos, mut dir)) = states.pop() {
        let Some(c) = grid.get(pos) else { continue };
        energized[pos] = true;

        match c {
            '.' => {}
            '|' if dir.is_vertical() => {}
            '|' => {
                let split = (pos + Dir::Up, Dir::Up);
                if should_add(split) {
                    states.push(split);
                }
                dir = Dir::Down;
            }
            '-' if dir.is_horizontal() => {}
            '-' => {
                let split = (pos + Dir::Left, Dir::Left);
                if should_add(split) {
                    states.push(split);
                }
                dir = Dir::Right;
            }
            '\\' => {
                dir = match dir {
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                }
            }
            '/' => {
                dir = match dir {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                }
            }
            _ => panic!("Unexpected char: {}", c),
        }
        let next_state = (pos + dir, dir);
        if should_add(next_state) {
            states.push(next_state);
        }
    }

    let result = energized.count();
    pv!(result);
}
