# Prerequires
```sh
rustup target add x86_64-pc-windows-gnu
sudo apt-get install -y mingw-w64
```

# Build
```sh
cargo build --release --target=x86_64-pc-windows-gnu  # for windows
cargo build --release # for ubuntu
```
