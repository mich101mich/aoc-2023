use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    fn recurse<'a>(
        springs: &'a [char],
        nums: &'a [usize],
        seen: &mut HashMap<&'a [char], HashMap<&'a [usize], usize>>,
    ) -> usize {
        let first_relevant = springs
            .iter()
            .position(|c| *c != '.')
            .unwrap_or(springs.len());
        let springs = &springs[first_relevant..];

        if nums.is_empty() {
            // no more numbers => either done or impossible
            if springs.iter().all(|c| *c != '#') {
                // all remaining tiles can be turned to '.' => done
                return 1;
            } else {
                // some definitely can't => impossible
                return 0;
            }
        }

        // minimum length: just the blocks with one empty space between them
        let min_len = nums.iter().sum::<usize>() + nums.len() - 1;
        if springs.len() < min_len {
            return 0;
        }

        // check the cache
        if let Some(&count) = seen.get(springs).and_then(|m| m.get(nums)) {
            return count;
        }

        let (n, next_nums) = nums.split_first().unwrap();
        let mut count = 0;
        // find all the possible places to put the next block `n`
        for start in 0..=springs.len() - min_len {
            if springs[start] == '.' {
                continue;
            }
            // possible: block has to consist of '#' or '?'
            let mut possible = springs[start..start + n].iter().all(|c| *c != '.');
            // possible 2: block has to be followed by an empty space, so '.', '?' or end of string
            if start + n < springs.len() && springs[start + n] == '#' {
                possible = false;
            }
            if !possible {
                if springs[start] == '#' {
                    // block _has_ to start here at the latest, so we can stop looking
                    break;
                } else {
                    continue;
                }
            }
            if start + n < springs.len() {
                // start + n => block. +1 for the empty space after the block
                count += recurse(&springs[start + n + 1..], next_nums, seen);
            } else if next_nums.is_empty() {
                // no more numbers => we're done (see reasoning at the start of the function)
                count += 1;
            }
            if springs[start] == '#' {
                // block _has_ to start here at the latest, so we can stop looking
                break;
            }
        }

        seen.entry(springs).or_default().insert(nums, count);

        count
    }

    let parsed = input
        .lines()
        .map(|l| {
            let mut iter = l.splitn(2, ' ');
            let single_springs = iter.next().unwrap().chars().to_vec();
            let mut springs = single_springs.clone();
            for i in 0..4 {
                springs.push('?');
                springs.extend_from_slice(&single_springs);
            }
            while let Some('.') = springs.last() {
                springs.pop();
            }

            let nums = iter
                .next()
                .unwrap()
                .split(',')
                .map(parse_u)
                .to_vec()
                .repeat(5);
            (springs, nums)
        })
        .to_vec();

    let mut seen = HashMap::new();
    let mut result = 0;
    for (springs, nums) in &parsed {
        result += recurse(&springs, &nums, &mut seen);
    }

    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    fn recurse(springs: &[char], nums: &[usize]) -> usize {
        if nums.is_empty() {
            if springs.iter().all(|c| *c != '#') {
                return 1;
            } else {
                return 0;
            }
        }
        let min_len = nums.iter().sum::<usize>() + nums.len() - 1;
        if springs.len() < min_len {
            return 0;
        }

        let (n, nums) = nums.split_first().unwrap();
        let mut count = 0;
        for start in 0..=springs.len() - min_len {
            if springs[start] == '.' {
                continue;
            }
            let mut possible = springs[start..start + n].iter().all(|c| *c != '.');
            if start + n < springs.len() && springs[start + n] == '#' {
                possible = false;
            }
            if !possible {
                if springs[start] == '#' {
                    break;
                } else {
                    continue;
                }
            }
            if start + n < springs.len() {
                count += recurse(&springs[start + n + 1..], nums);
            } else {
                count += recurse(&[], nums);
            }
            if springs[start] == '#' {
                break;
            }
        }

        count
    }

    let parsed = input
        .lines()
        .map(|l| {
            let mut iter = l.splitn(2, ' ');
            let springs = iter.next().unwrap().chars().to_vec();
            let nums = iter.next().unwrap().split(',').map(parse_u).to_vec();
            recurse(&springs, &nums)
        })
        .sum::<usize>();

    pv!(parsed);
}
