impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less = vec![];
        let mut more = vec![];
        let mut equal = 0;

        for &num in nums.iter() {
            if num == pivot {
                equal += 1;
            } else if num < pivot {
                less.push(num);
            } else {
                more.push(num);
            }
        }

        let mut res = Vec::with_capacity(nums.len());
        res.extend_from_slice(&less);
        (0..equal).for_each(|_| res.push(pivot));
        res.extend_from_slice(&more);

        res
    }
}