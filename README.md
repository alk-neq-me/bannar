# Prerequires
```sh
rustup target add x86_64-pc-windows-gnu
sudo apt-get install -y mingw-w64
```

# Build
```sh
cargo build --release --package=bannar  # for linux (ubuntu)
cargo build --release --package=bannar --target=x86_64-pc-windows-gnu  # for windows
cargo build --release --package=bannar-wasm --target=wasm32-unknown-unknown # for ubuntu
```
