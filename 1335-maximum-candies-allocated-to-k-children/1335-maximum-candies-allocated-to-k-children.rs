impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let sum = candies.iter().fold(0i64, |acc, el| acc + *el as i64);

        if sum < k {
            return 0;
        }

        let mut l = 1;
        let mut r = sum / k;

        while l < r {
            let m = (l + r + 1) / 2;

            if candies.iter().fold(0, |acc, el| acc + *el as i64 / m) >= k {
                l = m;
            } else {
                r = m - 1;
            }
        }

        l as i32
    }

}