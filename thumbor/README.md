# 类似 python thumbor 的图片服务器

```bash
cargo build --release  # cargo run 性能很差
RUST_LOG=info target/release/thumbor
# 打开控制台输出的测试链接
# 第二次访问该链接可以测试是否命中缓存
```