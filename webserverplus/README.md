```bash
# 多线程版本 server
cargo run -p multithreading
# tokio
cargo run -p asynctokio --bin version1
cargo run -p asynctokio --bin version2
cargo run -p asynctokio --bin version3
# async-std
cargo run -p asyncstd --bin version1
cargo run -p asyncstd --bin version2
cargo run -p asyncstd --bin version3
```