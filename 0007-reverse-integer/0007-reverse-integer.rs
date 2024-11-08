impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };
        let num = x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>();

        match num {
            Ok(num) => num.checked_mul(sign).unwrap_or(0),
            Err(_) => 0,
        }
    }
}