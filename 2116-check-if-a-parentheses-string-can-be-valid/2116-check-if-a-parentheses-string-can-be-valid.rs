impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        let n = s.len();
        
        if n % 2 != 0 {
            return false;
        }

        let s_bytes = s.as_bytes();
        let locked_bytes = locked.as_bytes();
        let mut count = 0;

        for i in 0..n {
            count += if locked_bytes[i] == b'1' {
                if s_bytes[i] == b'(' { 1 } else { -1 }
            } else {
                1
            };
            
            if count < 0 {
                return false;
            }
        }
        
        count = 0;

        for i in (0..n).rev() {
            count += if locked_bytes[i] == b'1' {
                if s_bytes[i] == b'(' { -1 } else { 1 }
            } else {
                1
            };
            
            if count < 0 {
                return false;
            }
        }
        
        true
    }
}