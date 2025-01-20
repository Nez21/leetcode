impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut feq = [0; 26];

        for ch in s.chars().into_iter() {
            let idx = (ch as u8) - ('a' as u8);
            feq[idx as usize] += 1;
        }

        feq.iter().fold(s.len() as i32, |len, &c| len - ((c - 1) / 2) * 2)
    }
}