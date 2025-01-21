impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let m = grid[0].len();

        if m == 1 {
            return 0;
        }

        let mut r1 = vec![0i64; m];
        let mut r2 = vec![0i64; m];

        r1[0] = grid[0][0] as i64;
        r2[0] = grid[1][0] as i64;

        for i in 1..m {
            r1[i] = r1[i - 1] + grid[0][i] as i64;
            r2[i] = r2[i - 1] + grid[1][i] as i64;
        }

        let mut min_d2 = i64::MAX;

        for i in 0..m {
            min_d2 = min_d2.min(
                (if i < m - 1 {
                    r1[m - 1] - r1[i]
                } else {
                    i64::MIN
                })
                .max(if i > 0 { r2[i - 1] } else { i64::MIN }),
            )
        }

        min_d2
    }
}