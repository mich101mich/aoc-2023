use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input
        .lines()
        .map(|l| {
            let mut first_values = vec![];
            let mut row = l.split_whitespace().map(parse).to_vec();
            while !row.iter().all(|v| *v == 0) {
                first_values.push(row[0]);
                row = row.windows(2).map(|w| w[1] - w[0]).to_vec();
            }
            first_values.iter().rev().fold(0, |diff, next| next - diff)
        })
        .sum::<isize>();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");

    let parsed = input
        .lines()
        .map(|l| {
            let mut last_values = vec![];
            let mut row = l.split_whitespace().map(parse).to_vec();
            while !row.iter().all(|v| *v == 0) {
                last_values.push(*row.last().unwrap());
                row = row.windows(2).map(|w| w[1] - w[0]).to_vec();
            }
            last_values.iter().sum::<isize>()
        })
        .sum::<isize>();

    pv!(parsed);
}
