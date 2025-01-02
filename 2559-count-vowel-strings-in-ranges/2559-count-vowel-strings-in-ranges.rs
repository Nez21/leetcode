impl Solution {
    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }

    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut count = vec![0; words.len()];

        count[0] = if Self::is_vowel(words[0].chars().nth(0).unwrap()) && Self::is_vowel(words[0].chars().last().unwrap()) {
            1
        } else {
            0
        };

        for i in 1..words.len() {
            count[i] = count[i - 1]
                + if Self::is_vowel(words[i].chars().nth(0).unwrap()) && Self::is_vowel(words[i].chars().last().unwrap()) {
                    1
                } else {
                    0
                };
        }

        queries
            .iter()
            .map(|q| {
                let start = q[0] as usize;
                let end = q[1] as usize;

                if start == 0 {
                    count[end]
                } else {
                    count[end] - count[start - 1]
                }
            })
            .collect()
    }
}