# Client for testing

## How to use

start the server and pass it the path to the config directory used for testing

```sh
cargo run --release -p sniper-server -- --config-path config
```

then in a separate terminal, launch the test client

```sh
cargo run --release -p test-client
```
