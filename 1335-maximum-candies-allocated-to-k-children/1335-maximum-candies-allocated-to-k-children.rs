impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let sum = candies.iter().fold(0i64, |acc, el| acc + *el as i64);

        if sum < k {
            return 0;
        }

        (1..=(sum / k))
            .collect::<Vec<_>>()
            .partition_point(|q| candies.iter().fold(0, |acc, el| acc + *el as i64 / *q) >= k)
            as i32
    }
}