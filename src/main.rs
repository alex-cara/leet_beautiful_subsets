use std::cell::*;
use std::collections::HashMap;
use std::ptr;

struct Link<'a> {
    start: Option<&'a UnsafeCell<Link<'a>>>,
    end: Option<&'a UnsafeCell<Link<'a>>>,
    contr: i32,
    len: i32,
    lap: i32,
}

impl<'a> Link<'a> {
    fn new() -> Link<'a> {
        Link {
            start: None,
            end: None,
            contr: 1,
            len: 1,
            lap: 1,
        }
    }
}

unsafe fn connect_chain<'a>(
    l_chain: Option<&'a UnsafeCell<Link<'a>>>,
    center: Option<&'a UnsafeCell<Link<'a>>>,
    r_chain: Option<&'a UnsafeCell<Link<'a>>>,
) {
    let c = center.unwrap().get();
    (*c).start = center;
    (*c).end = center;

    let calc_repeats = (u32::MAX >> (32 - (*c).contr)) as i32;
    (*c).contr = calc_repeats;
    (*c).lap = calc_repeats;
    (*c).len = calc_repeats;
    let l_valid = { l_chain.is_some() && (*l_chain.unwrap().get()).start.is_some() };
    let r_valid = { r_chain.is_some() && (*r_chain.unwrap().get()).start.is_some() };
    match (l_valid, r_valid) {
        (true, true) => {
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
            let l_contr_add = (*l_start).contr * (*r).len;
            let r_contr_add = (*r_end).contr * (*l).len;
            // Contribution when including center AND BOTH ENDS // NEEDS TO BE UPDATED INVALID CALCULATION (FINDING SMALLER OVERLAP)
            // Contribution when including INDIVIDUAL ENDS and CENTER
            let l_center_contr_add = ((*l_start).contr - (*l).lap) * (*c).contr
                + ((*l_start).contr - (*l).lap) * ((*r).len - (*r).contr) * (*c).contr;
            let r_center_contr_add = ((*r_end).contr - (*r).lap) * (*c).contr
                + ((*r_end).contr - (*r).lap) * ((*l).len - (*l).contr) * (*c).contr; // This is 0 (and should be)

            // Calculates all possible lapping from end to end (I believe) if added togehter
            let basic_ends_lap = (*l_start).contr * (*r_end).contr;
            let center_ends_lap =
                (((*l_start).contr - (*l).lap) * ((*r_end).contr - (*r).lap)) * (*c).contr; // Left lap with just center

            // This is All left and center * r_chain
            // Split up like this l_chain solo + l_add_val (which is left and center combo) times right without rightmost
            // And then + l_chain with right_most (can't have center) and + r_chain -> len is r_chain original length/contribution total
            (*l_start).len += ((*l).len + l_add_val) * ((*r).len - (*r).contr)
                + ((*l).len * (*r).contr)
                + (*r).len
                + (*c).contr
                + l_add_val
                + r_add_val; // This is a correct compuation for finding the # of of new items
            // All tharust libraries for testing speedt is needed is a more accurate contribution count
            (*r_end).len = (*l_start).len;

            (*r_end).lap = basic_ends_lap + center_ends_lap;
            (*l_start).lap = basic_ends_lap + center_ends_lap;

            (*r_end).contr += r_contr_add + r_center_contr_add;
            (*l_start).contr += l_contr_add + l_center_contr_add;
        }
        (true, _) => {
            let l = l_chain.unwrap().get();
            (*c).start = (*l).start;
            let added_value = ((*l).len - (*l).contr) * (*c).contr;
            let l = (*l).start.unwrap().get();
            (*l).end = center;

            (*c).lap = ((*l).contr - (*l).lap) * (*c).contr;
            (*l).lap = (*c).lap;

            (*c).contr += added_value;
            (*l).contr += (*c).lap;

            (*l).len += (*c).contr;
            (*c).len = (*l).len;
            (*l).end = center;
        }
        (_, true) => {
            let r = r_chain.unwrap().get();
            let added_value = ((*r).len - (*r).contr) * (*c).contr;

            (*c).end = (*r).end;
            (*r).start = center;

            let r = (*r).end.unwrap().get();
            (*r).start = center;

            (*c).lap = ((*r).contr - (*r).lap) * (*c).contr;

            (*r).lap = (*c).lap;

            (*c).contr += added_value;
            (*r).contr += (*c).lap;

            (*r).len += (*c).contr;
            (*c).len = (*r).len;
        }
        (_, _) => {}
    }
}

fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, UnsafeCell<Link>> = HashMap::new();

    for i in nums.iter() {
        if map.contains_key(i) {
            let val = map.get_mut(i).unwrap().get_mut();
            val.len += 1;
            val.contr += 1;
        } else {
            map.insert(*i, UnsafeCell::new(Link::new()));
        }
    }

    for i in nums.iter() {
        let curr_piece = map.get(&i).unwrap();
        if unsafe { (*(curr_piece.get())).start.is_none() } {
            unsafe {
                connect_chain(map.get(&(i - k)), Some(curr_piece), map.get(&(i + k)));
            }
        }
    }

    let mut total = 0;
    for i in nums.iter() {
        let curr_node = map.get(&i).unwrap().get();
        unsafe {
            if (*curr_node).start.is_some() && ptr::eq(curr_node, (*curr_node).start.unwrap().get())
            {
                (*curr_node).start = None;
                total = total + (*curr_node).len + total * (*curr_node).len;
            }
        }
    }

    return total;
}

fn main() {
    let nums = vec![4, 2, 5, 9, 10, 3];
    println!("{}", beautiful_subsets(nums, 1));
}
