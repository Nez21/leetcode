impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut feq = std::collections::HashMap::new();
        let mut max_feq = (0, -1);

        for &num in nums.iter() {
            let entry = feq.entry(num).or_insert(0);
            *entry += 1;

            if *entry > max_feq.0 {
                max_feq = (*entry, num);
            }
        }

        let mut cur_feq = 0;

        for i in 1..=n {
            if nums[i - 1] == max_feq.1 {
                cur_feq += 1;
            }

            if 2 * cur_feq > i && 2 * (max_feq.0 - cur_feq) > n - i {
                return (i - 1) as i32;
            }
        }

        -1
    }
}