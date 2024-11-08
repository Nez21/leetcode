impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let n = dungeon.len();
        let m = dungeon[0].len();
        let mut dp = vec![vec![i32::MAX; m + 1]; n + 1];
        dp[n][m - 1] = 1;
        dp[n - 1][m] = 1;
    
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                dp[i][j] = (dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j]).max(1);
            }
        }

        dp[0][0]
    }
}