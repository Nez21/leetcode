impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();

        let res = (1..s.len() - 1)
            .collect::<Vec<usize>>()
            .partition_point(|&size| {
                let mut count = vec![0; 26];

                for i in 0..s.len() - size + 1 {
                    if !chars[i..i + size].iter().all(|&item| item == chars[i]) {
                        continue;
                    }

                    count[chars[i] as usize - 'a' as usize] += 1;

                    if count[chars[i] as usize - 'a' as usize] == 3 {
                        return true
                    }
                }

                false
            }) as i32;

        if res == 0 {
            -1
        } else {
            res
        }
    }
}
