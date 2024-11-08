impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut mark = vec![false; n];

        for el in nums {
            if el <= 0 || el > n as i32 {
                continue;
            }

            mark[el as usize - 1] |= true;
        }

        for i in 0..n {
            if !mark[i] {
                return i as i32 + 1;
            }
        }

        n as i32 + 1
    }
}
