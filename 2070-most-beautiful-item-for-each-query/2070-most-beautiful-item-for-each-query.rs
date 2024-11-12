impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; queries.len()];
        let mut item_idx = 0;
        let mut max = 0;

        let mut sorted_queries = queries
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i))
            .collect::<Vec<(i32, usize)>>();

        sorted_queries.sort_by_key(|x| x.0);
        items.sort_by_key(|x| x[0]);

        for (query, query_idx) in sorted_queries {
            while item_idx < items.len() && items[item_idx][0] <= query {
                max = max.max(items[item_idx][1]);
                item_idx += 1;
            }

            result[query_idx] = max;
        }

        result
    }
}