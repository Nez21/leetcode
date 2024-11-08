impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {        
        let mut h = vec![i32::MIN; k as usize];
        let mut r = vec![0; k as usize];

        for p in prices {
            for i in (1..k as usize).rev() {
                r[i] = r[i].max(h[i] + p);
                h[i] = h[i].max(r[i - 1] - p);
            }

            r[0] = r[0].max(h[0] + p);
            h[0] = h[0].max(-p);
        }

        r[k as usize - 1]
    }
}