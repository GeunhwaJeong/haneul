// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use criterion::*;

use criterion::Criterion;
use haneul_core::signature_verifier::SignatureVerifierMetrics;
use haneul_types::digests::SenderSignedDataDigest;
use haneul_types::signature_verification::VerifiedDigestCache;
use rand::Rng;

fn verified_digest_cache_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut digests: Vec<_> = (0..(1 << 18))
        .map(|_| SenderSignedDataDigest::new(rng.r#gen()))
        .collect();
    digests.extend_from_slice(&digests.clone());
    rand::seq::SliceRandom::shuffle(digests.as_mut_slice(), &mut rand::rngs::OsRng);

    let cpus = num_cpus::get();
    let chunk_size = digests.len() / cpus;
    let chunks: Vec<Vec<SenderSignedDataDigest>> = digests
        .chunks(chunk_size)
        .take(cpus)
        .map(|c| c.to_vec())
        .collect();
    assert_eq!(chunks.len(), cpus);

    let registry = prometheus::Registry::new();
    let metrics = SignatureVerifierMetrics::new(&registry);
    let cache = VerifiedDigestCache::<SenderSignedDataDigest>::new(
        metrics.signed_data_cache_hits.clone(),
        metrics.signed_data_cache_misses.clone(),
        metrics.signed_data_cache_evictions.clone(),
    );

    let mut group = c.benchmark_group("digest-caching");
    group.throughput(Throughput::Elements(chunk_size as u64));

    group.bench_function("digest cache", |b| {
        b.iter(|| {
            std::thread::scope(|s| {
                let threads = chunks.iter().map(|chunk| {
                    s.spawn(|| {
                        for digest in &**chunk {
                            if cache.is_cached(digest) {
                                continue;
                            } else {
                                cache.cache_digest(*digest);
                            }
                        }
                    })
                });

                for thread in threads {
                    thread.join().unwrap();
                }
            });
        });
    });
    group.finish();
}

criterion_group!(benches, verified_digest_cache_bench);
criterion_main!(benches);
