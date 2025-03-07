impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut is_prime = vec![true; right as usize + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        let mut i = 2;

        while i <= right {
            if is_prime[i as usize] {
                let mut j = 2 * i;

                while j <= right {
                    is_prime[j as usize] = false;
                    j += i;
                }
            }

            i += 1;
        }

        let primes: Vec<i32> = (left..=right).filter(|&i| is_prime[i as usize]).collect();

        primes
            .windows(2)
            .min_by(|a, b| (a[1] - a[0]).cmp(&(b[1] - b[0])))
            .unwrap_or(&vec![-1, -1])
            .to_vec()
    }
}