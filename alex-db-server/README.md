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
RUST_LOG=alex-db-server=trace,info,debug,tokio=trace,runtime=trace cargo run
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
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee'
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
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee'
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
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee'
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
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee'
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
    "value": ["test4-value"],
    "ttl": 120
}'

curl --location --request PUT 'http://localhost:10240/values/test4-key' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
--data-raw '{
    "value": "test4-value-updated",
    "ttl": 200
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
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee'
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

## Performance

Presently, the server displays satisfactory performance on its API endpoints.

Execute the command

```sh
curl --location --request GET 'http://localhost:10240/values/test3-key' \
--header 'Content-Type: application/json' \
--header 'X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee' \
-w ' Total: %{time_total}s\n'
```

and you will receive the result

```sh
{"key":"test3-key","value":10} Total: 0.001278s
```

and

```sh
{"key":"test3-key","value":10} Total: 0.000617s
```

when you run project in release mode

```sh
cargo run --release
```

The response time for the API endpoint has been found to be slightly above 1ms in the development mode and below 1ms in the production mode, based on our evaluation on an Ubuntu 22.04 system hosted on VirtualBox.

Execute the command

```sh
ab -c 16 -n 100000 -H "X-Auth-Token: 63545360-301e-482f-93fc-84e6d11d8aee" http://localhost:10240/values/test3-key
```

and you will receive the result

```sh
This is ApacheBench, Version 2.3 <$Revision: 1879490 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 10000 requests
Completed 20000 requests
Completed 30000 requests
Completed 40000 requests
Completed 50000 requests
Completed 60000 requests
Completed 70000 requests
Completed 80000 requests
Completed 90000 requests
Completed 100000 requests
Finished 100000 requests


Server Software:
Server Hostname:        localhost
Server Port:            10240

Document Path:          /values/test3-key
Document Length:        30 bytes

Concurrency Level:      16
Time taken for tests:   12.560 seconds
Complete requests:      100000
Failed requests:        0
Total transferred:      13800000 bytes
HTML transferred:       3000000 bytes
Requests per second:    7961.69 [#/sec] (mean)
Time per request:       2.010 [ms] (mean)
Time per request:       0.126 [ms] (mean, across all concurrent requests)
Transfer rate:          1072.96 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0      14
Processing:     0    2   0.6      2      17
Waiting:        0    2   0.6      2      17
Total:          0    2   0.6      2      17

Percentage of the requests served within a certain time (ms)
  50%      2
  66%      2
  75%      2
  80%      2
  90%      2
  95%      2
  98%      2
  99%      3
 100%     17 (longest request)
```

This indicates that the server is capable of handling over 7,900 requests per second on the testing machine.

The performance of the internal database has not been measured yet. The numbers above reflect the performance when accessing the database through the HTTP protocol.

Please note that there is an overhead associated with the HTTP server. The server must process the HTTP request, query the database, serialize the data, and send the response.
