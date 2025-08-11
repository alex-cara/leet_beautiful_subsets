pub mod another {
    use std::collections::BTreeMap;
    fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: BTreeMap<i32, BTreeMap<i32, i32>> = BTreeMap::new();
        let mut total = 1;

        for i in nums {
            map.entry(i % k)
                .and_modify(|inner_map| {
                    inner_map.entry(i).and_modify(|val| *val += 1).or_insert(1);
                })
                .or_insert({
                    let mut inner = BTreeMap::new();
                    inner.insert(i, 1);
                    inner
                });
        }

        for inner_map in map.values() {
            let mut old_num = -k;
            let mut old1 = 1;
            let mut old2 = 1;
            let mut curr = 0;
            for key in inner_map.keys() {
                let value = inner_map.get(key).unwrap();
                println!(
                    "This is outer key value {} and inner value {} and prevNum {}",
                    *key, value, old_num
                );
                let skip = old1;
                let take = ((1 << value) - 1) * (if key - old_num == k { old2 } else { old1 });
                curr = skip + take;
                old2 = old1;
                old1 = curr;
                old_num = *key;
            }
            total *= curr;
        }

        return total - 1;
    }
}
