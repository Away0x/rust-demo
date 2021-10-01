1. 创建一个监听 ip:port 的服务
2. 创建一个客户端连接服务
3. 客户端发送信息
4. 服务端接收消息
5. 服务端发送信息
6. 客户端接收消息
5. ...


- 建立一对一 im
- 建立一堆多 im
    - 客户端通过服务端发消息给客户端时, 排除自己
    - 使用 `tokio::select!`

```bash
# start server
cargo run -p server
# start client
cargo run -p client
```