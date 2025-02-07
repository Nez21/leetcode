#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    x: usize,
    y: usize,
    t: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.t.cmp(&self.t)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.t.partial_cmp(&self.t)
    }
}

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut free = vec![vec![true; m]; n];
        let mut queue = std::collections::BinaryHeap::<State>::new();
        let mut time = vec![vec![i32::MAX; m]; n];

        time[0][0] = grid[0][0];
        queue.push(State {
            x: 0,
            y: 0,
            t: time[0][0],
        });

        let direction = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        while !queue.is_empty() && free[n - 1][m - 1] {
            let State { x, y, t: _ } = queue.pop().unwrap();

            if !free[x][y] {
                continue;
            }

            free[x][y] = false;

            for (dx, dy) in direction {
                let ru = x as i32 + dx;
                let rv = y as i32 + dy;

                if ru < 0 || rv < 0 || ru >= n as i32 || rv >= m as i32 {
                    continue;
                }

                let u = ru as usize;
                let v = rv as usize;

                if free[u][v] && time[u][v] > time[x][y].max(grid[u][v]) {
                    time[u][v] = time[x][y].max(grid[u][v]);

                    queue.push(State {
                        x: u,
                        y: v,
                        t: time[u][v],
                    });
                }
            }
        }

        time[n - 1][m - 1]
    }
}