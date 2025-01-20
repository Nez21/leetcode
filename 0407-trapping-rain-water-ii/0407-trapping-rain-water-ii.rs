use std::{collections::BinaryHeap, i32};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    x: usize,
    y: usize,
    height: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.height.cmp(&self.height)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let n = height_map.len();
        let m = height_map[0].len();
        let mut heap = BinaryHeap::<State>::new();

        for i in 0..n {
            for j in 0..m {
                if i == 0 || j == 0 || i == n - 1 || j == m - 1 {
                    heap.push(State {
                        x: i,
                        y: j,
                        height: height_map[i][j],
                    });
                    height_map[i][j] = -1;
                }
            }
        }

        let mut max_height = 0;
        let mut volume = 0;

        const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some(State { x, y, height }) = heap.pop() {
            max_height = max_height.max(height);

            for (d_x, d_y) in DIRECTIONS {
                let new_x = x as i32 + d_x;
                let new_y = y as i32 + d_y;

                if new_x < 0 || new_x >= n as i32 || new_y < 0 || new_y >= m as i32 || height_map[new_x as usize][new_y as usize] < 0 {
                    continue;
                }

                let cur_height = height_map[new_x as usize][new_y as usize];
                height_map[new_x as usize][new_y as usize] = -1;

                if cur_height < max_height {
                    volume += max_height - cur_height;
                }

                heap.push(State {
                    x: new_x as usize,
                    y: new_y as usize,
                    height: cur_height,
                });                
            }
        }

        volume
    }
}