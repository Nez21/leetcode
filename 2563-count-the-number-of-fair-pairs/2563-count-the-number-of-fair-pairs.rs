impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();

        let mut res = 0i64;

        for i in 0..nums.len() {
            let min = lower - nums[i];
            let max = upper - nums[i]; 

            let min_idx = nums[i + 1..].partition_point(|&x| x < min);
            let max_idx = nums[i + 1..].partition_point(|&x| x <= max);

            res += (max_idx - min_idx) as i64;
        }

        res
    }
}