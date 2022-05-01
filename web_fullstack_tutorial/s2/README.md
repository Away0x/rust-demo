Rust 没有内置的 HTTP 支持

# Web Server 结构
1. Server: 监听进来的 TCP 字节流
2. Router: 接收 HTTP 请求, 并决定调用哪个 Handler
3. Handler: 处理 HTTP 请求, 构建 HTTP 响应
4. HTTP Library
    1. 解释字节流, 把它转化为 HTTP 请求
    2. 把 HTTP 响应转化回字节流


```bash
cargo run -p httpserver

// localhost:3000                     => index.html
// localhost:3000/health              => health.html
// localhost:3000/aaa                 => 404.html
// localhost:3000/api/shipping/orders => orders.json
```