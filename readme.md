# Ruche
`Ruche` is a simple key/value storage written in rust.
This project is now for personal study and currently in develop.
If you are interested in this project. Welcome to contribute.

## Usage
`Ruche` only support simple `get/set` operation for now.
You can run `ruche-server` and `ruche-cli` for test just as below.
```shell script
wuzhiguos-MacBook-Pro:debug wzg$ RUST_LOG=debug ./ruche-server
[2020-11-29T11:57:20Z INFO  ruche_server] ruche-server 0.1.0
[2020-11-29T11:57:20Z INFO  ruche_server] Listening on 127.0.0.1:8030
[2020-11-29T11:57:20Z DEBUG ruche::server] waiting for connection
[2020-11-29T11:57:22Z DEBUG ruche::server] waiting for connection
[2020-11-29T11:57:22Z DEBUG ruche::server] connection from remote: 127.0.0.1:49797
[2020-11-29T11:57:22Z DEBUG ruche::server] waiting for data
[2020-11-29T11:57:25Z DEBUG ruche::server] Got Request: Set { key: "a", value: "a" }
[2020-11-29T11:57:25Z DEBUG ruche::server] waiting for data
[2020-11-29T11:57:26Z DEBUG ruche::server] Got Request: Get { key: "a" }
[2020-11-29T11:57:26Z DEBUG ruche::server] waiting for data
```

```shell script
wuzhiguos-MacBook-Pro:debug wzg$ RUST_LOG=debug ./ruche-cli
[2020-11-29T09:28:12Z INFO  ruche_cli] ruche-cli 0.1.0
[2020-11-29T09:28:12Z INFO  ruche_cli] ruche-cli connecting 127.0.0.1:8030...
[2020-11-29T09:28:12Z INFO  ruche_cli] ruche-cli connected!
ruche-cli > set a a
OK
ruche-cli > get a
a
```

## Progress
* Use epoll(Tokio) to handle network connection. ✅
* Support transactional.
* Support multiple command and data structure.