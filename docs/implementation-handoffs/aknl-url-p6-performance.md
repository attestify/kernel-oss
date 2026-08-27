# AKNL URL P6 performance qualification

The dependency-free release workload is the non-product benchmark target at
`tools/url-data/url_performance.rs`, run with:

```text
cargo bench --bench kernel_oss_url_performance --locked --offline
```

It uses only `std::time::Instant` and `std::hint::black_box`. Every workload is
warmed before its thirty measured samples: 1,000 warm-up operations for regular
URL operations, 50 for each long-input size, 100,000 for borrowed query-pair
access, and three complete passes for the compiled 891-row WPT corpus. Fixture
construction is outside each measured closure.

`iterations` is the number of work units within each sample. Throughput for all
rows is `1e9 / median_nanoseconds`: it is explicitly **URL operations per
second** for individual-operation rows and **complete corpus iterations per
second** for the WPT row. It is never multiplied by the sample iteration count.

The workload enforces coefficient of variation at most 20%, sub-millisecond
short parse/resolve/invalid/mutation medians, sub-250 ms long-input medians,
sub-second corpus medians, bounded long-input scaling, and a 64 KiB normalized
cost no more than twice the 4 KiB value. Borrowed `query_pairs()` access is a
reference-access workload; its allocation-free contract is enforced by the
public API and permanent tests.

Native release qualification passed with the following baseline:

| workload | size | iterations | median ns | mean ns | stddev ns | CV % | min ns | max ns | useful throughput | status |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| short-ascii-parse | 22 B | 1,000 | 14,790 | 14,849 | 354 | 2.38 | 14,271 | 15,650 | 67,614 URL operations/s | PASS |
| credentials-query-fragment-parse | 49 B | 1,000 | 15,326 | 15,338 | 266 | 1.73 | 14,943 | 15,985 | 65,249 URL operations/s | PASS |
| percent-heavy-parse | 49 B | 1,000 | 15,231 | 15,461 | 1,215 | 7.86 | 14,881 | 21,952 | 65,656 URL operations/s | PASS |
| ipv4-parse | 15 B | 1,000 | 17,778 | 17,892 | 507 | 2.83 | 17,511 | 20,486 | 56,250 URL operations/s | PASS |
| ipv6-parse | 27 B | 1,000 | 1,348 | 1,343 | 32 | 2.35 | 1,286 | 1,411 | 741,955 URL operations/s | PASS |
| unicode-idna-parse | 25 B | 1,000 | 32,267 | 32,563 | 1,380 | 4.24 | 31,226 | 38,478 | 30,991 URL operations/s | PASS |
| invalid-rejection | 17 B | 1,000 | 383 | 387 | 16 | 4.05 | 371 | 431 | 2,613,525 URL operations/s | PASS |
| relative-resolution | 8 B | 1,000 | 21,830 | 21,802 | 264 | 1.21 | 21,212 | 22,247 | 45,809 URL operations/s | PASS |
| borrowed-query-pairs | 0 B | 100,000 | 1 | 1 | 0 | 5.26 | 1 | 1 | 1,497,196,500 URL operations/s | PASS |
| immutable-mutation | 0 B | 1,000 | 22,332 | 22,501 | 1,133 | 5.03 | 21,881 | 28,432 | 44,779 URL operations/s | PASS |
| long-parse | 1 KiB | 50 | 21,668 | 21,810 | 1,204 | 5.52 | 20,487 | 25,473 | 46,151 URL operations/s | PASS |
| long-parse | 4 KiB | 50 | 39,818 | 40,322 | 1,535 | 3.81 | 38,386 | 44,180 | 25,114 URL operations/s | PASS |
| long-parse | 16 KiB | 50 | 112,650 | 113,096 | 2,553 | 2.26 | 109,565 | 119,880 | 8,877 URL operations/s | PASS |
| long-parse | 64 KiB | 50 | 405,066 | 405,977 | 5,212 | 1.28 | 395,056 | 416,458 | 2,469 URL operations/s | PASS |
| wpt-urltestdata-corpus | 891 rows | 1 | 6,324,625 | 6,313,693 | 101,133 | 1.60 | 6,160,791 | 6,541,000 | 158 complete corpus iterations/s | PASS |

All rows were stable, long-input scaling and normalized-cost checks passed, and
the result remains a broad regression qualification rather than a real-time
service guarantee.
