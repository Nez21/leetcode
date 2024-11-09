impl Solution {
    pub fn min_end(mut n: i32, x: i32) -> i64 {
        let max = (n as f32).log2().ceil() as usize;
        let mut zeros = Vec::with_capacity(max);

        for i in 0..64 {
            if ((1u64 << i) & (x as u64)) == 0 {
                zeros.push(i);
            }

            if zeros.len() == max {
                break;
            }
        }

        let mut shift = (n - 1) as u64;
        let mut res = x as u64;

        for i in 0..max {
            if ((1u64 << i) & shift) != 0 {
                res += 1u64 << zeros[i];
            }
        }

        res as i64
    }
}