
fn exists_in_win(m_char: char, s: &str, offset: usize, rad: usize) -> bool {
    let start_at = (offset as i32 - rad as i32).max(0) as usize;
    let end_at = (offset + rad).min(s.len());
    return s[start_at..end_at].chars().any(|c| c == m_char)
}

fn common_chars(chars1: &str, chars2: &str, match_radius: usize) -> Vec<char> {
    let mut result = Vec::with_capacity(chars1.len());
    result.extend(
        chars1.char_indices().filter_map(|(i, c)| {
            if exists_in_win(c, chars2, i, match_radius) {
                Some(c)
            } else {
                None
            }
        })
    );
    result
}

pub fn jaro(s1: &str, s2: &str) -> f64 {
    let match_radius = {
        let min_len = s1.len().min(s2.len());
        min_len / 2 + min_len % 2
    };

    let c1 = common_chars(s1, s2, match_radius);
    let c2 = common_chars(s2, s1, match_radius);
    let c1length = c1.len() as f64;
    let c2length = c2.len() as f64;

    let transpositions = {
        let mismatches = c1.iter().zip(c2.iter()).filter(|&(x, y)| x != y).count();
        (mismatches as f64 + (c1length - c2length).abs()) / 2.0
    };

    let t_length = c1length.max(c2length);
    let result = (c1length / s1.len() as f64 + c2length / s2.len() as f64 + (t_length - transpositions) / t_length) / 3.0;
    if result.is_nan() { 0.0 } else { result }
}
