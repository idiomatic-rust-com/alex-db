# alex-db-client

The command-line application that facilitates communication with the database server.

## Running in the Development Mode

```sh
cd alex-db-client/
cargo run
```

and you will receive the result

```sh
Welcome to AlexDB client
AlexDB〉
```

## Example Requests

Please substitute '63545360-301e-482f-93fc-84e6d11d8aee' with your 'initial API key' in this instance.

### Establishing a connection to the server

Execute the command

```sh
connect http://0.0.0.0:10240 63545360-301e-482f-93fc-84e6d11d8aee
```

and you will receive the result

```sh
Connect http://0.0.0.0:10240
```

### Create

Execute the command

```sh
create test100-key test100-value
```

and you will receive the result

```sh
Value created
Key: test100-key
Value: String("test100-value")
```

### List

Execute the command

```sh
create test101-key true
list
```

and you will receive the result

```sh
Values list
1) Key: test100-key
Value: String("test100-value")
2) Key: test101-key
Value: Boolean(true)
```

### Read

Execute the command

```sh
create test102-key 10
read test102-key
```

and you will receive the result

```sh
Value readed
Key: test102-key
Value: Integer(10)
```

### Update

Execute the command

```sh
create test103-key test103-value-a::test103-value-b
update test103-key test103-value-a::test103-value-c
```

and you will receive the result

```sh
Value updated
Key: test103-key
Value: Array([String("test103-value-a"), String("test103-value-c")])
```

### Delete

Execute the command

```sh
create test104-key test104-value::true::10
delete test104-key
```

and you will receive the result

```sh
Value deleted
```

### Append

Execute the command

```sh
create test105-key test105-value-1::test105-value-2
append test105-key test105-value-appended
```

and you will receive the result

```sh
Value appended
Key: test105-key
Value: Array([String("test105-value-1"), String("test105-value-2"), String("test105-value-appended")])
```

### Prepend

Execute the command

```sh
create test106-key test106-value-1::test106-value-2
prepend test106-key test106-value-prepended
```

and you will receive the result

```sh
Value prepended
Key: test106-key
Value: Array([String("test106-value-prepended"), String("test106-value-1"), String("test106-value-2")])
```

### Increment

Execute the command

```sh
create test107-key 1000
increment test107-key
increment test107-key 10
```

and you will receive the result

```sh
Value incremented
Key: test107-key
Value: Integer(1011)
```

### Decrement

Execute the command

```sh
create test108-key 5000
decrement test108-key
decrement test108-key 10
```

and you will receive the result

```sh
Value decremented
Key: test108-key
Value: Integer(4989)
```

### Pop front

Execute the command

```sh
create test109-key test109-value1::test109-value2::test109-value3::true::false::true::10::11::12
pop_front test109-key
pop_front test109-key 3
```

and you will receive the result

```sh
Value poped front
1) Value: String("test109-value2")
2) Value: String("test109-value3")
3) Value: Boolean(true)
```

### Pop back

Execute the command

```sh
create test110-key test110-value1::test110-value2::test110-value3::true::false::true::10::11::12
pop_back test110-key
pop_back test110-key 3
```

and you will receive the result

```sh
Value poped back
1) Value: Integer(11)
2) Value: Integer(10)
3) Value: Boolean(true)
```
