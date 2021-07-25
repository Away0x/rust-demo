```bash
# 例子是使用 application/octet-stream 方式上传下载文件的, 不支持 multipart/form-data
# 文件存储在临时目录中, 重启项目即消失

# 测试上传
curl -T ./a.jpg localhost:8080/api/file/
# 测试下载 
curl localhost:8080/api/file/a.jpg --output a2.jpg
```