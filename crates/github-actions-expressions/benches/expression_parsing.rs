use criterion::{black_box, criterion_group, criterion_main, Criterion};
use github_actions_expressions::Expr;

fn parse_simple_expressions(c: &mut Criterion) {
    c.bench_function("parse simple literal", |b| {
        b.iter(|| Expr::parse(black_box("42")))
    });

    c.bench_function("parse simple string", |b| {
        b.iter(|| Expr::parse(black_box("'hello world'")))
    });

    c.bench_function("parse simple boolean", |b| {
        b.iter(|| Expr::parse(black_box("true")))
    });

    c.bench_function("parse context reference", |b| {
        b.iter(|| Expr::parse(black_box("github.actor")))
    });
}

fn parse_complex_expressions(c: &mut Criterion) {
    c.bench_function("parse binary operation", |b| {
        b.iter(|| Expr::parse(black_box("github.event == 'push' && github.ref == 'refs/heads/main'")))
    });

    c.bench_function("parse function call", |b| {
        b.iter(|| Expr::parse(black_box("contains(github.event.head_commit.message, '[skip ci]')")))
    });

    c.bench_function("parse nested context", |b| {
        b.iter(|| Expr::parse(black_box("matrix.os[0].name")))
    });

    c.bench_function("parse complex boolean logic", |b| {
        b.iter(|| Expr::parse(black_box(
            "(github.event_name == 'pull_request' || github.event_name == 'push') && (github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/tags/'))"
        )))
    });
}

fn parse_function_calls(c: &mut Criterion) {
    c.bench_function("parse format function", |b| {
        b.iter(|| Expr::parse(black_box("format('{0} {1}', 'hello', 'world')")))
    });

    c.bench_function("parse contains function", |b| {
        b.iter(|| Expr::parse(black_box("contains(github.event.head_commit.message, 'skip')")))
    });

    c.bench_function("parse startsWith function", |b| {
        b.iter(|| Expr::parse(black_box("startsWith(github.ref, 'refs/heads/')")))
    });

    c.bench_function("parse nested functions", |b| {
        b.iter(|| Expr::parse(black_box(
            "contains(format('{0}/{1}', github.repository, github.sha), github.actor)"
        )))
    });
}

fn evaluate_simple_expressions(c: &mut Criterion) {
    c.bench_function("evaluate literal", |b| {
        b.iter(|| {
            let expr = Expr::parse("42").unwrap();
            expr.consteval()
        })
    });

    c.bench_function("evaluate string", |b| {
        b.iter(|| {
            let expr = Expr::parse("'test string'").unwrap();
            expr.consteval()
        })
    });

    c.bench_function("evaluate boolean operation", |b| {
        b.iter(|| {
            let expr = Expr::parse("true && false").unwrap();
            expr.consteval()
        })
    });
}

fn evaluate_complex_expressions(c: &mut Criterion) {
    c.bench_function("evaluate nested boolean logic", |b| {
        b.iter(|| {
            let expr = Expr::parse("true && (false || true)").unwrap();
            expr.consteval()
        })
    });

    c.bench_function("evaluate comparison", |b| {
        b.iter(|| {
            let expr = Expr::parse("42 > 10 && 10 < 100").unwrap();
            expr.consteval()
        })
    });

    c.bench_function("evaluate format function", |b| {
        b.iter(|| {
            let expr = Expr::parse("format('{0} {1}', 'hello', 'world')").unwrap();
            expr.consteval()
        })
    });

    c.bench_function("evaluate nested functions", |b| {
        b.iter(|| {
            let expr = Expr::parse("contains(format('{0} {1}', 'hello', 'world'), 'world')").unwrap();
            expr.consteval()
        })
    });
}

fn realistic_workflows(c: &mut Criterion) {
    c.bench_function("parse typical if condition", |b| {
        b.iter(|| {
            Expr::parse(black_box(
                "github.event_name == 'push' && github.ref == 'refs/heads/main'"
            ))
        })
    });

    c.bench_function("parse matrix condition", |b| {
        b.iter(|| {
            Expr::parse(black_box(
                "matrix.os == 'ubuntu-latest' && matrix.node == '20'"
            ))
        })
    });

    c.bench_function("parse skip ci check", |b| {
        b.iter(|| {
            Expr::parse(black_box(
                "!contains(github.event.head_commit.message, '[skip ci]') && !contains(github.event.head_commit.message, '[ci skip]')"
            ))
        })
    });

    c.bench_function("parse branch protection", |b| {
        b.iter(|| {
            Expr::parse(black_box(
                "(github.ref == 'refs/heads/main' || startsWith(github.ref, 'refs/heads/release/')) && github.actor != 'dependabot[bot]'"
            ))
        })
    });
}

criterion_group!(
    benches,
    parse_simple_expressions,
    parse_complex_expressions,
    parse_function_calls,
    evaluate_simple_expressions,
    evaluate_complex_expressions,
    realistic_workflows
);
criterion_main!(benches);
