use alex_db_lib::{
    config::Config,
    db::Db,
    value_record::{Value, ValuePost, ValuePut},
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::Arc;

fn update(db: Arc<Db>) {
    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_post = ValuePost {
            key,
            ttl: None,
            value: Value::String("test_value".to_string()),
        };

        db.try_create(value_post).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_put = ValuePut {
            ttl: None,
            value: Value::Integer(i as i64),
        };

        db.try_update(&key, value_put).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");

        db.try_delete(&key).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let config = Config::default();
    let db = Arc::new(Db::new(config));

    c.bench_function("update", |b| b.iter(|| update(db.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
