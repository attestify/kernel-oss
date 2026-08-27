# AKNL URL P6 performance qualification

The dependency-free release workload is
`tools/url-data/url_performance.rs`, run with:

```text
cargo run --release --locked --offline --bin kernel_oss_url_performance
```

It warms up and collects 30 samples for short ASCII, credentials/query/fragment,
percent-heavy, IPv4, IPv6, Unicode/IDNA, invalid rejection, relative resolution,
borrowed query-pair access, immutable mutation, the 891-row pinned WPT corpus,
and 1 KiB through 64 KiB scaling inputs. Inputs are constructed outside measured
closures; the WPT corpus is generated deterministically and compiled in. The
workload uses only `std::time::Instant` and `std::hint::black_box`.

The executable enforces the P6 bounds: coefficient of variation at most 20%,
sub-millisecond short parse/resolve/invalid/mutation medians, sub-250 ms long
input medians, sub-second corpus median, bounded 4x scaling, and 64 KiB
normalized cost no more than twice the 4 KiB value. Borrowed `query_pairs()`
access is measured as a reference access and its no-allocation/no-clone contract
is enforced by the public implementation signature and permanent API tests.

Native release qualification on the available host passed. Median times were
14.5 µs for short ASCII parsing, 21.9 µs for relative resolution, 384 ns for
invalid rejection, 22.3 µs for immutable mutation, and 6.20 ms for the complete
891-case WPT corpus. The 1/4/16/64 KiB parse medians were 21.6/39.5/114/406 µs;
each scaling ratio and normalized 64 KiB cost met the P6 bound. All 30-sample
coefficients of variation were below 6%, and the 64 KiB median was below
250 ms. Performance remains a broad regression qualification rather than a
real-time service guarantee.
