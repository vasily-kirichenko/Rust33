extern crate time;

use time::PreciseTime;

fn exists_in_win(m_char: char, s: &str, offset: usize, rad: usize) -> bool {
    let start_at = (offset as i32 - rad as i32).max(0) as usize;
    let end_at = (offset + rad).min(s.len());
    return s[start_at..end_at].chars().any(|c| c == m_char)
}

fn common_chars(chars1: &str, chars2: &str, match_radius: usize) -> Vec<char> {
    let mut result = Vec::with_capacity(chars1.len());
    let mut count = 0;

    for (i, c) in chars1.char_indices() {
        if exists_in_win(c, chars2, i, match_radius) {
            result.push(c);
        }
    }

    result
}

fn jaro(s1: &str, s2: &str) -> f64 {
    let match_radius = {
        let min_len = s1.len().min(s2.len());
        min_len / 2 + min_len % 2
    };

    let c1 = common_chars(s1, s2, match_radius);
    let c2 = common_chars(s2, s1, match_radius);

    let transpositions = {
        let mismatches = {
            let length = c1.len().min(c2.len());
            let mut mismatches = 0.0;
            for i in 0..length {
                if c1[i] != c2[i] {
                    mismatches += 1.0
                }
            }
            mismatches
        };

        (mismatches + (c1.len() as f64 - c2.len() as f64).abs()) / 2.0
    };

    let t_length = c1.len().max(c2.len()) as f64;
    let result = (c1.len() as f64 / s1.len() as f64 + c2.len() as f64 / s2.len() as f64 + (t_length - transpositions) / t_length) / 3.0;
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