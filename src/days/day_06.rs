use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let mut iter = input.lines();
    let time = parse_u(
        iter.next()
            .unwrap()
            .replace(' ', "")
            .split(':')
            .nth(1)
            .unwrap(),
    );
    let distance = parse_u(
        iter.next()
            .unwrap()
            .replace(' ', "")
            .split(':')
            .nth(1)
            .unwrap(),
    );

    let start = binary_search(0, |i| i * (time - i) > distance);
    let end = binary_search(start + 1, |i| i * (time - i) <= distance);
    let won = end - start;

    pv!(won);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let mut iter = input.lines();
    let time = iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(parse_u)
        .to_vec();
    let distance = iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(parse_u)
        .to_vec();

    let mut prod = 1;

    for (time, distance) in time.iter().zip(distance.iter()) {
        let mut won = 0;
        for i in 0..*time {
            let speed = i;
            let remaining = *time - i;
            let traveled = speed * remaining;
            if traveled > *distance {
                won += 1;
            }
        }
        prod *= won;
    }

    pv!(prod);
}
