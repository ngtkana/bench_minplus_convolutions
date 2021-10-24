use {
    bench_minplus_convolutions::*,
    criterion::{
        criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
        Throughput,
    },
    rand::{prelude::StdRng, SeedableRng},
};

fn minplus_convolutions(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Min-plus convolutions");
    group.plot_config(plot_config);
    let mut rng = StdRng::seed_from_u64(42);

    for &size in &[10, 100, 1000, 10000, 1000000] {
        group.throughput(Throughput::Elements(size as u64));
        let input = (
            generate_convex_vec(&mut rng, size),
            generate_convex_vec(&mut rng, size),
        );

        if size <= 1000 {
            group.bench_with_input(BenchmarkId::new("brute", size), &input, |bench, (a, b)| {
                bench.iter(|| drop(brute_minplus_convolution(a, b)))
            });
        }
        group.bench_with_input(
            BenchmarkId::new("monotone_minima", size),
            &input,
            |bench, (a, b)| bench.iter(|| drop(monotone_minima_minplus_convolution(a, b))),
        );
        group.bench_with_input(BenchmarkId::new("smawk", size), &input, |bench, (a, b)| {
            bench.iter(|| drop(smawk_minplus_convolution(a, b)))
        });
    }
    group.finish();
}

fn minplus_convolutions_all_zeros(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Min-plus convolutions all zeros");
    group.plot_config(plot_config);

    for &size in &[10, 100, 1000, 10000, 1000000] {
        group.throughput(Throughput::Elements(size as u64));
        let input = (vec![0; size], vec![0; size]);

        if size <= 1000 {
            group.bench_with_input(BenchmarkId::new("brute", size), &input, |bench, (a, b)| {
                bench.iter(|| drop(brute_minplus_convolution(a, b)))
            });
        }
        group.bench_with_input(
            BenchmarkId::new("monotone_minima", size),
            &input,
            |bench, (a, b)| bench.iter(|| drop(monotone_minima_minplus_convolution(a, b))),
        );
        group.bench_with_input(BenchmarkId::new("smawk", size), &input, |bench, (a, b)| {
            bench.iter(|| drop(smawk_minplus_convolution(a, b)))
        });
    }
    group.finish();
}

fn minplus_convolutions_small(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
    let mut group = c.benchmark_group("Min-plus convolutions small");
    group.plot_config(plot_config);
    let mut rng = StdRng::seed_from_u64(42);

    for &size in &[40, 60, 80, 100] {
        group.throughput(Throughput::Elements(size as u64));
        let input = (
            generate_convex_vec(&mut rng, size),
            generate_convex_vec(&mut rng, size),
        );

        group.bench_with_input(BenchmarkId::new("brute", size), &input, |bench, (a, b)| {
            bench.iter(|| drop(brute_minplus_convolution(a, b)))
        });
        group.bench_with_input(
            BenchmarkId::new("monotone_minima", size),
            &input,
            |bench, (a, b)| bench.iter(|| drop(monotone_minima_minplus_convolution(a, b))),
        );
        group.bench_with_input(BenchmarkId::new("smawk", size), &input, |bench, (a, b)| {
            bench.iter(|| drop(smawk_minplus_convolution(a, b)))
        });
    }
    group.finish();
}

fn minplus_convolutions_very_small(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Linear);
    let mut group = c.benchmark_group("Min-plus convolutions very small");
    group.plot_config(plot_config);
    let mut rng = StdRng::seed_from_u64(42);

    for &size in &[1, 2, 3, 4] {
        group.throughput(Throughput::Elements(size as u64));
        let input = (
            generate_convex_vec(&mut rng, size),
            generate_convex_vec(&mut rng, size),
        );

        group.bench_with_input(BenchmarkId::new("brute", size), &input, |bench, (a, b)| {
            bench.iter(|| drop(brute_minplus_convolution(a, b)))
        });
        group.bench_with_input(
            BenchmarkId::new("monotone_minima", size),
            &input,
            |bench, (a, b)| bench.iter(|| drop(monotone_minima_minplus_convolution(a, b))),
        );
        group.bench_with_input(BenchmarkId::new("smawk", size), &input, |bench, (a, b)| {
            bench.iter(|| drop(smawk_minplus_convolution(a, b)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    minplus_convolutions,
    minplus_convolutions_all_zeros,
    minplus_convolutions_small,
    minplus_convolutions_very_small
);
criterion_main!(benches);
