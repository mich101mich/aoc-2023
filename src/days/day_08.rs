use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let mut iter = input.lines();
    let instructions = iter.next().unwrap();
    iter.next().unwrap();

    let next_map = iter
        .map(|l| sscanf!(l, "{str} = ({str}, {str})").unwrap())
        .map(|(out, l, r)| (out, (l, r)))
        .to_map();

    let num_locations = next_map.len();
    let name_to_index = next_map.keys().copied().zip(0..).to_map();
    let names = next_map.keys().copied().to_vec();

    let mut left = vec![0; num_locations];
    let mut right = vec![0; num_locations];
    let mut is_z = vec![false; num_locations];
    let mut state = vec![];
    for (out, (l, r)) in next_map {
        let i = name_to_index[&out];
        left[i] = name_to_index[&l];
        right[i] = name_to_index[&r];
        is_z[i] = out.ends_with('Z');
        if out.ends_with('A') {
            state.push(i);
        }
    }

    let mut cycles = vec![];

    for mut i in state {
        let mut start = 0;
        let mut offset = 0;
        for ((ip, c), steps) in instructions.chars().enumerate().cycle().zip(0..) {
            if is_z[i] {
                start = steps;
                offset = ip;
                break;
            }
            i = if c == 'L' { left[i] } else { right[i] };
        }
        let backup_i = i;
        let mut seen = HashSet::new();
        let mut real_cycle_len = HashSet::new();
        let mut cycle_start = 0;
        for ((ip, c), steps) in instructions
            .chars()
            .enumerate()
            .cycle()
            .skip(offset)
            .zip(1..)
        {
            i = if c == 'L' { left[i] } else { right[i] };
            if is_z[i] {
                let cycle_len = steps - cycle_start;
                real_cycle_len.insert(cycle_len);
                cycle_start = steps;
            }
            if !seen.insert((i, ip)) {
                break;
            }
        }
        assert_eq!(real_cycle_len.len(), 1); // AoC being nice
        let cycle_len = *real_cycle_len.iter().next().unwrap();
        assert_eq!(start, cycle_len); // AoC being nice
        cycles.push(cycle_len as u64);
    }

    let total_cycle = cycles.iter().copied().reduce(num::integer::lcm).unwrap();
    pv!(total_cycle);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let mut iter = input.lines();
    let instructions = iter.next().unwrap();
    iter.next().unwrap();

    let locations = iter
        .map(|l| sscanf!(l, "{str} = ({str}, {str})").unwrap())
        .map(|(out, l, r)| (out, (l, r)))
        .to_map();

    let mut pos = "AAA";
    let mut steps = 0;
    for c in instructions.chars().cycle() {
        let (l, r) = locations[pos];
        pos = if c == 'L' { l } else { r };
        steps += 1;
        if pos == "ZZZ" {
            break;
        }
    }
    pv!(steps);
}
