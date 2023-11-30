#![allow(unused_imports)]

#[macro_use]
mod utils;
mod days {
    pub mod day_01;
}

fn main() {
    days::day_01::run();
}
