impl Solution {
    pub fn recursion(n: usize, l: usize, mark: &mut Vec<bool>, res: &mut Vec<i32>) -> bool {
        if l == 2 * n - 1 {
            return true;
        }

        if res[l] != 0 {
            return Self::recursion(n, l + 1, mark, res);
        }

        for i in (1..=n).rev() {
            if mark[i] || (i > 1 && l + i >= res.len()) {
                continue;
            }

            if i != 1 {
                if res[l + i] != 0 {
                    continue;
                }

                res[l + i] = i as i32;
            }

            mark[i] = true;
            res[l] = i as i32;

            if Self::recursion(n, l, mark, res) {
                return true;
            }

            mark[i] = false;
            res[l] = 0;

            if i != 1 {
                res[l + i] = 0;
            }
        }

        false
    }

    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut mark = vec![false; (n + 1) as usize];
        let mut res = vec![0; (2 * n - 1) as usize];

        if Self::recursion(n as usize, 0, &mut mark, &mut res) {
            res
        } else {
            vec![]
        }
    }
}