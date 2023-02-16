use alex_db_lib::{
    config::Config,
    db::Db,
    value_record::{Value, ValueAppend, ValuePost},
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::{collections::VecDeque, sync::Arc};

fn append(db: Arc<Db>) {
    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_post = ValuePost {
            key,
            ttl: None,
            value: Value::Array(VecDeque::from([Value::String("test_value".to_string())])),
        };

        db.try_create(value_post).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_append = ValueAppend {
            append: Value::Array(VecDeque::from([Value::String(
                "test_value_appended".to_string(),
            )])),
        };

        db.try_append(&key, value_append).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");

        db.try_delete(&key).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let config = Config::default();
    let db = Arc::new(Db::new(config));

    c.bench_function("append", |b| b.iter(|| append(db.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
