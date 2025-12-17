use codspeed_criterion_compat::{Criterion, black_box, criterion_group, criterion_main};
use std::path::Path;

// We'll benchmark the workflow parsing and basic validation
fn bench_parse_workflow(c: &mut Criterion) {
    // Use one of the actual workflow files from the repo
    let workflow_path = Path::new(".github/workflows/ci.yml");

    c.bench_function("parse_ci_workflow", |b| {
        b.iter(|| {
            // Read and parse the workflow file
            let content = std::fs::read_to_string(black_box(workflow_path))
                .expect("Failed to read workflow file");
            let _parsed: serde_yaml::Value =
                serde_yaml::from_str(&content).expect("Failed to parse workflow");
        });
    });
}

fn bench_multiple_workflows(c: &mut Criterion) {
    let workflows = [
        ".github/workflows/ci.yml",
        ".github/workflows/release-binaries.yml",
        ".github/workflows/site.yml",
    ];

    c.bench_function("parse_multiple_workflows", |b| {
        b.iter(|| {
            for workflow_path in &workflows {
                if Path::new(workflow_path).exists() {
                    let content = std::fs::read_to_string(black_box(workflow_path))
                        .expect("Failed to read workflow file");
                    let _parsed: serde_yaml::Value =
                        serde_yaml::from_str(&content).expect("Failed to parse workflow");
                }
            }
        });
    });
}

criterion_group!(benches, bench_parse_workflow, bench_multiple_workflows);
criterion_main!(benches);
