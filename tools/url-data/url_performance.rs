//! Dependency-free P6 release-mode URL workload.
//!
//! Run with `cargo run --release --locked --offline --bin kernel_oss_url_performance`.
//! The generated corpus is compiled in; fixture construction is outside every
//! measured closure.

use kernel_oss::values::uri::url::URL;
use std::{hint::black_box, time::Instant};

include!("url_benchmark_corpus.rs");

const SAMPLES: usize = 30;

struct Measurement {
    name: &'static str,
    input_bytes: usize,
    operations: usize,
    samples: Vec<f64>,
}

impl Measurement {
    fn median_ns(&self) -> f64 {
        let mut values = self.samples.clone();
        values.sort_by(f64::total_cmp);
        let middle = values.len() / 2;
        if values.len().is_multiple_of(2) {
            (values[middle - 1] + values[middle]) / 2.0
        } else {
            values[middle]
        }
    }

    fn mean_ns(&self) -> f64 {
        self.samples.iter().sum::<f64>() / self.samples.len() as f64
    }

    fn stddev_ns(&self) -> f64 {
        let mean = self.mean_ns();
        (self
            .samples
            .iter()
            .map(|value| {
                let delta = *value - mean;
                delta * delta
            })
            .sum::<f64>()
            / self.samples.len() as f64)
            .sqrt()
    }

    fn min_ns(&self) -> f64 {
        self.samples
            .iter()
            .copied()
            .min_by(f64::total_cmp)
            .unwrap_or(0.0)
    }

    fn max_ns(&self) -> f64 {
        self.samples
            .iter()
            .copied()
            .max_by(f64::total_cmp)
            .unwrap_or(0.0)
    }

    fn cv_percent(&self) -> f64 {
        self.stddev_ns() / self.mean_ns() * 100.0
    }

    fn throughput_per_second(&self) -> f64 {
        self.operations as f64 / (self.median_ns() / 1_000_000_000.0)
    }
}

fn main() {
    let short = "https://example.test/a";
    let credential = "https://user:pass@example.test/a?x=1&x=2#fragment";
    let percent = "https://example.test/%25%2F%3F%23?a=%25%26%3D#%25";
    let ipv4 = "http://0x7f.1/a";
    let ipv6 = "http://[::ffff:192.0.2.1]/a";
    let idna = "https://b\u{fc}cher.example/a";
    let invalid = "https://[::zzzz]/";
    let base = required_url("https://example.test/a/b?x=1#f");
    let query = required_url("https://example.test/?a=1&a=2&=empty");

    warm_up(short, &base);
    let mut measurements = vec![
        measure("short-ascii-parse", short.len(), 1_000, || {
            URL::try_new(black_box(short)).is_ok()
        }),
        measure(
            "credentials-query-fragment-parse",
            credential.len(),
            1_000,
            || URL::try_new(black_box(credential)).is_ok(),
        ),
        measure("percent-heavy-parse", percent.len(), 1_000, || {
            URL::try_new(black_box(percent)).is_ok()
        }),
        measure("ipv4-parse", ipv4.len(), 1_000, || {
            URL::try_new(black_box(ipv4)).is_ok()
        }),
        measure("ipv6-parse", ipv6.len(), 1_000, || {
            URL::try_new(black_box(ipv6)).is_ok()
        }),
        measure("unicode-idna-parse", idna.len(), 1_000, || {
            URL::try_new(black_box(idna)).is_ok()
        }),
        measure("invalid-rejection", invalid.len(), 1_000, || {
            URL::try_new(black_box(invalid)).is_err()
        }),
        measure("relative-resolution", 8, 1_000, || {
            base.try_resolve(black_box("../next")).is_ok()
        }),
        measure("borrowed-query-pairs", 0, 100_000, || {
            let url = black_box(&query);
            black_box(url.query_pairs().first()).is_some()
        }),
        measure("immutable-mutation", 0, 1_000, || {
            base.mutator()
                .host(Some("example.org"))
                .port(Some(8443))
                .try_mutate()
                .is_ok()
        }),
    ];

    for (size, input) in long_inputs() {
        measurements.push(measure("long-parse", size, 50, || {
            URL::try_new(black_box(input.as_str())).is_ok()
        }));
    }
    measurements.push(measure("wpt-urltestdata-corpus", 0, 1, || {
        WPT_URLTESTDATA_INPUTS.iter().all(|input| {
            let _ = black_box(URL::try_new(black_box(*input)));
            true
        })
    }));

    print_report(&measurements);
    enforce_bounds(&measurements);
}

