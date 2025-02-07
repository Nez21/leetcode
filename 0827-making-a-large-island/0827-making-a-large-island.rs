impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut lands = vec![0; 2];
        let mut max = 0;

        let directions: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    let mut volume = 1;
                    let mut queue = std::collections::VecDeque::<(usize, usize)>::new();

                    queue.push_front((i, j));
                    grid[i][j] = lands.len() as i32;

                    while let Some((x, y)) = queue.pop_back() {
                        for (dx, dy) in directions.iter() {
                            let u = match x.checked_add_signed(*dx) {
                                Some(val) => val,
                                None => continue,
                            };
                            let v = match y.checked_add_signed(*dy) {
                                Some(val) => val,
                                None => continue,
                            };

                            if u < n && v < m && grid[u][v] == 1 {
                                queue.push_front((u, v));
                                grid[u][v] = lands.len() as i32;
                                volume += 1;
                            }
                        }
                    }

                    lands.push(volume);
                    max = max.max(volume);
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    let mut four = [0usize; 4];

                    for (idx, &(dx, dy)) in directions.iter().enumerate() {
                        let u = match i.checked_add_signed(dx) {
                            Some(val) => val,
                            None => continue,
                        };
                        let v = match j.checked_add_signed(dy) {
                            Some(val) => val,
                            None => continue,
                        };

                        if u < m && v < m {
                            let cur_set = grid[u][v] as usize;

                            if !four.contains(&cur_set) {
                                four[idx] = cur_set;
                            }
                        }
                    }

                    max = max.max(1 + lands[four[0]] + lands[four[1]] + lands[four[2]] + lands[four[3]])
                }
            }
        }

        max
    }
}