impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut h1 = i32::MIN;
        let mut h2 = i32::MIN;
        let mut r1 = 0;
        let mut r2 = 0;

        for p in prices {
            r2 = r2.max(h2 + p);
            h2 = h2.max(r1 - p);
            r1 = r1.max(h1 + p);
            h1 = h1.max(-p);
        }

        r2
    }
}