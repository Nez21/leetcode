impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut mark = vec![true; a.len() + 1];
        let mut res = 0;

        a.iter()
            .zip(b.iter())
            .map(|(&el_a, &el_b)| {
                mark[el_a as usize] ^= true;
                mark[el_b as usize] ^= true;

                if el_a == el_b && mark[el_a as usize] {
                    res += 1;

                    return res
                }

                if mark[el_a as usize] {
                    res += 1;
                }

                if mark[el_b as usize] {
                    res += 1;
                }

                res
            })
            .collect()
    }
}