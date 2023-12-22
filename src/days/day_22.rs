use std::ops::RangeInclusive;

use crate::utils::*;

struct Brick {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}
impl Brick {
    pub fn new(
        x: RangeInclusive<usize>,
        y: RangeInclusive<usize>,
        z: RangeInclusive<usize>,
    ) -> Self {
        Self { x, y, z }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (Point, usize)> + 'a {
        self.x.clone().flat_map(move |x| {
            self.y
                .clone()
                .flat_map(move |y| self.z.clone().map(move |z| (p2(x, y), z)))
        })
    }

    pub fn layer_iter<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.x
            .clone()
            .flat_map(move |x| self.y.clone().map(move |y| p2(x, y)))
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let (mut w, mut h, mut d) = (0, 0, 0);
    let mut bricks = input
        .lines()
        .map(|l| sscanf!(l, "{0},{0},{0}~{0},{0},{0}", usize).unwrap())
        .map(|(ax, ay, az, bx, by, bz)| {
            w = w.max(ax).max(bx);
            h = h.max(ay).max(by);
            d = d.max(az).max(bz);
            Brick::new(ax..=bx, ay..=by, az..=bz)
        })
        .to_vec();

    let n = bricks.len();

    w += 1;
    h += 1;
    d += 1;

    let mut grid = vec![Grid::new_clone(p2(w, h), n); d];

    for (i, brick) in bricks.iter().enumerate() {
        for (p, z) in brick.iter() {
            grid[z][p] = i;
        }
    }

    let mut dependents = vec![HashSet::<usize>::new(); n];
    let mut dependencies = vec![HashSet::<usize>::new(); n];

    let mut queue = (0..n).to_queue();
    while let Some(i) = queue.pop_front() {
        let brick = &mut bricks[i];
        let bottom = *brick.z.start();
        if bottom == 1 {
            continue;
        }
        let deps = &mut dependencies[i];
        let mut next_z = 1;
        for z in (1..bottom).rev() {
            let layer = &grid[z];
            for p in brick.layer_iter() {
                if layer[p] != n {
                    deps.insert(layer[p]);
                }
            }
            if !deps.is_empty() {
                next_z = z + 1;
                break;
            }
        }

        for &dep in deps.iter() {
            dependents[dep].insert(i);
        }

        if next_z != bottom {
            for (p, z) in brick.iter() {
                grid[z][p] = n;
            }
            let top = *brick.z.end() - (bottom - next_z);
            brick.z = next_z..=top;
            for (p, z) in brick.iter() {
                grid[z][p] = i;
            }

            for dependent in dependents[i].drain() {
                dependencies[dependent].remove(&i);
                if dependencies[dependent].is_empty() {
                    queue.push_back(dependent);
                }
            }
        }
    }

    let mut count = 0;
    let mut falling = vec![false; n];
    for i in 0..n {
        falling.fill(false);
        queue.clear();
        falling[i] = true;
        queue.push_back(i);
        while let Some(x) = queue.pop_front() {
            for &y in dependents[x].iter() {
                if falling[y] {
                    continue;
                }
                falling[y] = dependencies[y].iter().all(|&y| falling[y]);
                if falling[y] {
                    queue.push_back(y);
                    count += 1;
                }
            }
        }
    }
    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    let (mut w, mut h, mut d) = (0, 0, 0);
    let mut bricks = input
        .lines()
        .map(|l| sscanf!(l, "{0},{0},{0}~{0},{0},{0}", usize).unwrap())
        .map(|(ax, ay, az, bx, by, bz)| {
            w = w.max(ax).max(bx);
            h = h.max(ay).max(by);
            d = d.max(az).max(bz);
            Brick::new(ax..=bx, ay..=by, az..=bz)
        })
        .to_vec();

    let n = bricks.len();

    w += 1;
    h += 1;
    d += 1;

    let mut grid = vec![Grid::new_clone(p2(w, h), n); d];

    for (i, brick) in bricks.iter().enumerate() {
        for (p, z) in brick.iter() {
            grid[z][p] = i;
        }
    }

    let mut dependents = vec![HashSet::<usize>::new(); n];
    let mut num_dependencies = vec![0; n];

    let mut queue = (0..n).to_queue();
    while let Some(i) = queue.pop_front() {
        let brick = &mut bricks[i];
        let bottom = *brick.z.start();
        if bottom == 1 {
            continue;
        }
        let mut dependencies = HashSet::new();
        let mut next_z = 1;
        for z in (1..bottom).rev() {
            let layer = &grid[z];
            for p in brick.layer_iter() {
                if layer[p] != n {
                    dependencies.insert(layer[p]);
                }
            }
            if !dependencies.is_empty() {
                next_z = z + 1;
                break;
            }
        }

        num_dependencies[i] = dependencies.len();
        for dep in dependencies {
            dependents[dep].insert(i);
        }

        if next_z != bottom {
            for (p, z) in brick.iter() {
                grid[z][p] = n;
            }
            let top = *brick.z.end() - (bottom - next_z);
            brick.z = next_z..=top;
            for (p, z) in brick.iter() {
                grid[z][p] = i;
            }

            for dependent in dependents[i].drain() {
                num_dependencies[dependent] -= 1;
                if num_dependencies[dependent] == 0 {
                    queue.push_back(dependent);
                }
            }
        }
    }

    let disintegratable = (0..n)
        .filter(|&i| dependents[i].iter().all(|&j| num_dependencies[j] > 1))
        .count();
    pv!(disintegratable);
}
