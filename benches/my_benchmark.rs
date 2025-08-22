mod algorithms;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use rand::prelude::*;
use std::iter;

use crate::algorithms::algorithms::{o_n_new_hash, shivam};
static SEED: u64 = 15239478589816006428;

fn beautiful_subsets(c: &mut Criterion) {
    let rng = SmallRng::seed_from_u64(SEED); // Used a random number generator to get some seed
    let mut beautiful_subsets = c.benchmark_group("Beautiful subsets ");
    let vec: Vec<i128> = rng
        .random_iter()
        .take(1000 * 126)
        .map(|i: i128| ((i as u128 % 256) + 1) as i128)
        .collect();
    let tests: Vec<(i128, &str)> = vec![
        (1, "k is 1"),
        (2, "k is 2"),
        (8, "k is 8"),
        (32, "k is 32"),
        (128, "k is 128"),
        (300, "no chains"),
    ];
    let set = &vec;

    for (k, test_name) in tests {
        beautiful_subsets.bench_function(format!(" {test_name} my func"), |b| {
            b.iter(|| {
                for set in set.chunks_exact(126) {
                    o_n_new_hash(&set.to_vec(), k);
                }
            })
        });

        beautiful_subsets.bench_function(format!(" {test_name} Shivam Aggarwal func"), |b| {
            b.iter(|| {
                for set in set.chunks_exact(126) {
                    shivam(&set.to_vec(), k);
                }
            })
        });
    }

    beautiful_subsets.finish();
}

criterion_group!(benches, beautiful_subsets);
criterion_main!(benches);
