impl Solution {
    pub fn min_operations(nums: Vec<i32>, rk: i32) -> i32 {
        let k = rk as i64;
        let mut queue = 
            nums.iter().map(|num| std::cmp::Reverse(*num as i64)).collect::<std::collections::BinaryHeap<_>>();
        let mut steps = 0;

        loop {
            let first = queue.pop().unwrap().0;

            if first >= k {
                break;
            }

            let second = queue.pop().unwrap().0;

            queue.push(std::cmp::Reverse(first.min(second) * 2 + first.max(second)));
            steps += 1;
        }

        steps
    }
}