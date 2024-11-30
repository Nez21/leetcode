use std::{
    collections::HashMap,
    ops::{AddAssign, SubAssign},
};

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut edges: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut degs: HashMap<i32, i32> = HashMap::new();

        for pair in pairs {
            let from = pair[0];
            let to = pair[1];

            edges.entry(from).or_insert(Vec::new()).push(to);
            degs.entry(from).or_insert(0).add_assign(1);
            edges.entry(to).or_insert(Vec::new());
            degs.entry(to).or_insert(0).sub_assign(1);
        }

        let mut cur_v = *degs
            .iter()
            .find(|&(_, deg)| *deg == 1)
            .unwrap_or((degs.keys().last().unwrap(), &0))
            .0;
        let mut cur_path: Vec<i32> = vec![cur_v];
        let mut res: Vec<i32> = vec![];

        while cur_path.len() > 0 {
            if let Some(next_v) = edges.get_mut(&cur_v).unwrap().pop() {
                cur_path.push(cur_v);
                cur_v = next_v;
            } else {
                res.push(cur_v);
                cur_v = cur_path.pop().unwrap();
            }
        }

        (1..res.len())
            .rev()
            .map(|i| vec![res[i], res[i - 1]])
            .collect()
    }
}