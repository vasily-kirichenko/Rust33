extern crate time;

use time::PreciseTime;

fn exists_in_win(m_char: char, s: &str, offset: usize, rad: usize) -> bool {
    let start_at = (offset as i32 - rad as i32).max(0) as usize;
    let length = (offset + rad).min(s.len()) - start_at;

    s.chars()
        .skip(start_at as usize)
        .take(length as usize)
        .any(|c| c == m_char)
}

fn common_chars(chars1: &str, chars2: &str, match_radius: usize, buff: &mut [char; 12]) -> usize {
    //let mut result = Vec::with_capacity(chars1.len());
    let mut count = 0;

    for (i, c) in chars1.char_indices() {
        if exists_in_win(c, chars2, i, match_radius) {
            buff[count] = c;
            count += 1;
        }
    }

    count
}

fn jaro(s1: &str, s2: &str, buff1: &mut [char; 12], buff2: &mut [char; 12]) -> f64 {
    let match_radius = {
        let min_len = s1.len().min(s2.len());
        min_len / 2 + min_len % 2
    };

    let c1length = common_chars(s1, s2, match_radius, buff1);
    let c2length = common_chars(s2, s1, match_radius, buff2);

    let transpositions = {
        let mismatches = {
            let length = c1length.min(c2length);
            let mut mismatches = 0.0;
            for i in 0..length {
                if buff1[i] != buff2[i] {
                    mismatches += 1.0
                }
            }
            mismatches
        };

        (mismatches + (c1length as f64 - c2length as f64).abs()) / 2.0
    };

    let t_length = c1length.max(c2length) as f64;
    let result = (c1length as f64 / s1.len() as f64 + c2length as f64 / s2.len() as f64 + (t_length - transpositions) / t_length) / 3.0;
    if result.is_nan() { 0.0 } else { result }
}

fn main() {
    let s1 = "Environment";
    let s2 = "Envronment";
    let mut buff1: [char; 12] = [' '; 12];
    let mut buff2: [char; 12] = [' '; 12];
    println!("{}, {} => {}", s1, s2, jaro(s1, s2, &mut buff1, &mut buff2));

    let start = PreciseTime::now();
    println!("buff1 = {:?}", buff1);

    for _ in 0..10_000_000 {
        //    for _ in 0..100_000 {
        jaro(s1, s2, &mut buff1, &mut buff2);
    }

    let end = PreciseTime::now();
    println!("Elapsed {}", start.to(end))
}