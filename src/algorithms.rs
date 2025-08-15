pub mod algorithms {
    use std::cell::*;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::ptr;
    pub fn beautiful_subsets_shivam_aggarwal(nums: &Vec<i128>, k: i128) -> i128 {
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

    struct Link<'a> {
        contr: i128,
        len: i128,
        lap: i128,
        start: Option<&'a UnsafeCell<Link<'a>>>,
        end: Option<&'a UnsafeCell<Link<'a>>>,
    }

    impl<'a> Link<'a> {
        fn new() -> Link<'a> {
            Link {
                contr: 1,
                len: 1,
                lap: 1,
                start: None,
                end: None,
            }
        }
    }

    unsafe fn connect_chain<'a>(
        l_chain: Option<&'a UnsafeCell<Link<'a>>>,
        center: Option<&'a UnsafeCell<Link<'a>>>,
        r_chain: Option<&'a UnsafeCell<Link<'a>>>,
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

    pub fn beautiful_subsets_o_n(nums: &Vec<i128>, k: i128) -> i128 {
        let mut map: HashMap<i128, UnsafeCell<Link>> = HashMap::new();
        let mut vec: Vec<i128> = Vec::new();

        for i in nums {
            map.entry(*i)
                .and_modify(|val| unsafe {
                    (*val.get()).len += 1;
                })
                .or_insert({
                    vec.push(*i);
                    UnsafeCell::new(Link::new())
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
}
