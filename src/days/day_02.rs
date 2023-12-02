use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromScanf, Hash)]
#[sscanf(autogen = "lowercase")]
enum Color {
    Red,
    Green,
    Blue,
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "Game {usize}: {str}").unwrap())
        .map(|(n, s)| {
            let necessary = s
                .split("; ")
                .map(|part| {
                    part.split(", ")
                        .map(|cube| sscanf!(cube, "{usize} {Color}").unwrap())
                        .fold(HashMap::new(), |mut acc, (n, c)| {
                            *acc.entry(c).or_insert(0) += n;
                            acc
                        })
                })
                .fold(HashMap::new(), |mut acc, cubes| {
                    for (color, n) in cubes {
                        match acc.entry(color) {
                            Entry::Occupied(mut e) => {
                                *e.get_mut() = n.max(*e.get());
                            }
                            Entry::Vacant(e) => {
                                e.insert(n);
                            }
                        }
                    }
                    acc
                });
            necessary.values().product::<usize>()
        })
        .sum::<usize>();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let available = [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]
        .into_iter()
        .to_map();

    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "Game {usize}: {str}").unwrap())
        .map(|(n, s)| {
            let possible = !s
                .split("; ")
                .map(|part| {
                    part.split(", ")
                        .map(|cube| sscanf!(cube, "{usize} {Color}").unwrap())
                        .fold(HashMap::new(), |mut acc, (n, c)| {
                            *acc.entry(c).or_insert(0) += n;
                            acc
                        })
                })
                .any(|cubes| cubes.iter().any(|(c, n)| *n > available[c]));
            if possible {
                n
            } else {
                0
            }
        })
        .sum::<usize>();

    pv!(parsed);
}
