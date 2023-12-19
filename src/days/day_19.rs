use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromScanf)]
#[sscanf(autogen = "lowercase")]
enum Target {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

#[derive(Debug, Clone, Copy, FromScanf)]
enum Matcher {
    #[sscanf("{target}{comp}{n}:{rule}")]
    Specific {
        target: Target,
        comp: Comp,
        n: usize,
        rule: &'static str,
    },
    #[sscanf("{rule}")]
    Any { rule: &'static str },
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let mut iter = input.lines();

    let workflows = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| sscanf!(l, "{str}{{{str}}}").unwrap())
        .map(|(name, list)| {
            let list = list
                .split(',')
                .map(|s| sscanf!(s, "{Matcher}").unwrap())
                .to_vec();
            (name, list)
        })
        .to_map();

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct State {
        pos: &'static str,
        ranges: [(usize, usize); 4],
    }

    let mut queue = vec![State {
        pos: "in",
        ranges: [(1, 4000); 4],
    }];

    let mut sum = 0u128;
    while let Some(mut state) = queue.pop() {
        if state.pos == "A" {
            sum += state
                .ranges
                .iter()
                .map(|(a, b)| (b - a + 1) as u128)
                .product::<u128>();
            continue;
        } else if state.pos == "R" {
            continue;
        }

        let workflow = &workflows[&state.pos];
        for matcher in workflow {
            match *matcher {
                Matcher::Specific {
                    target,
                    comp,
                    n,
                    rule,
                } => {
                    let target = target as usize;
                    let (start, end) = state.ranges[target];
                    match comp {
                        Comp::Gt => {
                            //                   n
                            // range                  |------| --> matches
                            // range    |---|                  --> continues
                            // range  |------------------|
                            //        |----------|             --> continues
                            //                    |------|     --> matches
                            if start > n {
                                // all matches
                                state.pos = rule;
                                queue.push(state);
                                break;
                            } else if end <= n {
                                // all continues
                            } else {
                                // split
                                let mut state2 = state;
                                state.ranges[target] = (start, n);
                                state2.ranges[target] = (n + 1, end);
                                state2.pos = rule;
                                queue.push(state2);
                                // `state` continues
                            }
                        }
                        Comp::Lt => {
                            //                   n
                            // range                  |------| --> continues
                            // range    |---|                  --> matches
                            // range  |------------------|
                            //        |---------|              --> matches
                            //                   |-------|     --> continues
                            if start >= n {
                                // all continues
                            } else if end < n {
                                // all matches
                                state.pos = rule;
                                queue.push(state);
                                break;
                            } else {
                                // split
                                let mut state2 = state;
                                state.ranges[target] = (n, end);
                                state2.ranges[target] = (start, n - 1);
                                state2.pos = rule;
                                queue.push(state2);
                                // `state` continues
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                Matcher::Any { rule } => {
                    state.pos = rule;
                    queue.push(state);
                    break;
                }
            }
        }
    }

    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let mut iter = input.lines();

    let workflows = iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| sscanf!(l, "{str}{{{str}}}").unwrap())
        .map(|(name, list)| {
            let list = list
                .split(',')
                .map(|s| sscanf!(s, "{Matcher}").unwrap())
                .to_vec();
            (name, list)
        })
        .to_map();

    let mut sum = 0;
    for part in iter {
        let part = part.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let part = part
            .split(',')
            .map(|s| sscanf!(s, "{Target}={usize}").unwrap())
            .to_map();

        let mut pos = "in";
        while !matches!(pos, "A" | "R") {
            let workflow = &workflows[pos];
            for matcher in workflow {
                match matcher {
                    Matcher::Specific {
                        target,
                        comp,
                        n,
                        rule,
                    } => {
                        let val = part.get(target).unwrap_or(&0);
                        if comp.apply(*val, *n) {
                            pos = rule;
                            break;
                        }
                    }
                    Matcher::Any { rule } => {
                        pos = rule;
                    }
                }
            }
        }
        if pos == "A" {
            for (_, val) in part {
                sum += val;
            }
        }
    }

    pv!(sum);
}
