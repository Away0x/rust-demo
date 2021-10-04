> rust 编写的类似 python httpie 的 cli 小工具

```bash
cargo build --quiet
target/debug/httpie post https://httpbin.org/post greeting=hola name=Tyr
```