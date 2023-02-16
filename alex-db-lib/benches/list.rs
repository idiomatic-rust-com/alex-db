use alex_db_lib::{
    config::Config,
    db::{Db, Direction, Sort},
    value_record::{Value, ValuePost},
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::Arc;

fn list(db: Arc<Db>) {
    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_post = ValuePost {
            key,
            ttl: None,
            value: Value::String("test_value".to_string()),
        };

        db.try_create(value_post).unwrap();
    }

    for i in 0..u8::MAX {
        db.list(
            Direction::Asc,
            Some(u8::MAX as usize),
            Some(i as usize),
            Sort::CreatedAt,
        )
        .unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");

        db.try_delete(&key).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let config = Config::default();
    let db = Arc::new(Db::new(config));

    c.bench_function("list", |b| b.iter(|| list(db.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
