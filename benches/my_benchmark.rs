mod algorithms;
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use rand::prelude::*;
use std::iter;

use crate::algorithms::algorithms::{o_n_new_hash, shivam};

fn beautiful_subsets(c: &mut Criterion) {
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
