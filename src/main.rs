use algorithms::algorithms::{faster_o_n, hashmap_o_n, o_n_new_hash, shivam};
use rand::prelude::*;
use std::time::Instant;
mod algorithms;

static SEED: u64 = 15239478589816006428;

fn time_comp<F>(
    subset_function: F,
    the_print: &str,
    input: &Vec<i128>,
    k: i128,
    results: &mut Vec<i128>,
) where
    F: Fn(&Vec<i128>, i128) -> i128,
{
    let now = Instant::now();
    for set in input.chunks_exact(126) {
        results.push(subset_function(&set.to_vec(), k));
    }
    let elapsed = now.elapsed();
    println!("{:<30 } | {:.2?} for k = {k}", the_print, elapsed);
}

fn main() {
    let rng = SmallRng::seed_from_u64(SEED); // Used a random number generator to get some seed
    let vec: Vec<i128> = rng
        .random_iter()
        .take(100000 * 126)
        .map(|i: i128| ((i as u128 % 256) + 1) as i128)
        .collect();
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut v3 = Vec::new();
    let mut v4 = Vec::new();
    {
        let ops: [(&str, fn(&Vec<i128>, i128) -> i128, &mut Vec<i128>); 4] = [
            (
                "Time elaps. for Shivam's",
                |nums, k| shivam(nums, k),
                &mut v1,
            ),
            (
                "Time elaps. for O(n)",
                |nums, k| hashmap_o_n(nums, k),
                &mut v2,
            ),
            (
                "Time elaps. for array O(n)",
                |nums, k| faster_o_n(nums, k),
                &mut v3,
            ),
            (
                "Time elaps. for new Hash O(n)",
                |nums, k| o_n_new_hash(nums, k),
                &mut v4,
            ),
        ];

        for i in 0..10 {
            let k = 1 << i;
            for i in 0..ops.len() {
                time_comp(ops[i].1, ops[i].0, &vec, k, ops[i].2);
            }
            println!();
        }
    }

    let mut results = true;
    for i in 0..v1.len() {
        if v1[i] != v2[i] || v2[i] != v3[i] || v3[i] != v4[i] {
            results = false;
        }
    }
    println!("The results are equal: {results}");
}
