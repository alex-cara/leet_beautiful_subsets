mod algorithms;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use rand::prelude::*;
use std::iter;

use crate::algorithms::algorithms::{beautiful_subsets_o_n, beautiful_subsets_shivam_aggarwal};

static SEED: u64 = 15239478589816007421;

fn beautiful_subsets(c: &mut Criterion) {
    // Seed for reproducibility
    let rng = SmallRng::seed_from_u64(SEED); // Used a random number generator to get some seed

    let mut beautiful_subsets = c.benchmark_group("Beautiful subsets ");
    let vec: Vec<i128> = (1..126).collect();
    let tests: Vec<(i128, &str)> = vec![
        (1, "all overlapping"),
        (2, "2 chunks"),
        (8, "8 chunks"),
        (64, "1 partner"),
        (128, "No partners"),
    ];
    let set = &vec;

    for (k, test_name) in tests {
        beautiful_subsets.bench_function(format!(" {test_name} my func"), |b| {
            b.iter(|| {
                for set in set.chunks_exact(126) {
                    beautiful_subsets_o_n(&set.to_vec(), k);
                }
            })
        });
        beautiful_subsets.bench_function(format!(" {test_name} shivam func"), |b| {
            b.iter(|| {
                for set in set.chunks_exact(126) {
                    beautiful_subsets_shivam_aggarwal(&set.to_vec(), k);
                }
            })
        });
    }

    beautiful_subsets.finish();
}

criterion_group!(benches, beautiful_subsets);
criterion_main!(benches);
