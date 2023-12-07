use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let mut strength_map = [('T', 10), ('J', 1), ('Q', 12), ('K', 13), ('A', 14)]
        .into_iter()
        .to_map();
    for c in '0'..='9' {
        strength_map.insert(c, c.to_digit(10).unwrap() as usize);
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
    enum SetType {
        High,
        Pair,
        TwoPair,
        Three,
        FullHouse,
        Four,
        Five,
    }
    impl From<&HashMap<char, usize>> for SetType {
        fn from(occurrences: &HashMap<char, usize>) -> Self {
            let highest = occurrences.values().max().unwrap();
            match (occurrences.len(), highest) {
                (1, _) => SetType::Five,
                (2, 3) => SetType::FullHouse,
                (2, 4) => SetType::Four,
                (3, 2) => SetType::TwoPair,
                (3, 3) => SetType::Three,
                (4, _) => SetType::Pair,
                (5, _) => SetType::High,
                _ => unreachable!("Invalid set: {:?}", occurrences),
            }
        }
    }
    fn jokerize(occurrences: &HashMap<char, usize>, jokers: usize) -> SetType {
        if jokers == 0 {
            return SetType::from(occurrences);
        }
        let mut next_occurrences = occurrences.clone();
        occurrences
            .keys()
            .map(|c| {
                *next_occurrences.get_mut(c).unwrap() += 1;
                let ret = jokerize(&next_occurrences, jokers - 1);
                *next_occurrences.get_mut(c).unwrap() -= 1;
                ret
            })
            .max()
            .unwrap()
    }
    impl From<&str> for SetType {
        fn from(s: &str) -> Self {
            let mut occurrences = HashMap::new();
            for c in s.chars() {
                *occurrences.entry(c).or_insert(0) += 1;
            }
            let jokers = occurrences.remove(&'J').unwrap_or(0);
            if jokers == 5 {
                return SetType::Five;
            }
            jokerize(&occurrences, jokers)
        }
    }

    let mut parsed = input
        .lines()
        .map(|l| sscanf!(l, "{str} {usize}").unwrap())
        .map(|(s, n)| {
            (
                SetType::from(s),
                s.chars().map(|c| strength_map[&c]).to_vec(),
                n,
            )
        })
        .to_vec();

    parsed.sort_unstable();

    let score = parsed
        .iter()
        .enumerate()
        .map(|(i, (_, _, n))| n * (i + 1))
        .sum::<usize>();

    pv!(score);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let mut strength_map = [('T', 10), ('J', 11), ('Q', 12), ('K', 13), ('A', 14)]
        .into_iter()
        .to_map();
    for c in '0'..='9' {
        strength_map.insert(c, c.to_digit(10).unwrap() as usize);
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
    enum SetType {
        High,
        Pair,
        TwoPair,
        Three,
        FullHouse,
        Four,
        Five,
    }
    impl From<&str> for SetType {
        fn from(s: &str) -> Self {
            let mut occurrences = HashMap::new();
            for c in s.chars() {
                *occurrences.entry(c).or_insert(0) += 1;
            }
            let highest = occurrences.values().max().unwrap();
            match (occurrences.len(), highest) {
                (1, _) => SetType::Five,
                (2, 3) => SetType::FullHouse,
                (2, 4) => SetType::Four,
                (3, 2) => SetType::TwoPair,
                (3, 3) => SetType::Three,
                (4, _) => SetType::Pair,
                (5, _) => SetType::High,
                _ => unreachable!("Invalid set: {} ({:?})", s, occurrences),
            }
        }
    }

    let mut parsed = input
        .lines()
        .map(|l| sscanf!(l, "{str} {usize}").unwrap())
        .map(|(s, n)| {
            (
                SetType::from(s),
                s.chars().map(|c| strength_map[&c]).to_vec(),
                n,
            )
        })
        .to_vec();

    parsed.sort_unstable();

    let score = parsed
        .iter()
        .enumerate()
        .map(|(i, (_, _, n))| n * (i + 1))
        .sum::<usize>();

    pv!(score);
}
