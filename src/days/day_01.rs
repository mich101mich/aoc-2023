use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let parsed = input
        .lines()
        .map(|l| {
            let a = digits
                .iter()
                .enumerate()
                .filter_map(|(i, d)| l.find(d).map(|pos| (pos, i + 1)))
                .chain(
                    l.bytes()
                        .enumerate()
                        .find_map(|(i, c)| (c as char).to_digit(10).map(|d| (i, d as usize))),
                )
                .min_by_key(|(pos, _)| *pos)
                .unwrap()
                .1;

            let b = digits
                .iter()
                .enumerate()
                .filter_map(|(i, d)| l.rfind(d).map(|pos| (pos, i + 1)))
                .chain(
                    l.bytes()
                        .enumerate()
                        .rev()
                        .find_map(|(i, c)| (c as char).to_digit(10).map(|d| (i, d as usize))),
                )
                .max_by_key(|(pos, _)| *pos)
                .unwrap()
                .1;

            a * 10 + b
        })
        .sum::<usize>();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let parsed = input
        .lines()
        .map(|l| {
            let a = l.chars().find_map(|c| c.to_digit(10)).unwrap();
            let b = l.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            a * 10 + b
        })
        .sum::<u32>();

    pv!(parsed);
}
