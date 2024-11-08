impl Solution {
    pub fn divide(a: i32, b: i32) -> i32 {
        if a == i32::MIN && b == -1 {
            return i32::MAX;
        }
                
        let sign = if (a < 0) ^ (b < 0) { -1 } else { 1 };
        let mut ua = a.abs() as u32;
        let ub = b.abs() as u32;
        let mut q: u32 = 0; 
        let mut d = 0;
        
        for i in 0..32 {
            if ((1u32 << i) & ub) != 0 {
                d = i;
            }
        }

        for i in (0..32 - d).rev() { 
            if (ub << i) <= ua {
                ua -= ub << i;
                q |= 1u32 << i;
            }
        }
        
        if sign == -1 { -(q as i32) } else { q as i32 }
    }
}
