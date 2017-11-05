extern crate time;

use time::PreciseTime;

fn exists_in_win (m_char: char, s: &str, offset: i32, rad: i32) -> bool {
    let start_at = (offset - rad).max(0);
    let end_at = (offset + rad).min(s.len() as i32);
    if end_at - start_at < 0 { false } else {
        for i in start_at..end_at {
            if s.chars().nth(i as usize).unwrap() == m_char {
                return true
            }
        }
        return false
    }
}

fn jaro(s1: &str, s2: &str) -> f64 {

    // An inner function which recursively finds the number of matched characters within the radius.
    fn common_chars(chars1: &str, chars2: &str, match_radius: i32) -> Vec<char> {
        chars1
            .chars()
            .rev()
            .enumerate()
            .filter(|&(i, c)| exists_in_win(c, chars2, i as i32, match_radius))
            .map(|(_,c)| c)
            .collect()
    }

    // The radius is half of the lesser of the two string lengths rounded up.
    let match_radius = {
        let min_len = s1.len().min(s2.len()) as i32;
        min_len / 2 + min_len % 2
    };

    // The sets of common characters and their lengths as floats
    let c1 = common_chars(s1, s2, match_radius);
    let c2 = common_chars(s2, s1, match_radius);
    let c1length = c1.len() as f64;
    let c2length = c2.len() as f64;

    // The number of transpositions within the sets of common characters.
    let transpositions = {
        let mut mismatches = 0.0;
        for i in 0..c1.len().min(c2.len()) {
            if c1[i] != c2[i] {
                mismatches += 1.0
            }
        }

        // If one common string is longer than the other each additional char counts as half a transposition
        (mismatches + (c1length - c2length).abs()) / 2.0
    };

    let t_length = c1length.max(c2length);

    // The jaro distance as given by 1/3 ( m2/|s1| + m1/|s2| + (mc-t)/mc )
    let result = (c1length / s1.len() as f64 + c2length / s2.len() as f64 + (t_length - transpositions) / t_length) / 3.0;

    // This is for cases where |s1|, |s2| or m are zero
    if result.is_nan() { 0.0 } else { result }
}

fn main() {
    let s1 = "Environment";
    let s2 = "Envronment";

    let start = PreciseTime::now();

    for _ in 0..10_000_000 {
        jaro(s1, s2);
    }

    let end = PreciseTime::now();
    println!("Elapsed {}", start.to(end))
}