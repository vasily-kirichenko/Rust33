extern crate time;

use time::PreciseTime;
use std::collections::*;

fn exists_in_win(m_char: char, s: &str, offset: i32, rad: i32) -> bool {
    let start_at = (offset - rad).max(0);
    let end_at = (offset + rad).min(s.len() as i32);

    if end_at - start_at < 0 {
        false
    } else {
        s.chars()
            .skip(start_at as usize)
            .take((end_at - start_at) as usize)
            .any(|c| c == m_char)
    }
}

fn common_chars(chars1: &str, chars2: &str, match_radius: i32) -> Vec<char> {
    let mut result = Vec::with_capacity(chars1.len());

    for (i, c) in chars1.char_indices().rev() {
        if exists_in_win(c, chars2, i as i32, match_radius) {
            result.push(c);
        }
    }

    result.reverse();
    result
}

fn jaro(s1: &str, s2: &str) -> f64 {
    let match_radius = {
        let min_len = s1.len().min(s2.len()) as i32;
        min_len / 2 + min_len % 2
    };

    let c1 = common_chars(s1, s2, match_radius);
    let c2 = common_chars(s2, s1, match_radius);
    let c1length = c1.len() as f64;
    let c2length = c2.len() as f64;

    let transpositions = {
        let mut mismatches = 0.0;
        for (i, j) in c1.iter().zip(c2.iter()) {
            if i != j {
                mismatches += 1.0
            }
        }

        (mismatches + (c1length - c2length).abs()) / 2.0
    };

    let t_length = c1length.max(c2length);
    let result = (c1length / s1.len() as f64 + c2length / s2.len() as f64 + (t_length - transpositions) / t_length) / 3.0;
    if result.is_nan() { 0.0 } else { result }
}

fn main() {
    let s1 = "Environment";
    let s2 = "Envronment";
    println!("{}, {} => {}", s1, s2, jaro(s1, s2));

    let start = PreciseTime::now();

    for _ in 0..10_000_000 {
        //    for _ in 0..100_000 {
        jaro(s1, s2);
    }

    let end = PreciseTime::now();
    println!("Elapsed {}", start.to(end))
}