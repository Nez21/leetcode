impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 { 
        let mut max = 0;
        let includes: std::collections::HashSet<i32> = arr.iter().cloned().collect();
        let mut map = std::collections::HashMap::<(i32, i32), i32>::new();

        for i in 2..arr.len() {
            for j in 1..i {
                let num_k = arr[i] - arr[j];

                if num_k < arr[j] && includes.contains(&num_k) {
                    let len_zj = *map.entry((num_k, arr[j])).or_insert(2);
                    max = max.max(len_zj + 1);
                    map.insert((arr[j], arr[i]), len_zj + 1);
                }
            }
        }

        max
    }
}