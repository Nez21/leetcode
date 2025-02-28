impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let s1: Vec<char> = str1.chars().collect();
        let s2: Vec<char> = str2.chars().collect();
        let n = s1.len();
        let m = s2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if s1[i] == s2[j] {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                } else {
                    dp[i][j] = dp[i][j + 1].max(dp[i + 1][j]);
                }
            }
        }

        let mut res = String::new();
        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            if s1[i] == s2[j] {
                res.push(s1[i]);
                i += 1;
                j += 1;
            } else if dp[i + 1][j] >= dp[i][j + 1] {
                res.push(s1[i]);
                i += 1;
            } else {
                res.push(s2[j]);
                j += 1;
            }
        }

        while i < n {
            res.push(s1[i]);
            i += 1;
        }
        while j < m {
            res.push(s2[j]);
            j += 1;
        }

        res
    }
}