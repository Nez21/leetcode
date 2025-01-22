use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = is_water.len();
        let m = is_water[0].len();
        let mut height = vec![vec![0; m]; n];
        let mut queue = VecDeque::<(usize, usize)>::new();

        for i in 0..n {
            for j in 0..m {
                if is_water[i][j] == 1 {
                    queue.push_back((i, j));
                }
            }
        }

        if queue.is_empty() {
            queue.push_back((0, 0));
            height[0][0] = 1;
        }

        const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in DIRECTIONS {
                let tu = x as isize + dx;
                let tv = y as isize + dy;

                if tu < 0 || tv < 0 || tu >= n as isize || tv >= m as isize {
                    continue;
                }

                let u = tu as usize;
                let v = tv as usize;

                if is_water[u][v] != 0 {
                    continue;
                }

                is_water[u][v] = -1;
                height[u][v] = height[x][y] + 1;
                queue.push_back((u, v));
            }
        }

        height
    }
}