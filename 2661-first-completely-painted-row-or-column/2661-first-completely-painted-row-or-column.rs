impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();
        let mut pos = vec![(0, 0); arr.len() + 1];

        for i in 0..n {
            for j in 0..m {
                pos[mat[i][j] as usize] = (i, j);
            }
        }

        let mut row_count = vec![0; n];
        let mut column_count = vec![0; m];

        for i in 0..arr.len() {
            let num = arr[i];
            let (row, column) = pos[num as usize];

            row_count[row] += 1;
            column_count[column] += 1;

            if row_count[row] == m || column_count[column] == n {
                return i as i32;
            }
        }

        0
    }
}