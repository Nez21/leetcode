impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut free = vec![true; n as usize];
        let mut champions = n;

        for edge in edges {
            if champions == 0 {
                break;
            }

            if free[edge[1] as usize] {
                champions -= 1;
            }

            free[edge[1] as usize] = false;
        }

        let left = free.iter().position(|x| *x);

        if left.is_some() && champions == 1 {
            return left.unwrap() as i32;
        }

        -1
    }
}