#[macro_use]
extern crate lazy_static;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
// mod day_5; // this takes a couple seconds to run in release mode, skip typically
mod day_6;
mod day_7;
mod day_8;
mod day_9;
// ---- comment to keep logical ordering of mods
mod day_10;
// mod day_11; // this takes ~25s to run in release mode, skip usually
// mod day_12; // about 2s
mod day_13;
// mod day_14; // about 35s
// mod day_15; // about 1.25s
// mod day_16; // about 0.35s

mod md5;

fn main() {
  println!("{}", "See the individual files and their tests!");
}
