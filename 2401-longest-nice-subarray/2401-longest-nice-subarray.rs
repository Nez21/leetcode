impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut l = 0;
        let mut sum = nums[0];
        let mut max = 1;

        for r in 1..n {
            while sum + nums[r] != sum | nums[r] && l < r {
                sum -= nums[l];
                l += 1;
            }

            sum += nums[r];
            max = max.max(r - l + 1);
        }

        max as i32
    }
}