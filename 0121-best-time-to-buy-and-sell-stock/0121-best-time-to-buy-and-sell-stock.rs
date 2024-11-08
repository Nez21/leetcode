impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut h = i32::MIN;
        let mut r = 0;

        for p in prices {
            r = r.max(h + p);
            h = h.max(-p);
        }

        r
    }
}