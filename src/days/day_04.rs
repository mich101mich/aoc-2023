use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let parsed = input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            assert_eq!(iter.next(), Some("Card"));
            assert!(iter.next().unwrap().ends_with(":"));
            let winning = iter
                .by_ref()
                .take_while(|s| *s != "|")
                .map(parse_u)
                .to_set();
            let have = iter.map(parse_u).to_vec();
            have.iter().filter(|&c| winning.contains(c)).count()
        })
        .to_vec();

    let mut have = vec![1; parsed.len()];

    for i in 0..parsed.len() {
        let copies = have[i];
        have[i + 1..i + 1 + parsed[i]]
            .iter_mut()
            .for_each(|x| *x += copies);
    }

    let total = have.iter().sum::<usize>();
    pv!(total);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let parsed = input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            assert_eq!(iter.next(), Some("Card"));
            assert!(iter.next().unwrap().ends_with(":"));
            let winning = iter
                .by_ref()
                .take_while(|s| *s != "|")
                .map(parse_u)
                .to_set();
            let have = iter.map(parse_u).to_vec();
            let wins = have.iter().filter(|&c| winning.contains(c)).count();
            if wins > 0 {
                1 << (wins - 1)
            } else {
                0
            }
        })
        .sum::<usize>();

    pv!(parsed);
}
