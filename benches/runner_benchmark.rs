use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deno_runner::Builder;
use std::collections::HashMap;

fn simple_execution(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("simple execution", |b| {
        b.to_async(&rt).iter(|| async {
            let runner = Builder::new().build();
            let result = runner.run::<_, String, i32>("1 + 1", None).await.unwrap();
            black_box(result);
        });
    });
}

fn with_variables(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("with variables", |b| {
        b.to_async(&rt).iter(|| async {
            let runner = Builder::new().build();
            let vars = HashMap::from([("a", 10), ("b", 20)]);
            let result = runner.run("a + b", Some(vars)).await.unwrap();
            black_box(result);
        });
    });
}

fn complex_computation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("complex computation", |b| {
        b.to_async(&rt).iter(|| async {
            let code = r#"
                let sum = 0;
                for (let i = 0; i < 1000; i++) {
                    sum += i;
                }
                sum
            "#;
            let runner = Builder::new().build();
            let result = runner.run::<_, String, i32>(code, None).await.unwrap();
            black_box(result);
        });
    });
}

fn json_processing(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("json processing", |b| {
        b.to_async(&rt).iter(|| async {
            let code = r#"
                const data = { users: [
                    { name: "Alice", age: 30 },
                    { name: "Bob", age: 25 }
                ]};
                data.users.map(u => u.name).join(', ')
            "#;
            let runner = Builder::new().build();
            let result = runner.run::<_, String, i32>(code, None).await.unwrap();
            black_box(result);
        });
    });
}

criterion_group!(
    benches,
    simple_execution,
    with_variables,
    complex_computation,
    json_processing
);
criterion_main!(benches);
