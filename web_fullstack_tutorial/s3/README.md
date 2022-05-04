```bash
cargo run -p webservice --bin base-service
// http://localhost:3000/health
```
```bash
cargo run -p webservice --bin teacher-service
// http localhost:3000/courses/  teacher_id:=1 course_name="first course"
// http localhost:3000/courses/1
// http localhost:3000/courses/1/1
```

# Actix 的并发
Actix 支持两类并发

1. 异步 I/O: 给定的 OS 原生线程在等待 I/O 时执行其他任务 (例如侦听网络连接)
2. 多线程并行: 默认情况下启动 OS 原生线程的数量与系统逻辑 CPU 数量相同