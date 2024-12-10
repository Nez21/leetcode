impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();

        let res = (1..s.len() - 1)
            .collect::<Vec<usize>>()
            .partition_point(|&size| {
                let mut mark = vec![false; 26];

                for i in 0..s.len() - size + 1 {
                    if mark[chars[i] as usize - 'a' as usize] {
                        continue;
                    }

                    if !chars[i..i + size].iter().all(|&item| item == chars[i]) {
                        continue;
                    }

                    let mut count = 1;

                    for sub_b in chars[i + 1..].windows(size) {
                        if chars[i..i + size].iter().zip(sub_b.iter()).all(|(a, b)| a == b) {
                            count += 1;
                        }
                    
                        if count == 3 {
                            return true;
                        }
                    }

                    mark[chars[i] as usize - 'a' as usize] = true;
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
