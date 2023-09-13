# **Sync**

A simple sync solution according to [google/tarpc](https://github.com/google/tarpc/) in PubSub mode.


# **How to use**

Create two server:

```shell
cargo run --bin rust-learn 8887

cargo run --bin rust-learn 8888
```

Run client multiple times: 

```shell
cargo run --bin client
```

This will create a client to `port-8888` server, and register from `8888` to `8887`;

Then, add random string data.

After multiple running, the data in two server always remain equal:

```shell
8888-server:

[DEBUG][2023-09-13 16:56:36]:rust_learn::syncer - Current data: {
    "1",
    "1vKKw",
    "HNEXC",
    "OCb",
    "ZD4",
    "dY5",
    "em1",
    "jRjq3",
    "pPi",
    "qza",
    "uzOmU",
    "ytA",
}

8887-server:
[DEBUG][2023-09-13 16:56:36]:rust_learn::syncer - Current data: {
    "1",
    "1vKKw",
    "HNEXC",
    "OCb",
    "ZD4",
    "dY5",
    "em1",
    "jRjq3",
    "pPi",
    "qza",
    "uzOmU",
    "ytA",
}
```
