pub mod algorithms {
    use std::collections::BTreeMap;
    pub fn shivam(nums: &Vec<i128>, k: i128) -> i128 {
        let mut map: BTreeMap<i128, BTreeMap<i128, i128>> = BTreeMap::new();
        let mut total = 1;

        for i in nums {
            map.entry(i % k)
                .and_modify(|inner_map| {
                    inner_map.entry(*i).and_modify(|val| *val += 1).or_insert(1);
                })
                .or_insert_with(|| {
                    let mut inner = BTreeMap::new();
                    inner.insert(*i, 1);
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

    #[derive(Default, Clone, Copy)]
    struct Link {
        contr: i128,
        len: i128,
        lap: i128,
        start: usize,
        end: usize,
    }

    fn connect(
        values: &mut [Link],
        center: (usize, i128),
        left: (usize, i128),
        right: (usize, i128),
    ) {
        let mut c = values[center.0];
        let l = values[left.0];
        let r = values[right.0];
        let calc_repeats = (1 << center.1) - 1;
        c.len = calc_repeats;
        c.contr = calc_repeats;
        c.lap = calc_repeats;
        if l.start == 0 && r.start == 0 {
            values[center.0] = c;
            return;
        }
        let mut l_start = values[l.start];
        let mut r_end = values[r.end];

        c.len += (l.len + (l.len - l.contr) * c.contr) * (r.len - r.contr) // Left and center * (r-r.contr)
        + (l.len * r.contr) // Left without center and r.contr
        + r.len // All of right
        + l.len // All of left
        + (l.len - l.contr) * c.contr // left with center
        + (r.len - r.contr) * c.contr; // Right with center

        let l_contr_add = (l_start.contr - l.lap) * c.contr + l_start.contr * r.len // When center is in
        + (l_start.contr - l.lap) * (r.len - r.contr) * c.contr;
        let r_contr_add = (r_end.contr - r.lap) * c.contr + r_end.contr * l.len // When center is in
        + (r_end.contr - r.lap) * (l.len - l.contr) * c.contr;

        let ends_lap = l_start.contr * r_end.contr // Will be 0 if left and right 0
            + (l_start.contr - l.lap) * (r_end.contr - r.lap) * c.contr; // Left lap with just center

        let center_laps = ((l_start.contr - l.lap) + (r_end.contr - r.lap)) * c.contr
            + c.lap * (l.start == 0 && r.start == 0) as i128; // We need to calculate center too to ignore branching
        let center_contr = ((l.len - l.contr) + (r.len - r.contr)) * c.contr;

        c.start = (l.start == 0) as usize * center.0 + l.start;
        c.end = (r.end == 0) as usize * center.0 + r.end;
        c.contr += center_contr;
        c.lap = center_laps;

        // Make sure right connects start is no longer saying itself
        values[right.0].start = c.start * (r.start != 0) as usize;

        r_end.start = c.start * (r.start != 0) as usize;
        l_start.end = c.end * (l.end != 0) as usize;

        r_end.lap = ends_lap + center_laps * (l.start == 0 && r.start != 0) as i128;
        l_start.lap = ends_lap + center_laps * (l.start != 0 && r.start == 0) as i128;
        r_end.contr += r_contr_add;
        l_start.contr += l_contr_add;
        r_end.len = c.len * (r.end != 0) as i128;
        l_start.len = c.len * (l.start != 0) as i128;
        values[center.0] = c;
        values[l.start] = l_start;
        values[r.end] = r_end;
    }

    use rustc_hash::FxHashMap;
    pub fn o_n_new_hash(nums: &Vec<i128>, k: i128) -> i128 {
        let mut map: FxHashMap<i128, (usize, i128)> = FxHashMap::default();
        let mut vec: Vec<i128> = Vec::with_capacity(128);
        let mut the_chains = [Link::default(); 129];
        let mut index = 0;

        for i in nums {
            map.entry(*i)
                .and_modify(|val| val.1 += 1)
                .or_insert_with(|| {
                    vec.push(*i);
                    index += 1;
                    (index, 1)
                });
        }

        for i in vec.iter() {
            let curr_piece = *map.get(i).unwrap_or_else(|| &(0, 0));
            let left = *map.get(&(i - k)).unwrap_or_else(|| &(0, 0));
            let right = *map.get(&(i + k)).unwrap_or_else(|| &(0, 0));

            connect(&mut the_chains[0..128], left, curr_piece, right);
        }

        let mut total = 0;
        for i in vec.iter() {
            let curr_node_loc = *map.get(&i).unwrap();
            let curr_node = &the_chains[curr_node_loc.0];
            total = (curr_node.start == curr_node_loc.0 as usize) as i128
                * (total + curr_node.len + total * curr_node.len);
        }

        return total;
    }
}
