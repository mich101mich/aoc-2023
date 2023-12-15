use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let mut boxes = vec![Vec::<(&str, usize)>::new(); 256];

    let parsed = input.split(',').for_each(|s| {
        let mut n = None;
        let mut remove = false;
        let label = if s.ends_with('-') {
            remove = true;
            &s[..s.len() - 1]
        } else if let Ok((l, x)) = sscanf!(s, "{str}={usize}") {
            n = Some(x);
            l
        } else {
            unreachable!()
        };

        let mut v = 0;
        for c in label.chars() {
            v = (v + (c as usize)) * 17 % 256;
        }

        let b = &mut boxes[v];
        let existing = b.iter().position(|(l, _)| *l == label);
        if remove {
            if let Some(i) = existing {
                b.remove(i);
            }
        } else if let Some(i) = existing {
            b[i].1 = n.unwrap();
        } else {
            b.push((label, n.unwrap()));
        }
    });

    let result = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(move |(j, (l, n))| (i + 1) * (j + 1) * n)
                .sum::<usize>()
        })
        .sum::<usize>();

    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let parsed = input
        .split(',')
        .map(|s| {
            let mut v = 0;
            for c in s.chars() {
                v = (v + (c as usize)) * 17 % 256;
            }
            v
        })
        .sum::<usize>();

    pv!(parsed);
}
