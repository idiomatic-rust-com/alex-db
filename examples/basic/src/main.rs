use alex_db_lib::{
    config::Config,
    db::{Db, Direction, Sort},
    value_record::{
        Value, ValueAppend, ValueDecrement, ValueIncrement, ValuePopBack, ValuePopFront, ValuePost,
        ValuePrepend, ValuePut,
    },
};
use std::collections::VecDeque;

fn main() {
    let config = Config::default();
    let db = Db::new(config);

    println!("Our starting point is an empty database.");
    let value_responses = db
        .list(Direction::Asc, None, None, Sort::CreatedAt)
        .unwrap();
    println!("List value_responses = {value_responses:?}\n");

    let key = "test_key";
    println!("Our next step is to create a record in the database.");
    let value_post = ValuePost {
        key: key.to_string(),
        ttl: None,
        value: Value::String("test_value".to_string()),
    };
    let value_response = db.try_create(value_post).unwrap().unwrap();
    println!("Create value_response = {value_response:?}\n");

    println!("Our next step is to read a record from the database.");
    let value_response = db.try_read(key).unwrap().unwrap();
    println!("Read value_response = {value_response:?}\n");

    println!("Our next step is to update a record in the database.");
    let value_put = ValuePut {
        ttl: None,
        value: Value::Integer(10),
    };
    let value_response = db.try_update(key, value_put).unwrap().unwrap();
    println!("Update value_response = {value_response:?}\n");

    println!("Our next step is to list values from the database.");
    let value_responses = db
        .list(Direction::Asc, None, None, Sort::CreatedAt)
        .unwrap();
    println!("List value_responses = {value_responses:?}\n");

    println!("Our next steps is to increment a value in the database.");
    let value_increment = ValueIncrement { increment: None };
    let value_response = db.try_increment(key, value_increment).unwrap().unwrap();
    println!("Increment value_response = {value_response:?}");
    let value_increment = ValueIncrement {
        increment: Some(10),
    };
    let value_response = db.try_increment(key, value_increment).unwrap().unwrap();
    println!("Increment value_response = {value_response:?}\n");

    println!("Our next steps is to decrement the value in the database.");
    let value_decrement = ValueDecrement { decrement: None };
    let value_response = db.try_decrement(key, value_decrement).unwrap().unwrap();
    println!("Decrement value_response = {value_response:?}");
    let value_decrement = ValueDecrement {
        decrement: Some(10),
    };
    let value_response = db.try_decrement(key, value_decrement).unwrap().unwrap();
    println!("Decrement value_response = {value_response:?}\n");

    println!("Our next step is to update a record in the database.");
    let value_put = ValuePut {
        ttl: None,
        value: Value::Array(VecDeque::from([Value::String("value1".to_string())])),
    };
    let value_response = db.try_update(key, value_put).unwrap().unwrap();
    println!("Update value_response = {value_response:?}\n");

    println!("Our next step is to append a value to the database.");
    let value_append = ValueAppend {
        append: Value::Array(VecDeque::from([Value::String(
            "appended-value1".to_string(),
        )])),
    };
    let value_response = db.try_append(key, value_append).unwrap().unwrap();
    println!("Append value_response = {value_response:?}\n");

    println!("Our next step is to append more values to the database.");
    let value_append = ValueAppend {
        append: Value::Array(VecDeque::from([
            Value::String("appended-value2".to_string()),
            Value::String("appended-value3".to_string()),
        ])),
    };
    let value_response = db.try_append(key, value_append).unwrap().unwrap();
    println!("Append value_response = {value_response:?}\n");

    println!("Our next step is to prepend a value to the database.");
    let value_prepend = ValuePrepend {
        prepend: Value::Array(VecDeque::from([Value::String(
            "prepended-value1".to_string(),
        )])),
    };
    let value_response = db.try_prepend(key, value_prepend).unwrap().unwrap();
    println!("Prepend value_response = {value_response:?}\n");

    println!("Our next step is to prepend more values to the database.");
    let value_prepend = ValuePrepend {
        prepend: Value::Array(VecDeque::from([
            Value::String("prepended-value2".to_string()),
            Value::String("prepended-value3".to_string()),
        ])),
    };
    let value_response = db.try_prepend(key, value_prepend).unwrap().unwrap();
    println!("Prepend value_response = {value_response:?}\n");

    println!("Our next step is to pop back a value from the database.");
    let value_pop_back = ValuePopBack { pop_back: None };
    let value_response = db.try_pop_back(key, value_pop_back).unwrap().unwrap();
    println!("Pop back value_response = {value_response:?}\n");

    println!("Our next step is to read a record from the database.");
    let value_response = db.try_read(key).unwrap().unwrap();
    println!("Read value_response = {value_response:?}\n");

    println!("Our next step is to pop back more values from the database.");
    let value_pop_back = ValuePopBack { pop_back: Some(2) };
    let value_response = db.try_pop_back(key, value_pop_back).unwrap().unwrap();
    println!("Pop back value_response = {value_response:?}\n");

    println!("Our next step is to read a record from the database.");
    let value_response = db.try_read(key).unwrap().unwrap();
    println!("Read value_response = {value_response:?}\n");

    println!("Our next step is to pop front a value from the database.");
    let value_pop_front = ValuePopFront { pop_front: None };
    let value_response = db.try_pop_front(key, value_pop_front).unwrap().unwrap();
    println!("Pop front value_response = {value_response:?}\n");

    println!("Our next step is to read a record from the database.");
    let value_response = db.try_read(key).unwrap().unwrap();
    println!("Read value_response = {value_response:?}\n");

    println!("Our next step is to pop front more values from the database.");
    let value_pop_front = ValuePopFront { pop_front: Some(2) };
    let value_response = db.try_pop_front(key, value_pop_front).unwrap().unwrap();
    println!("Pop front value_response = {value_response:?}\n");

    println!("Our next step is to read a record from the database.");
    let value_response = db.try_read(key).unwrap().unwrap();
    println!("Read value_response = {value_response:?}\n");

    println!("Our next step is to delete a record from the database.");
    let value_response = db.try_delete(key).unwrap().unwrap();
    println!("Read value_response = {value_response:?}\n");

    println!("Our next step is to list values from the database.");
    let value_responses = db
        .list(Direction::Asc, None, None, Sort::CreatedAt)
        .unwrap();
    println!("List value_responses = {value_responses:?}\n");
}
