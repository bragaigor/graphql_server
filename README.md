# Frist GraphQL Rust Server

### The following works with Rust nightly build

### Steps

Uninstall current Rust version
```
rustup self uninstall
```
Install nightly rust
```
curl https://sh.rustup.rs -sSf | sh
```

Nightly build with cargo
```
cargo +nightly build
```

Run the server
```
cargo +nightly run
```

Accessing GraphQL Playground to make requests.
If using `warp` server access:
```
http://127.0.0.1:3030/graphiql
```

If using `Rocket` server access:
```
http://127.0.0.1:8080
```