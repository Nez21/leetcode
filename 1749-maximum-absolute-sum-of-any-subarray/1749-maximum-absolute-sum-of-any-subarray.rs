impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut max = 0;
        let mut res = 0;

        for num in nums {
            min = (min + num).min(num);
            max = (max + num).max(num);
            res = res.max(min.abs().max(max.abs()))
        }

        res
    }
}