use alex_db_lib::{
    config::Config,
    db::Db,
    value_record::{Value, ValuePopFront, ValuePost},
};
use criterion::{criterion_group, criterion_main, Criterion};
use std::{collections::VecDeque, sync::Arc};

fn pop_front(db: Arc<Db>) {
    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_post = ValuePost {
            key,
            ttl: None,
            value: Value::Array(VecDeque::from([
                Value::String("test_value1".to_string()),
                Value::String("test_value2".to_string()),
            ])),
        };

        db.try_create(value_post).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");
        let value_pop_front = ValuePopFront { pop_front: None };

        db.try_pop_front(&key, value_pop_front).unwrap();
    }

    for i in 0..u16::MAX {
        let key = format!("test_key_{i}");

        db.try_delete(&key).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let config = Config::default();
    let db = Arc::new(Db::new(config));

    c.bench_function("pop_front", |b| b.iter(|| pop_front(db.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
