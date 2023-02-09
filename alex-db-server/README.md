# alex-db-server

Database server application offering REST API for communication.

## Running in the Development Mode

```sh
cd alex-db-server/
cp .env .env.local
# Edit the '.env.local' file now using your preferred editor.
set -a
source .env.local
set +a
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 1.30s
     Running `/home/michal/projects/alex-db/target/debug/alex-db-server`
2023-02-09T14:21:02.746448Z  INFO alex_db_server::config: data_dir = Some("/home/michal/data/")
2023-02-09T14:21:02.746504Z  INFO alex_db_server::config: enable_security_api_keys = true
2023-02-09T14:21:02.746530Z  INFO alex_db_server::config: port = 10240
2023-02-09T14:21:02.746555Z  INFO alex_db_server::config: save_triggered_after_ms = 27000
2023-02-09T14:21:02.746575Z  INFO alex_db_server::config: save_triggered_by_threshold = 4
2023-02-09T14:21:02.746599Z  INFO alex_db_server::config: sleep_time_between_gc_ms = 900
2023-02-09T14:21:02.746622Z  INFO alex_db_server::config: sleep_time_between_saves_ms = 9000
2023-02-09T14:21:02.749852Z  INFO alex_db_server::app: initial api key created: Some(63545360-301e-482f-93fc-84e6d11d8aee)
2023-02-09T14:21:02.760592Z  INFO alex_db_server: listening on 0.0.0.0:10240
```

## Example Requests

Access the API documentation by navigating to http://localhost:10240/swagger-ui/ in your web browser.

Please substitute '63545360-301e-482f-93fc-84e6d11d8aee' with your 'initial API key' in this instance.

### Database stats

Execute the command

```sh
curl --location --request GET 'http://localhost:10240/stats' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw ''
```

and you will receive the result

```sh
{"reads":0,"requests":0,"saved_at":"2023-02-09T14:26:00.865051741Z","saved_writes":0,"writes":0}
```

### Create

Execute the command

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test1-key",
    "value": "test1-value"
}'
```

and you will receive the result

```sh
{"key":"test1-key","value":"test1-value"}
```

### List

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test2-key",
    "value": true
}'

curl --location --request GET 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw ''
```

and you will receive the result

```sh
[{"key":"test1-key","value":"test1-value"},{"key":"test2-key","value":true}]
```

There are additional parameters that you can use for sorting and paginating.

- direction:
  - asc
  - desc
- sort:
  - created_at
  - delete_at
  - key
  - updated_at
- page - page number
- limit - limit of items per page

Execute the command

```sh
curl --location --request GET 'http://localhost:10240/values?sort=created_at&direction=asc&page=1&limit=1' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw ''
```

and you will receive the result

```sh
[{"key":"test1-key","value":"test1-value"}]
```

### Read

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test3-key",
    "value": 10
}'

curl --location --request GET 'http://localhost:10240/values/test3-key' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw ''
```

and you will receive the result

```sh
{"key":"test3-key","value":10}
```

### Update

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test4-key",
    "value": ["test4-value"]
}'

curl --location --request PUT 'http://localhost:10240/values/test4-key' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test4-key",
    "value": "test4-value-updated"
}'
```

and you will receive the result

```sh
{"key":"test4-key","value":"test4-value-updated"}
```

### Delete

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test5-key",
    "value": ["test5-value", true, 10]
}'

curl --location --request DELETE 'http://localhost:10240/values/test5-key' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw ''
```

### Append

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test6-key",
    "value": ["test6-value"]
}'

curl --location --request PUT 'http://localhost:10240/values/test6-key/append' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "append": ["test6-value-appended"]
}'
```

and you will receive the result

```sh
{"key":"test6-key","value":["test6-value","test6-value-appended"]}
```

### Prepend

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test7-key",
    "value": ["test7-value"]
}'

curl --location --request PUT 'http://localhost:10240/values/test7-key/prepend' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "prepend": ["test7-value-prepended"]
}'
```

and you will receive the result

```sh
{"key":"test7-key","value":["test7-value-prepended","test7-value"]}
```

### Increment

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test8-key",
    "value": 1000
}'

curl --location --request PUT 'http://localhost:10240/values/test8-key/increment' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{}'

curl --location --request PUT 'http://localhost:10240/values/test8-key/increment' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "increment": 10
}'
```

and you will receive the result

```sh
{"key":"test8-key","value":1011}
```

### Decrement

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test9-key",
    "value": 5000
}'

curl --location --request PUT 'http://localhost:10240/values/test9-key/decrement' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{}'

curl --location --request PUT 'http://localhost:10240/values/test9-key/decrement' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "decrement": 10
}'
```

and you will receive the result

```sh
{"key":"test9-key","value":4989}
```

### Pop front

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test10-key",
    "value": ["test10-value1", "test10-value2", "test10-value3", true, false, true, 10, 11, 12]
}'

curl --location --request PUT 'http://localhost:10240/values/test10-key/pop-front' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{}'

curl --location --request PUT 'http://localhost:10240/values/test10-key/pop-front' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "pop_front": 3
}'
```

and you will receive the result

```sh
["test10-value2","test10-value3",true]
```

### Pop back

Execute the commands

```sh
curl --location --request POST 'http://localhost:10240/values' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "key": "test11-key",
    "value": ["test11-value1", "test11-value2", "test11-value3", true, false, true, 10, 11, 12, ["test11-a-value1", "test11-a-value2", "test11-a-value3"], ["test11-b-value1", "test11-b-value2", "test11-b-value3"], ["test11-c-value1", "test11-c-value2", "test11-c-value3"]]
}'

curl --location --request PUT 'http://localhost:10240/values/test11-key/pop-back' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{}'

curl --location --request PUT 'http://localhost:10240/values/test11-key/pop-back' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "pop_back": 3
}'
```

and you will receive the result

```sh
[["test11-b-value1","test11-b-value2","test11-b-value3"],["test11-a-value1","test11-a-value2","test11-a-value3"],12]
```
