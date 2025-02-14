#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    x: usize,
    y: usize,
    t: i32,
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.t.cmp(&self.t)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.t.partial_cmp(&self.t)
    }
}

impl Solution {
    pub fn min_time_to_reach(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        let mut time = vec![vec![i32::MAX; m]; n];
        let mut visited = vec![vec![false; m]; n];
        let mut queue = std::collections::BinaryHeap::<Move>::new();

        let directions: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        time[0][0] = 0;
        queue.push(Move {
            x: 0,
            y: 0,
            t: 0,
        });

        while let Some(Move { x, y, t }) = queue.pop() {
            if x == n - 1 && y == m - 1 {
                return time[n - 1][m - 1];
            }

            if visited[x][y] {
                continue;
            }

            visited[x][y] = true;

            for (dx, dy) in directions.iter() {
                let u = match x.checked_add_signed(*dx) {
                    Some(val) => val,
                    _ => continue,
                };
                let v = match y.checked_add_signed(*dy) {
                    Some(val) => val,
                    _ => continue,
                };

                if u < n && v < m {
                    let est = t.max(grid[u][v]) + 1;

                    if !visited[u][v] && time[u][v] > est {
                        time[u][v] = est;
                        queue.push(Move {
                            x: u,
                            y: v,
                            t: time[u][v],
                        });
                    }
                }
            }
        }

        -1
    }
}