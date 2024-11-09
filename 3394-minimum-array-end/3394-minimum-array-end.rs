impl Solution {
    pub fn min_end(mut n: i32, x: i32) -> i64 {
        let mut res = x as u64;
        let mut rem = (n - 1) as u64;
        let mut pos = 1;

        while rem > 0 {
            if res & pos == 0 {
                res |= (rem & 1) * pos;
                rem >>= 1;
            }

            pos <<= 1;
        }

        res as i64
    }
}