use num::integer::{div_ceil, gcd, Roots};

use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    let (mut min_vx, mut min_vy) = (f64::MAX, f64::MAX);
    let (mut max_vx, mut max_vy) = (f64::MIN, f64::MIN);

    type Vector = cgmath::Vector3<f64>;
    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "{f64}, {f64}, {f64} @ {f64}, {f64}, {f64}").unwrap())
        .map(|(px, py, pz, vx, vy, vz)| {
            min_vx = min_vx.min(vx);
            min_vy = min_vy.min(vy);
            max_vx = max_vx.max(vx);
            max_vy = max_vy.max(vy);
            (Vector::new(px, py, pz), Vector::new(vx, vy, vz))
        })
        .to_vec();

    for vx in min_vx as i64..=max_vx as i64 {
        let vx = vx as f64;
        for vy in min_vy as i64..=max_vy as i64 {
            let vy = vy as f64;
            let mut target = None;
            'finder: for (i, (pa, va)) in parsed.iter().enumerate() {
                let va = p2(va.x - vx, va.y - vy);
                for (pb, vb) in &parsed[i + 1..] {
                    let vb = p2(vb.x - vx, vb.y - vy);

                    // position_a := pa + t * va
                    // position_b := pb + t * vb
                    // position_a == position_b
                    // pa.x + ta * va.x == pb.x + tb * vb.x
                    // <=> pa.x - pb.x + ta * va.x == tb * vb.x
                    // <=> tb == (pa.x - pb.x) / vb.x + ta * va.x / vb.x
                    //
                    // pa.y + ta * va.y == pb.y + tb * vb.y
                    // <=> pa.y + ta * va.y == pb.y + ((pa.x - pb.x) / vb.x + ta * va.x / vb.x) * vb.y
                    // <=> pa.y + ta * va.y == pb.y + (pa.x - pb.x) / vb.x * vb.y + ta * va.x / vb.x * vb.y
                    // <=> ta * va.y - ta * va.x / vb.x * vb.y == pb.y - pa.y + (pa.x - pb.x) / vb.x * vb.y
                    // <=> ta * (va.y - va.x / vb.x * vb.y) == pb.y - pa.y + (pa.x - pb.x) / vb.x * vb.y
                    // <=> ta == (pb.y - pa.y + (pa.x - pb.x) / vb.x * vb.y) / (va.y - va.x / vb.x * vb.y)

                    if vb.x == 0.0 {
                        continue;
                    }

                    let denom = va.y - va.x * vb.y / vb.x;
                    if denom == 0.0 {
                        // parallel lines or identical lines
                        continue;
                    }
                    let numerator = pb.y - pa.y + (pa.x - pb.x) * vb.y / vb.x;
                    let ta = f64::floor(numerator / denom);
                    if ta < 0.0 {
                        continue;
                    }
                    let tb = f64::floor((pa.x - pb.x + ta * va.x) / vb.x);
                    if tb < 0.0 {
                        continue;
                    }
                    if p2(pa.x, pa.y) + ta * va != p2(pb.x, pb.y) + tb * vb {
                        continue;
                    }
                    target = Some(p2(pa.x, pa.y) + ta * va);
                    break 'finder;
                }
            }
            let Some(target) = target else {
                continue;
            };
            let correct = parsed.iter().all(|(p, v)| {
                let v = p2(v.x - vx, v.y - vy);
                let diff = target - p2(p.x, p.y);
                if v == p2(0.0, 0.0) {
                    return diff == p2(0.0, 0.0);
                }
                let tx = diff.x / v.x;
                let ty = diff.y / v.y;
                if v.x == 0.0 {
                    if diff.x != 0.0 {
                        return false;
                    }
                    return ty.floor() == ty;
                }
                if v.y == 0.0 {
                    if diff.y != 0.0 {
                        return false;
                    }
                    return tx.floor() == tx;
                }
                tx.floor() == tx && tx == ty
            });
            if correct {
                let (p0, v0) = parsed[0];
                let (p1, v1) = parsed[1];
                let t0 = (target.x - p0.x) / (v0.x - vx);
                let t1 = (target.x - p1.x) / (v1.x - vx);
                // p0 + t0 * (v0 - v) == p1 + t1 * (v1 - v)
                // <=> p0 + t0 * v0 - t0 * v == p1 + t1 * v1 - t1 * v
                // <=> t1 * v - t0 * v == p1 + t1 * v1 - p0 - t0 * v0
                // <=> v * (t1 - t0) == p1 + t1 * v1 - p0 - t0 * v0
                // <=> v == (p1 + t1 * v1 - p0 - t0 * v0) / (t1 - t0)
                let vz = (p1.z + t1 * v1.z - p0.z - t0 * v0.z) / (t1 - t0);
                let target = p0 + t0 * (v0 - p3(vx, vy, vz));
                let result = (target.x + target.y + target.z) as i64;
                pv!(vx, vy, vz, target, result);
                return;
            }
        }
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");
    let area_min = 200000000000000.0;
    let area_max = 400000000000000.0;

    type Vector = cgmath::Vector2<f64>;
    let parsed = input
        .lines()
        .map(|l| sscanf!(l, "{f64}, {f64}, {f64} @ {f64}, {f64}, {f64}").unwrap())
        .map(|(px, py, _, vx, vy, _)| (Vector::new(px, py), Vector::new(vx, vy)))
        .to_vec();

    let mut count = 0;
    for (i, (pa, va)) in parsed.iter().enumerate() {
        for (pb, vb) in parsed[i + 1..].iter() {
            // position_a := pa + t * va
            // position_b := pb + t * vb
            // position_a == position_b
            // pa.x + ta * va.x == pb.x + tb * vb.x
            // <=> pa.x - pb.x + ta * va.x == tb * vb.x
            // <=> tb == (pa.x - pb.x) / vb.x + ta * va.x / vb.x
            //
            // pa.y + ta * va.y == pb.y + tb * vb.y
            // <=> pa.y + ta * va.y == pb.y + ((pa.x - pb.x) / vb.x + ta * va.x / vb.x) * vb.y
            // <=> pa.y + ta * va.y == pb.y + (pa.x - pb.x) / vb.x * vb.y + ta * va.x / vb.x * vb.y
            // <=> ta * va.y - ta * va.x / vb.x * vb.y == pb.y - pa.y + (pa.x - pb.x) / vb.x * vb.y
            // <=> ta * (va.y - va.x / vb.x * vb.y) == pb.y - pa.y + (pa.x - pb.x) / vb.x * vb.y
            // <=> ta == (pb.y - pa.y + (pa.x - pb.x) / vb.x * vb.y) / (va.y - va.x / vb.x * vb.y)

            if vb.x == 0.0 {
                if pa.x == pb.x {
                    // x always a match, just check y
                    todo!();
                }
                continue;
            }

            let denom = va.y - va.x * vb.y / vb.x;
            if denom == 0.0 {
                // parallel lines or identical lines
                // identical <=> pa == pb + t * vb
                // <=> pa - pb == t * vb
                // <=> (pa - pb) / vb == t
                let tx = (pa.x - pb.x) / vb.x; // vb.x != 0.0 see above
                let pa_y = pa.y + tx * vb.y;
                if pa_y == pb.y {
                    // identical
                    count += 1;
                }
                // else parallel
                continue;
            }
            let numerator = pb.y - pa.y + (pa.x - pb.x) * vb.y / vb.x;
            let ta = numerator / denom;
            if ta < 0.0 {
                continue;
            }
            let tb = (pa.x - pb.x) / vb.x + ta * va.x / vb.x;
            if tb < 0.0 {
                continue;
            }
            let intersection = pa + ta * va;
            if intersection.x < area_min || intersection.x > area_max {
                continue;
            }
            if intersection.y < area_min || intersection.y > area_max {
                continue;
            }
            count += 1;
        }
    }

    pv!(count);
}
