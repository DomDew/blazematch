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
