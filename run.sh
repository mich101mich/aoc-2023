#!/bin/bash

DAY=$1
PART=${2:-1}

FUNCTION=part_one
if [ $PART = 2 ]; then
    FUNCTION=run
fi

echo "#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_${DAY};
}

fn main() {
    days::day_${DAY}::${FUNCTION}();
}" > src/main.rs

cargo run --release
