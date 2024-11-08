impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut count: Vec<i32> = vec![0; 32];

        for mut c in candidates {
            let mut i = 0;

            while c > 0 {
                if c & 1 == 1 {
                    count[i] += 1;
                }

                c >>= 1;
                i += 1;
            }
        }

        *count.iter().max().unwrap()
    }
}