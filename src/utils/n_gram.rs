use std::collections::HashSet;

fn generate_n_grams(text: &str, n: usize) -> Vec<&str> {
    let n_grams = text
        .as_bytes()
        .windows(n)
        .map(|window| std::str::from_utf8(window).unwrap())
        .collect();
    return n_grams;
}

pub fn get_similarity(s1: &str, s2: &str, n: usize) -> f64 {
    let n_grams_s1 = generate_n_grams(s1, n);
    let n_grams_s2 = generate_n_grams(s2, n);

    let set_s1: HashSet<_> = n_grams_s1.iter().cloned().collect();
    let set_s2: HashSet<_> = n_grams_s2.iter().cloned().collect();

    let common_count = set_s1.intersection(&set_s2).count();
    let total_unique = set_s1.union(&set_s2).count();

    if total_unique > 0 {
        common_count as f64 / total_unique as f64
    } else {
        0.0
    }
}
