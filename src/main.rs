#![feature(test)]

extern crate time;
extern crate test;

mod jaro;
mod tests;

use time::PreciseTime;


fn main() {
    let s1 = "Environment";
    let s2 = "Envronment";
    println!("{}, {} => {}", s1, s2, jaro::jaro(s1, s2));

    let start = PreciseTime::now();

    for _ in 0..10_000_000 {
        //    for _ in 0..100_000 {
        jaro::jaro(s1, s2);
    }

    let end = PreciseTime::now();
    println!("Elapsed {}", start.to(end))
}