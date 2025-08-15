use algorithms::algorithms::{beautiful_subsets_o_n, beautiful_subsets_shivam_aggarwal};
use rand::prelude::*;
use std::iter;
use std::time::Instant;
mod algorithms;

static SEED: u64 = 15239478589816007421;

fn main() {
    let rng = SmallRng::seed_from_u64(SEED); // Used a random number generator to get some seed

    let vec: Vec<i128> = rng.random_iter().take(126 * 500000).collect();
    let custom_vec: Vec<i128> = vec
        .iter()
        .map(|i| ((*i as u128 % 256) + 1) as i128)
        .collect();
    let set = &custom_vec;

    for i in 0..10 {
        let k = 1 << i;
        let now = Instant::now();
        for set in set.chunks_exact(126) {
            beautiful_subsets_o_n(&set.to_vec(), k);
        }

        let elapsed1 = now.elapsed();
        println!("Time elapsed for O(n) is {:.2?} with k: {k}", elapsed1);

        let now = Instant::now();
        for set in set.chunks_exact(126) {
            beautiful_subsets_shivam_aggarwal(&set.to_vec(), k);
        }

        let elapsed2 = now.elapsed();
        println!("Time elapsed shivam is {:.2?} with k: {k}\n", elapsed2);
    }
}
