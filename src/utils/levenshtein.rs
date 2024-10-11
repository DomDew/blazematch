pub fn get_distance(
    s1: &str,
    s2: &str,
    del_cost: usize,
    ins_cost: usize,
    sub_cost: usize,
) -> usize {
    let m = s1.len();
    let n = s2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i * del_cost;
    }
    for j in 0..=n {
        dp[0][j] = j * ins_cost;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                sub_cost
            };
            dp[i][j] = std::cmp::min(
                std::cmp::min(dp[i - 1][j] + del_cost, dp[i][j - 1] + ins_cost),
                dp[i - 1][j - 1] + cost,
            );
        }
    }

    dp[m][n]
}

pub fn normalize_similarity(shorter_string_length: usize, distance: usize) -> f64 {
    if distance > shorter_string_length {
        return 0.0;
    }

    return 1.0 / f64::exp(distance as f64 / (shorter_string_length as f64 - distance as f64));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distance() {
        assert_eq!(get_distance("kitten", "sitting", 1, 1, 1), 3);
        assert_eq!(get_distance("flaw", "lawn", 1, 1, 1), 2);
        assert_eq!(get_distance("gumbo", "gambol", 1, 1, 1), 2);
        assert_eq!(get_distance("book", "back", 1, 1, 1), 2);
        assert_eq!(get_distance("intention", "execution", 1, 1, 1), 5);
    }

    #[test]
    fn test_get_distance_with_custom_costs() {
        assert_eq!(get_distance("kitten", "sitting", 2, 2, 1), 4);
        assert_eq!(get_distance("flaw", "lawn", 1, 2, 3), 3);
        assert_eq!(get_distance("gumbo", "gambol", 2, 2, 2), 4);
        assert_eq!(get_distance("book", "back", 1, 3, 2), 4);
        assert_eq!(get_distance("intention was", "execution is", 3, 3, 2), 15);
    }

    #[test]
    fn test_normalize_similarity() {
        assert_eq!(normalize_similarity(6, 3), 0.36787944117144233);
        assert_eq!(normalize_similarity(15, 2), 0.8574039191604412);
        assert_eq!(normalize_similarity(84, 62), 0.059714415732185264);
    }

    #[test]
    fn test_normalize_similarity_with_distance_greater_than_length() {
        assert_eq!(normalize_similarity(3, 4), 0.0);
        assert_eq!(normalize_similarity(5, 6), 0.0);
    }
}
