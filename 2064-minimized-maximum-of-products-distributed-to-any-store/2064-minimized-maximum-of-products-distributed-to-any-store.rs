impl Solution {
    fn is_possible(n: i32, k: i32, quantities: Vec<i32>) -> bool {
        let mut c = 0;

        for el in quantities {
            c += el / k;

            if el % k != 0 {
                c += 1;
            }

            if c > n {
                return false;
            }
        }

        true
    }

    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let max = *quantities.iter().max().unwrap();

        (1..=max)
            .collect::<Vec<_>>()
            .partition_point(|&k| !Self::is_possible(n, k, quantities.clone())) as i32
            + 1
    }
}