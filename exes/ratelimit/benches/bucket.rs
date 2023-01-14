use std::ops::Add;
use std::time::{Duration, SystemTime};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ratelimit::buckets::bucket::Bucket;
use tokio::runtime::Runtime;
use twilight_http_ratelimiting::RatelimitHeaders;

pub fn acquire_ticket(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let bucket = rt.block_on(async move {
        let bucket = Bucket::new();

        let mreset = SystemTime::now()
            .add(Duration::from_secs(3600))
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let headers = [
            (
                "x-ratelimit-bucket",
                "d721dea6054f6322373d361f98e5c38b".as_bytes(),
            ),
            ("x-ratelimit-limit", "100".as_bytes()),
            ("x-ratelimit-remaining", "1".as_bytes()),
            ("x-ratelimit-reset", mreset.as_bytes()),
            ("x-ratelimit-reset-after", "100000.000".as_bytes()),
        ];
        if let RatelimitHeaders::Present(present) =
            RatelimitHeaders::from_pairs(headers.into_iter()).unwrap()
        {
            bucket.update(
                &present,
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            );
        }
        bucket
    });

    let size: usize = 1024;
    c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, _| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(&rt).iter(|| async {
            bucket.ticket().await.unwrap();
        });
    });
}

criterion_group!(benches, acquire_ticket);
criterion_main!(benches);
