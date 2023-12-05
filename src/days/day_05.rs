use nom::InputIter;

use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");
    // let input = "";

    let mut iter = input.lines();

    let seed_numbers = iter
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(parse_u)
        .to_vec();

    iter.next().unwrap(); // empty line

    let mut maps: HashMap<&str, HashMap<&str, _>> = HashMap::new();

    while let Some(header) = iter.next() {
        let (from, to) = sscanf!(header, "{str}-to-{str} map:").unwrap();
        let map = maps
            .entry(from)
            .or_insert_with(HashMap::new)
            .entry(to)
            .or_insert_with(Vec::new);

        for line in iter.by_ref().take_while(|line| !line.is_empty()) {
            let (dest_start, src_start, len) = sscanf!(line, "{usize} {usize} {usize}").unwrap();
            map.push((src_start..src_start + len, dest_start));
        }
    }

    maps.iter_mut().for_each(|(_, map)| {
        map.iter_mut().for_each(|(_, ranges)| {
            ranges.sort_by_key(|(src, _)| src.start);
        });
    });

    let mut position = "seed";

    let mut values = seed_numbers
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .to_vec();

    while position != "location" {
        let targets = &maps[position];
        let (target_name, ranges) = targets.iter().next().unwrap();

        let mut new_values = vec![];

        for mut value in values {
            // ranges: --------  ----       -------------   ----    -------
            // value:                     --------------------

            let mut iter = ranges.iter().peekable();

            while let Some(_) = iter.next_if(|(from, _)| from.end <= value.start) {
                // ranges: --------  ----
                // value:                     --------------------
                // => skip ranges
            }

            for (from, to) in iter {
                if value.is_empty() {
                    break;
                }

                if from.start >= value.end {
                    // ranges:                                              -------
                    // value:                     --------------------
                    // add                        --------------------
                    new_values.push(value.start..value.end);
                    break;
                }

                // ranges:                      -------------   ----
                // value:                     --------------------

                if value.start < from.start {
                    // ranges:                      -------------   ----
                    // value:                     --------------------
                    // add                        --             ---
                    new_values.push(value.start..from.start);
                    value.start = from.start;
                }

                // ranges:                      -------------   ----
                // value:                     --------------------
                // convert                      -------------
                let convert_start = value.start.max(from.start);
                let convert_end = value.end.min(from.end);
                let converted_start = to + convert_start - from.start;
                let converted_end = to + convert_end - from.start;
                new_values.push(converted_start..converted_end);
                value.start = convert_end;
            }
        }
        position = target_name;
        values = new_values;
    }

    let min = values.iter().map(|r| r.start).min().unwrap();
    pv!(min);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut iter = input.lines();

    let seeds = iter
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(parse_u)
        .to_vec();

    iter.next().unwrap(); // empty line

    let mut maps: HashMap<&str, HashMap<&str, _>> = HashMap::new();

    while let Some(header) = iter.next() {
        let (from, to) = sscanf!(header, "{str}-to-{str} map:").unwrap();
        let map = maps
            .entry(from)
            .or_insert_with(HashMap::new)
            .entry(to)
            .or_insert_with(Vec::new);

        for line in iter.by_ref().take_while(|line| !line.is_empty()) {
            let (dest_start, src_start, len) = sscanf!(line, "{usize} {usize} {usize}").unwrap();
            map.push((src_start..src_start + len, dest_start));
        }
    }

    let mut position = "seed";
    let mut values = seeds;

    while position != "location" {
        let targets = &maps[position];
        let (target_name, ranges) = targets.iter().next().unwrap();

        for value in &mut values {
            for (src, dest) in ranges {
                if src.contains(&value) {
                    *value = dest + *value - src.start;
                    break;
                }
            }
        }
        position = target_name;
    }

    let min = values.iter().min().unwrap();
    pv!(min);
}
