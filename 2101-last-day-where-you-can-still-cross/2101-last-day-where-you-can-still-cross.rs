impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let n = row as usize;
        let m = col as usize;

        (0..=cells.len())
            .collect::<Vec<usize>>()
            .partition_point(|&day| {
                let mut grid =
                    cells
                        .iter()
                        .take(day)
                        .fold(vec![vec![true; m]; n], |mut visited, val| {
                            visited[val[0] as usize - 1][val[1] as usize - 1] = false;
                            visited
                        });

                let mut queue = std::collections::VecDeque::<(usize, usize)>::new();
                let directions: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

                grid[0]
                    .iter_mut()
                    .enumerate()
                    .filter(|(_, visited)| **visited)
                    .for_each(|(j, visited)| {
                        *visited = false;
                        queue.push_front((0, j));
                    });

                while !queue.is_empty() {
                    let (x, y) = queue.pop_back().unwrap();

                    for (dx, dy) in directions.iter() {
                        let u = match x.checked_add_signed(*dx) {
                            Some(val) => val,
                            None => continue,
                        };

                        let v = match y.checked_add_signed(*dy) {
                            Some(val) => val,
                            None => continue,
                        };

                        if u < n && v < m && grid[u][v] {
                            if u == n - 1 {
                                return true;
                            }

                            grid[u][v] = false;
                            queue.push_front((u, v));
                        }
                    }
                }

                false
            }) as i32
            - 1
    }
}