impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let left = nums.partition_point(|num| *num < 0);
        let right = nums[left..].partition_point(|num| *num == 0);

        left.max(nums.len() - left - right) as i32
    }
}