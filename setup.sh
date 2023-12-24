#!/bin/bash

DAY=$1

SRC_FILE="src/days/day_${DAY}.rs"
INPUT_FILE="src/input/${DAY}.txt"

echo "use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!(\"../input/${DAY}.txt\");
    // let input = \"\";

    // let mut grid = _grid(input);

    let parsed = input
        //.lines()
        //.chars()
        //.map(parse)
        //.map(|l| sscanf!(l, \"\").unwrap())
        //.map(|()|)
        //.filter(|()|)
        //.to_vec()
        //.sum::<isize>()
        //.count()
        ;

    pv!(parsed);

}" > "${SRC_FILE}"

echo "#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_${DAY};
}

fn main() {
    days::day_${DAY}::run();
}" > src/main.rs

touch "${INPUT_FILE}"

if [ -d /mnt/c ]; then
    cmd.exe /c code "${SRC_FILE}" "${INPUT_FILE}"
else
    code "${SRC_FILE}" "${INPUT_FILE}"
fi
