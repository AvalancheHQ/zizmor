use codspeed_criterion_compat::{criterion_group, criterion_main, Criterion};
use std::process::Command;

fn bench_analyze_workflow(c: &mut Criterion) {
    // Build zizmor in release mode if not already built
    let zizmor_bin = std::env::current_dir()
        .unwrap()
        .join("target/release/zizmor");

    c.bench_function("analyze_ci_workflow", |b| {
        b.iter(|| {
            Command::new(&zizmor_bin)
                .args(&["--offline", "--format=plain", "--no-exit-codes", "--no-config"])
                .arg(".github/workflows/ci.yml")
                .output()
                .expect("Failed to execute zizmor");
        })
    });

    c.bench_function("analyze_codegen_workflow", |b| {
        b.iter(|| {
            Command::new(&zizmor_bin)
                .args(&["--offline", "--format=plain", "--no-exit-codes", "--no-config"])
                .arg(".github/workflows/codegen.yml")
                .output()
                .expect("Failed to execute zizmor");
        })
    });

    c.bench_function("analyze_all_workflows", |b| {
        b.iter(|| {
            Command::new(&zizmor_bin)
                .args(&["--offline", "--format=plain", "--no-exit-codes", "--no-config"])
                .arg(".github/workflows")
                .output()
                .expect("Failed to execute zizmor");
        })
    });
}

criterion_group!(benches, bench_analyze_workflow);
criterion_main!(benches);
