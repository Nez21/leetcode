impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut max_rl = vec![i32::MIN; n];

        for i in (0..n - 1).rev() {
            max_rl[i] = max_rl[i + 1].max(values[i + 1] - i as i32 - 1);
        }

        values
            .iter()
            .enumerate()
            .zip(max_rl.iter())
            .fold(i32::MIN, |acc, ((i, &val), &r)| acc.max(i as i32 + val + r))
    }
}