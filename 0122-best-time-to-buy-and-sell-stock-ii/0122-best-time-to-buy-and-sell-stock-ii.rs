impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![i32::MIN; n]; 2];

        dp[0][0] = -prices[0];
        dp[1][0] = 0;

        for i in 1..n {
            dp[0][i] = dp[0][i - 1].max(dp[1][i - 1] - prices[i]);
            dp[1][i] = dp[1][i - 1].max(dp[0][i - 1] + prices[i]);
        }

        dp[1][n - 1].max(0)
    }
}