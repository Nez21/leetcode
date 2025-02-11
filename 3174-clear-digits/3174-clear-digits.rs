impl Solution {
    pub fn clear_digits(s: String) -> String {
        s.chars()
            .into_iter()
            .fold(Vec::<char>::new(), |mut acc, ch| {
                if ch.is_digit(10) {
                    acc.pop();
                } else {
                    acc.push(ch);
                }

                acc
            })
            .iter()
            .collect::<String>()
    }
}