fn required_url(value: &str) -> URL {
    match URL::try_new(value) {
        Ok(url) => url,
        Err(_) => std::process::exit(2),
    }
}

fn warm_up(short: &str, base: &URL) {
    for _ in 0..1_000 {
        let _ = black_box(URL::try_new(black_box(short)));
        let _ = black_box(base.try_resolve(black_box("next")));
    }
}

fn measure(
    name: &'static str,
    input_bytes: usize,
    inner: usize,
    mut operation: impl FnMut() -> bool,
) -> Measurement {
    let mut samples = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let started = Instant::now();
        for _ in 0..inner {
            if !black_box(operation()) {
                std::process::exit(3);
            }
        }
        samples.push(started.elapsed().as_nanos() as f64 / inner as f64);
    }
    Measurement {
        name,
        input_bytes,
        operations: inner,
        samples,
    }
}

fn long_inputs() -> Vec<(usize, String)> {
    [1_024usize, 4_096, 16_384, 65_536]
        .into_iter()
        .map(|size| {
            (
                size,
                format!(
                    "https://example.test/{}",
                    "a".repeat(size - "https://example.test/".len())
                ),
            )
        })
        .collect()
}

fn print_report(measurements: &[Measurement]) {
    println!("# Kernel OSS URL P6 performance workload");
    println!("samples={SAMPLES}; corpus_cases={WPT_URLTESTDATA_CASE_COUNT}");
    println!(
        "| workload | bytes | median ns | mean ns | stddev ns | cv % | min ns | max ns | ops/s |"
    );
    println!("|---|---:|---:|---:|---:|---:|---:|---:|---:|");
    for measurement in measurements {
        println!(
            "| {} | {} | {:.0} | {:.0} | {:.0} | {:.2} | {:.0} | {:.0} | {:.0} |",
            measurement.name,
            measurement.input_bytes,
            measurement.median_ns(),
            measurement.mean_ns(),
            measurement.stddev_ns(),
            measurement.cv_percent(),
            measurement.min_ns(),
            measurement.max_ns(),
            measurement.throughput_per_second(),
        );
    }
}

fn enforce_bounds(measurements: &[Measurement]) {
    let stable = measurements
        .iter()
        .all(|measurement| measurement.cv_percent() <= 20.0);
    let short = measurements
        .iter()
        .filter(|measurement| {
            matches!(
                measurement.name,
                "short-ascii-parse"
                    | "relative-resolution"
                    | "invalid-rejection"
                    | "immutable-mutation"
            )
        })
        .all(|measurement| measurement.median_ns() < 1_000_000.0);
    let long: Vec<&Measurement> = measurements
        .iter()
        .filter(|measurement| measurement.name == "long-parse")
        .collect();
    let long_medians_bounded = long
        .windows(2)
        .all(|pair| pair[1].median_ns() <= pair[0].median_ns() * 6.0);
    let long_normalized = long
        .get(1)
        .zip(long.get(3))
        .is_some_and(|(four_kib, sixty_four_kib)| {
            sixty_four_kib.median_ns() / sixty_four_kib.input_bytes as f64
                <= four_kib.median_ns() / four_kib.input_bytes as f64 * 2.0
        });
    let long_completion = long
        .iter()
        .all(|measurement| measurement.median_ns() < 250_000_000.0);
    let corpus = measurements
        .iter()
        .find(|measurement| measurement.name == "wpt-urltestdata-corpus")
        .is_some_and(|measurement| measurement.median_ns() < 1_000_000_000.0);
    if !(stable && short && long_medians_bounded && long_normalized && long_completion && corpus) {
        std::process::exit(4);
    }
}
