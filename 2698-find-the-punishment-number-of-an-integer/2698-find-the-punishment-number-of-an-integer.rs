impl Solution {
    const ST: [i32; 29] = [
        1, 9, 10, 36, 45, 55, 82, 91, 99, 100, 235, 297, 369, 370, 379, 414, 657, 675, 703, 756,
        792, 909, 918, 945, 964, 990, 991, 999, 1000,
    ];

    // pub fn is_satisfy(n: i32, mut i: i32, mut c: i32, l: i32, s: i32) -> bool {
    //     if s > n {
    //         return false;
    //     }

    //     if i == 0 {
    //         return n == s + c;
    //     }

    //     let m = i % 10;
    //     i /= 10;
    //     c += m * l;

    //     Self::is_satisfy(n, i, c, l * 10, s) || Self::is_satisfy(n, i, 0, 1, s + c)
    // }


    pub fn punishment_number(n: i32) -> i32 {
        Self::ST.iter().fold(0, |acc, &num| {
            if num <= n {
                acc + num * num
            } else {
                acc
            }
        })

        // (1..=n).into_iter().fold(0, |acc, num| {
        //     let num2 = num * num;

        //     if Self::is_satisfy(num, num2, 0, 1, 0) {
        //         acc + num2
        //     } else {
        //         acc
        //     }
        // })
    }
}