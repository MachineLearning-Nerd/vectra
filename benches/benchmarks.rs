/// Benchmarks for vectra — measures queries/sec at different scales.
///
/// Run with: cargo bench
/// Results will show how brute-force slows as vector count grows.
/// Later phases will add HNSW benchmarks for comparison.

use criterion::{Criterion, criterion_group, criterion_main, black_box};
use rand::Rng;
use vectra::distance::DistanceMetric;
use vectra::store::VectorStore;

/// Generate a random vector of given dimension
fn random_vector(dim: usize) -> Vec<f32> {
    let mut rng = rand::thread_rng();
    (0..dim).map(|_| rng.gen_range(-1.0..1.0)).collect()
}

/// Build a store with `count` random vectors of given dimension
fn build_store(count: usize, dim: usize) -> VectorStore {
    let mut store = VectorStore::new(dim, DistanceMetric::Cosine);
    for _ in 0..count {
        store.insert(random_vector(dim)).unwrap();
    }
    store
}

fn bench_brute_force_search(c: &mut Criterion) {
    let dim = 128;

    // Benchmark at different scales to see how brute force degrades
    for &count in &[1_000, 10_000, 100_000] {
        let store = build_store(count, dim);
        let query = random_vector(dim);

        c.bench_function(&format!("brute_force_k10_{count}vecs_{dim}d"), |b| {
            b.iter(|| {
                store.search(black_box(&query), 10).unwrap()
            })
        });
    }
}

fn bench_distance_functions(c: &mut Criterion) {
    let dim = 128;
    let a = random_vector(dim);
    let b = random_vector(dim);

    c.bench_function("cosine_distance_128d", |bench| {
        bench.iter(|| vectra::distance::cosine_distance(black_box(&a), black_box(&b)).unwrap())
    });

    c.bench_function("euclidean_distance_128d", |bench| {
        bench.iter(|| vectra::distance::euclidean_distance(black_box(&a), black_box(&b)).unwrap())
    });

    c.bench_function("dot_product_128d", |bench| {
        bench.iter(|| vectra::distance::dot_product(black_box(&a), black_box(&b)).unwrap())
    });
}

criterion_group!(benches, bench_distance_functions, bench_brute_force_search);
criterion_main!(benches);
