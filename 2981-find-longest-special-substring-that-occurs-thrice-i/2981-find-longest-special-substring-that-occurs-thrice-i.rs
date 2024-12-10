impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let n = s.len();

        let res = (1..chars.len() - 1)
            .collect::<Vec<usize>>()
            .partition_point(|&size| {
                let mut mark = vec![false; 26];

                for i in 0..n - size + 1 {
                    if mark[chars[i] as usize - 'a' as usize] {
                        continue;
                    }

                    let mut same = true;

                    for j in i..(i + size) {
                        if chars[i] != chars[j] {
                            same = false;
                            break;
                        }
                    }

                    if !same {
                        continue;
                    }

                    let mut count = 1;

                    for sub in chars[i + 1..].windows(size) {
                        if chars[i..i + size]
                            .iter()
                            .zip(sub.iter())
                            .all(|(a, b)| a == b) {
                            count += 1;
                        }

                        if count == 3 {
                            break;
                        }
                    }

                    if count == 3 {
                        return true;
                    }

                    mark[chars[i] as usize - 'a' as usize] = true;
                }

                false
            }) as i32;

        if res == 0 { -1 } else { res }
    }
}