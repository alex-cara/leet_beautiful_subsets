pub mod algorithms {
    use std::cell::*;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::ptr;
    pub fn shivam(nums: &Vec<i128>, k: i128) -> i128 {
        let mut map: BTreeMap<i128, BTreeMap<i128, i128>> = BTreeMap::new();
        let mut total = 1;

        for i in nums {
            map.entry(i % k)
                .and_modify(|inner_map| {
                    inner_map.entry(*i).and_modify(|val| *val += 1).or_insert(1);
                })
                .or_insert({
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

    struct LinkUnsafe<'a> {
        contr: i128,
        len: i128,
        lap: i128,
        start: Option<&'a UnsafeCell<LinkUnsafe<'a>>>,
        end: Option<&'a UnsafeCell<LinkUnsafe<'a>>>,
    }

    impl<'a> LinkUnsafe<'a> {
        fn new() -> LinkUnsafe<'a> {
            LinkUnsafe {
                contr: 1,
                len: 1,
                lap: 1,
                start: None,
                end: None,
            }
        }
    }

    unsafe fn connect_chain<'a>(
        l_chain: Option<&'a UnsafeCell<LinkUnsafe<'a>>>,
        center: Option<&'a UnsafeCell<LinkUnsafe<'a>>>,
        r_chain: Option<&'a UnsafeCell<LinkUnsafe<'a>>>,
    ) {
        unsafe {
            let c = center.unwrap().get();
            (*c).start = center;
            (*c).end = center;
            let calc_repeats = (1 << (*c).len) - 1;
            (*c).len = calc_repeats;
            (*c).contr = calc_repeats;
            (*c).lap = calc_repeats;

            let l_valid = { l_chain.is_some() && (*l_chain.unwrap().get()).start.is_some() };
            let r_valid = { r_chain.is_some() && (*r_chain.unwrap().get()).start.is_some() };
            if l_valid && r_valid {
                let l = l_chain.unwrap().get();
                let r = r_chain.unwrap().get();
                let l_start = (*l).start.unwrap().get();
                let r_end = (*r).end.unwrap().get();

                // Remove any false starts and set ends to eachother
                (*c).start = (*l).start;
                (*r).start = (*l).start;
                (*r_end).start = (*l).start;
                (*l_start).end = (*r).end;

                let l_add_val = ((*l).len - (*l).contr) * (*c).contr; // All combos with left and center (no left solo)
                let r_add_val = ((*r).len - (*r).contr) * (*c).contr; // All combos with right and center (no right solo)

                let l_contr_add = ((*l_start).contr - (*l).lap) * (*c).contr + (*l_start).contr * (*r).len // When center is in
                + ((*l_start).contr - (*l).lap) * ((*r).len - (*r).contr) * (*c).contr;
                let r_contr_add = ((*r_end).contr - (*r).lap) * (*c).contr + (*r_end).contr * (*l).len // When center is in
                + ((*r_end).contr - (*r).lap) * ((*l).len - (*l).contr) * (*c).contr;

                // Calculates all possible lapping from end to end if added togehter
                let ends_lap = (*l_start).contr * (*r_end).contr
                    + (((*l_start).contr - (*l).lap) * ((*r_end).contr - (*r).lap)) * (*c).contr; // Left lap with just center

                (*l_start).len += ((*l).len + l_add_val) * ((*r).len - (*r).contr) // Left and center * (r-r.contr)
                + ((*l).len * (*r).contr) // Left without center and r.contr
                + (*r).len // All of right
                + (*c).contr // All of center
                + l_add_val // left with center
                + r_add_val; // Right with center
                (*r_end).len = (*l_start).len;

                (*r_end).lap = ends_lap;
                (*l_start).lap = ends_lap;
                (*r_end).contr += r_contr_add;
                (*l_start).contr += l_contr_add;
            } else if l_valid || r_valid {
                let (far_end, added_value) = if l_valid {
                    let l = l_chain.unwrap().get();
                    (*c).start = (*l).start;
                    let added_value = ((*l).len - (*l).contr) * (*c).contr;
                    let l_start = (*l).start.unwrap().get();
                    (*l_start).end = center;
                    (l_start, added_value)
                } else {
                    let r = r_chain.unwrap().get();
                    let added_value = ((*r).len - (*r).contr) * (*c).contr;
                    (*c).end = (*r).end;
                    (*r).start = center;
                    let r_end = (*r).end.unwrap().get();
                    (*r_end).start = center;
                    (r_end, added_value)
                };

                (*c).lap = ((*far_end).contr - (*far_end).lap) * (*c).contr;
                (*far_end).lap = (*c).lap;

                (*c).contr += added_value;
                (*far_end).contr += (*c).lap;

                (*far_end).len += (*c).contr;
                (*c).len = (*far_end).len;
                (*far_end).end = center;
            }
        }
    }

    pub fn hashmap_o_n(nums: &Vec<i128>, k: i128) -> i128 {
        let mut map: HashMap<i128, UnsafeCell<LinkUnsafe>> = HashMap::new();
        let mut vec: Vec<i128> = Vec::new();

        for i in nums {
            map.entry(*i)
                .and_modify(|val| unsafe {
                    (*val.get()).len += 1;
                })
                .or_insert({
                    vec.push(*i);
                    UnsafeCell::new(LinkUnsafe::new())
                });
        }

        for i in vec.iter() {
            let curr_piece = map.get(&i).unwrap();
            if unsafe { (*(curr_piece.get())).start.is_none() } {
                unsafe {
                    connect_chain(map.get(&(i - k)), Some(curr_piece), map.get(&(i + k)));
                }
            }
        }

        let mut total = 0;
        for i in vec.iter() {
            let curr_node = map.get(&i).unwrap().get();
            unsafe {
                if (*curr_node).start.is_some()
                    && ptr::eq(curr_node, (*curr_node).start.unwrap().get())
                {
                    (*curr_node).start = None;
                    total = total + (*curr_node).len + total * (*curr_node).len;
                }
            }
        }

        return total;
    }

    #[derive(Default, Clone, Copy, Debug)]
    struct Link {
        contr: i128,
        len: i128,
        lap: i128,
        start: usize,
        end: usize,
    }

    fn connect_branchless(values: &mut [Link], left: usize, center: usize, right: usize) {
        let mut c = values[center];
        let l = values[left];
        let r = values[right];
        let calc_repeats = (1 << c.len) - 1;
        c.len = calc_repeats;
        c.contr = calc_repeats;
        c.lap = calc_repeats;
        let mut l_start = values[l.start];
        let mut r_end = values[r.end];

        let l_add_val = (l.len - l.contr) * c.contr; // All combos with left and center (no left solo) // WIll be 0 if l 0
        let r_add_val = (r.len - r.contr) * c.contr; // All combos with right and center (no right solo) // Will be 0 if r 0

        let l_contr_add = (l_start.contr - l.lap) * c.contr + l_start.contr * r.len // When center is in
        + (l_start.contr - l.lap) * (r.len - r.contr) * c.contr;
        let r_contr_add = (r_end.contr - r.lap) * c.contr + r_end.contr * l.len // When center is in
        + (r_end.contr - r.lap) * (l.len - l.contr) * c.contr;

        let ends_lap = l_start.contr * r_end.contr // Will be 0 if left and right 0
            + (l_start.contr - l.lap) * (r_end.contr - r.lap) * c.contr; // Left lap with just center

        let center_laps = ((l_start.contr - l.lap) + (r_end.contr - r.lap)) * c.contr
            + c.lap * (left == 0 && right == 0) as i128; // We need to calculate center too to ignore branching
        let center_contr = ((l.len - l.contr) + (r.len - r.contr)) * c.contr;

        // This needs to be modified to use center length such that center is the one that updates and replaces the ends
        c.len += (l.len + l_add_val) * (r.len - r.contr) // Left and center * (r-r.contr)
        + (l.len * r.contr) // Left without center and r.contr
        + r.len // All of right
        + l.len // All of left
        + l_add_val // left with center
        + r_add_val; // Right with center

        c.start = (left == 0) as usize * center + l.start;
        c.end = (right == 0) as usize * center + r.end;
        c.contr += center_contr;
        c.lap = center_laps;

        values[right].start = c.start * (right != 0) as usize;

        r_end.start = c.start * (right != 0) as usize;
        l_start.end = c.end * (left != 0) as usize;

        r_end.lap = ends_lap + center_laps * (left == 0 && right != 0) as i128;
        l_start.lap = ends_lap + center_laps * (left != 0 && right == 0) as i128;
        r_end.contr += r_contr_add;
        l_start.contr += l_contr_add;
        r_end.len = c.len * (right != 0) as i128;
        l_start.len = c.len * (left != 0) as i128;
        values[center] = c;
        values[l.start] = l_start;
        values[r.end] = r_end;
    }

    fn connect(values: &mut [Link], left: usize, center: usize, right: usize) {
        let mut c = values[center];
        let l = values[left];
        let r = values[right];
        let calc_repeats = (1 << c.len) - 1;
        c.len = calc_repeats;
        c.contr = calc_repeats;
        c.lap = calc_repeats;
        if l.start == 0 && r.start == 0 {
            c.start = center;
            c.end = center;
            values[center] = c;
            return;
        }
        let mut l_start = values[l.start];
        let mut r_end = values[r.end];

        let l_add_val = (l.len - l.contr) * c.contr; // All combos with left and center (no left solo) // WIll be 0 if l 0
        let r_add_val = (r.len - r.contr) * c.contr; // All combos with right and center (no right solo) // Will be 0 if r 0

        let l_contr_add = (l_start.contr - l.lap) * c.contr + l_start.contr * r.len // When center is in
        + (l_start.contr - l.lap) * (r.len - r.contr) * c.contr;
        let r_contr_add = (r_end.contr - r.lap) * c.contr + r_end.contr * l.len // When center is in
        + (r_end.contr - r.lap) * (l.len - l.contr) * c.contr;

        let ends_lap = l_start.contr * r_end.contr // Will be 0 if left and right 0
            + (l_start.contr - l.lap) * (r_end.contr - r.lap) * c.contr; // Left lap with just center

        let center_laps = ((l_start.contr - l.lap) + (r_end.contr - r.lap)) * c.contr
            + c.lap * (l.start == 0 || r.start == 0) as i128; // We need to calculate center too to ignore branching
        let center_contr = ((l.len - l.contr) + (r.len - r.contr)) * c.contr;

        // This needs to be modified to use center length such that center is the one that updates and replaces the ends
        c.len += (l.len + l_add_val) * (r.len - r.contr) // Left and center * (r-r.contr)
        + (l.len * r.contr) // Left without center and r.contr
        + r.len // All of right
        + l.len // All of left
        + l_add_val // left with center
        + r_add_val; // Right with center

        c.start = (l.start == 0) as usize * center + l.start;
        c.end = (r.end == 0) as usize * center + r.end;
        c.contr += center_contr;
        c.lap = center_laps;

        values[right].start = c.start * (r.start != 0) as usize;

        r_end.start = c.start * (r.start != 0) as usize;
        l_start.end = c.end * (l.end != 0) as usize;

        r_end.lap = ends_lap + center_laps * (l.start == 0 && r.start != 0) as i128;
        l_start.lap = ends_lap + center_laps * (l.start != 0 && r.start == 0) as i128;
        r_end.contr += r_contr_add;
        l_start.contr += l_contr_add;
        r_end.len = c.len * (r.end != 0) as i128;
        l_start.len = c.len * (l.start != 0) as i128;
        values[center] = c;
        values[l.start] = l_start;
        values[r.end] = r_end;
    }

    use rustc_hash::FxHashMap;
    pub fn o_n_new_hash(nums: &Vec<i128>, k: i128) -> i128 {
        let mut map: FxHashMap<i128, usize> = FxHashMap::default();
        let mut vec: Vec<i128> = Vec::with_capacity(128);
        let mut the_chains = [Link::default(); 129];
        let mut index = 0;

        for i in nums {
            map.entry(*i)
                .and_modify(|val| the_chains[*val].len += 1)
                .or_insert_with(|| {
                    vec.push(*i);
                    index += 1;
                    the_chains[index].len = 1;
                    index
                });
        }

        for i in vec.iter() {
            let curr_piece = *map.get(i).unwrap_or_else(|| &0);
            let left = *map.get(&(i - k)).unwrap_or_else(|| &0);
            let right = *map.get(&(i + k)).unwrap_or_else(|| &0);
            let left = left * (the_chains[left].start != 0) as usize;
            let right = right * (the_chains[right].start != 0) as usize;

            connect(&mut the_chains[0..128], left, curr_piece, right);
        }

        let mut total = 0;
        for i in vec.iter() {
            let curr_node_loc = *map.get(&i).unwrap();
            let curr_node = &the_chains[curr_node_loc];
            total = (curr_node.start == curr_node_loc as usize) as i128
                * (total + curr_node.len + total * curr_node.len);
        }

        return total;
    }

    pub fn faster_o_n(nums: &Vec<i128>, k: i128) -> i128 {
        let mut map: HashMap<i128, usize> = HashMap::new();
        let mut vec: Vec<i128> = Vec::with_capacity(128);
        let mut the_chains = [Link::default(); 129];
        let mut index = 0;

        for i in nums {
            map.entry(*i)
                .and_modify(|val| the_chains[*val].len += 1)
                .or_insert_with(|| {
                    vec.push(*i);
                    index += 1;
                    the_chains[index].len = 1;
                    index
                });
        }

        for i in vec.iter() {
            let curr_piece = *map.entry(*i).or_default();
            let left = *map.entry(i - k).or_default();
            let right = *map.entry(i + k).or_default();
            let left = left * (the_chains[left].start != 0) as usize;
            let right = right * (the_chains[right].start != 0) as usize;
            connect(&mut the_chains[0..129], left, curr_piece, right);
        }

        let mut total = 0;
        for i in vec.iter() {
            let curr_node_loc = *map.get(&i).unwrap();
            let curr_node = &the_chains[curr_node_loc];
            total += (curr_node.start == curr_node_loc) as i128
                * (curr_node.len + total * curr_node.len);
        }
        return total;
    }
}
