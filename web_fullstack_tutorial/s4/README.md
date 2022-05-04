```bash
cargo run -p webservice --bin service
// http localhost:3000/courses/  tutor_id:=1 course_name="first course" course_id:=1
// http localhost:3000/courses/1
// http localhost:3000/courses/1/1
```

# 创建自定义错误处理器
1. 创建一个自定义错误类型
2. 实现 From trait, 用于把其他错误类型转化为该类型
3. 为自定义错误类型实现 ResponseError trait
4. 在 handler 里返回自定义错误类型
5. Actix 会把错误转化为 HTTP 响应