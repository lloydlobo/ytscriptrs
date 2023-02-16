use criterion::{criterion_group, criterion_main};

#[cfg(test)]
mod bench_par {
    use criterion::Criterion;
    use ytscriptrs::{find_subtitle_filename_no_shrink, find_subtitle_filename_shrink};

    static OUTPUT: &str = r#"[youtube] Extracting URL: https://youtu.be/HHjgK6p4nrw
 [youtube] HHjgK6p4nrw: Downloading we
 bpage
 [youtube] HHjgK6p4nrw: Downloading android player API JSON
 [info] HHjgK6p4nrw: Downloading subtitl
 es: en-ehkg1hFWq8A
 [info] HHjgK6p4nrw: Downloading 1 format(s): 247+251
 Deleting existing file Guy Kawas
 aki： The Top 10 Mistakes of Entrepreneurs [HHjgK6p4nrw].en-ehkg1hFWq8A.ttml
 [info] Writing video subtitl
 es to: Guy Kawasaki： The Top 10 Mistakes of Entrepreneurs [HHjgK6p4nrw].en-ehkg1hFWq8A.ttml
 [download] D
 estination: Guy Kawasaki： The Top 10 Mistakes of Entrepreneurs [HHjgK6p4nrw].en-ehkg1hFWq8A.ttml

 [down
 load]    1.00KiB at  Unknown B/s (00:00:00)
 [download]    3.00KiB at    1.42MiB/s (00:00:00)
 [download]
    7.00KiB at    1.74MiB/s (00:00:00)
 [download]   15.00KiB at    1.37MiB/s (00:00:00)
 [download]   31.0
 0KiB at    1.47MiB/s (00:00:00)
 [download]   63.00KiB at    1.11MiB/s (00:00:00)
 [download]  127.00KiB a
 t  997.55KiB/s (00:00:00)
 [download]  171.51KiB at  743.02KiB/s (00:00:00)
 [download] 100% of  171.51KiB
  in 00:00:00 at 379.97KiB/s
 "#;

    pub fn target_par(c: &mut Criterion) -> criterion::BenchmarkId {
        c.bench_function("find_subtitle_filename_no_shrink", |b| {
            b.iter(|| find_subtitle_filename_no_shrink(OUTPUT))
        });
        c.bench_function("find_subtitle_filename_shrink", |b| {
            b.iter(|| find_subtitle_filename_shrink(OUTPUT))
        });

        criterion::BenchmarkId::new("find_subtitle_filename", "output")
    }
}
// criterion_group!(benches, target_par);
// criterion_main!(benches);

criterion_group!(benches, bench_par::target_par);
criterion_main!(benches);

/*
2023-02-16 19:36
     Running benches/bench_par.rs (target/release/deps/bench_par-3f1d358437808f08)
Benchmarking find_subtitle_filename_no_shrink: Collecting 100 samples in estimated 5.0569 s (217k iteratio
find_subtitle_filename_no_shrink
                        time:   [22.917 µs 23.862 µs 24.846 µs]
                        change: [-2.6766% +0.3850% +3.3018%] (p = 0.80 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  9 (9.00%) high mild

find_subtitle_filename_shrink
                        time:   [26.150 µs 26.990 µs 27.713 µs]
                        change: [+10.189% +13.688% +17.463%] (p = 0.00 < 0.05)
                        Performance has regressed.
2023-02-16 19:35
    Running benches/bench_par.rs (target/release/deps/bench_par-3f1d358437808f08)
Benchmarking find_subtitle_filename_no_shrink: Collecting 100 samples in estimated 5.1143 s (217k iteratio
find_subtitle_filename_no_shrink
                        time:   [23.257 µs 23.944 µs 24.646 µs]
                        change: [-6.3089% -3.2405% -0.2034%] (p = 0.04 < 0.05)
                        Change within noise threshold.

find_subtitle_filename_shrink
                        time:   [23.331 µs 24.423 µs 25.455 µs]
                        change: [-14.951% -12.601% -10.347%] (p = 0.00 < 0.05)
                        Performance has improved.
2023-02-16 19:34
    Running benches/bench_par.rs (target/release/deps/bench_par-3f1d358437808f08)
Benchmarking find_subtitle_filename_no_shrink: Collecting 100 samples in estimated 5.0456 s (207k iteratio
find_subtitle_filename_no_shrink
                        time:   [22.881 µs 23.618 µs 24.327 µs]

find_subtitle_filename_shrink
                        time:   [26.558 µs 27.024 µs 27.475 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

*/
