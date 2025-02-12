impl Solution {
    #[inline]
    fn sum_digits(mut num: i32) -> i32 {
        let mut res = 0;

        while num != 0 {
            res += num % 10;
            num /= 10;
        }

        res
    }

    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max = -1;
        let mut map = [-1; 82];

        for num in nums.iter() {
            let sum = Self::sum_digits(*num) as usize;

            if map[sum] >= 0 {
                max = max.max(map[sum] + *num);
                map[sum] = map[sum].max(*num);
            } else {
                map[sum] = *num;
            }
        }

        max
    }
}