use std::collections::VecDeque;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        
        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        #[inline]
        fn is_valid(x: i32, y: i32, n: i32, m: i32) -> bool {
            x >= 0 && x < n && y >= 0 && y < m
        }

        let mut queue = VecDeque::with_capacity(n * m);
        let mut free = vec![vec![true; m]; n];
        let mut cost = vec![vec![i32::MAX; m]; n];

        cost[0][0] = 0;
        queue.push_front((0, 0));
        
        let target = (n - 1, m - 1);

        while let Some((x, y)) = queue.pop_front() {
            if !free[x][y] {
                continue;
            }

            free[x][y] = false;

            if (x, y) == target {
                return cost[x][y];
            }

            let current_cost = cost[x][y];
            let current_value = grid[x][y];

            for (k, &(dx, dy)) in DIRECTIONS.iter().enumerate() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if !is_valid(nx, ny, n as i32, m as i32) {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if !free[nx][ny] {
                    continue;
                }

                let weight = (k as i32 + 1 != current_value) as i32;
                let new_cost = current_cost + weight;

                if new_cost < cost[nx][ny] {
                    cost[nx][ny] = new_cost;
                    if weight == 0 {
                        queue.push_front((nx, ny));
                    } else {
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        0
    }
}