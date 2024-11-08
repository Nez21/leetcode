impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let max = (2 as i32).pow(n as u32) - 1;
        let mut cur = nums.iter().fold(0, |acc, x| acc ^ x);
        let mut res = Vec::new();

        res.push(max ^ cur);

        for i in (1..nums.len()).rev() {
            cur ^= nums[i];
            res.push(max ^ cur);
        }

        res
    }
}