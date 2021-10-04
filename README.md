# Actix-Web demo

A demo for using [actix-web](https://github.com/actix/actix-web) to create a RESTful API.

## How to use

start project:

```bash
cargo run
```

test project:

```bash
curl http://localhost:8080/user/0
{"id":"0","name":"foo"}

curl http://localhost:8080/user/1
{"id":"1","name":"bar"}

curl http://localhost:8080/user/2
{"id":"2","name":"baz"}

curl http://localhost:8080/user/3
null

curl http://localhost:8080/user/123/abc
{"id":"123","name":"abc"}
```
