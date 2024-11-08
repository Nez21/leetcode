impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![i32::MIN; n]; 4];

        dp[0][0] = -prices[0];
        dp[1][0] = 0;
        dp[2][0] = -prices[0];
        dp[3][0] = 0;

        for i in 1..n {
            dp[0][i] = dp[0][i - 1].max(-prices[i]);
            dp[2][i] = dp[2][i - 1].max(-prices[i]);
        }

        for i in 1..4 {
            for j in 1..n {
                if i % 2 == 0 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1]).max(dp[i - 1][j - 1] - prices[j]);
                } else {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1]).max(dp[i - 1][j - 1] + prices[j]);
                }
            }
        }

        dp[3][n - 1].max(0)
    }
}