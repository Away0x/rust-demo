# api service
```bash
cargo run -p webservice --bin service

# 获取指定老师的课程
# http localhost:3000/courses/1
# 获取老师的指定课程
# http localhost:3000/courses/1/1
# 新增课程
# http post localhost:3000/courses/  teacher_id:=1 name=test
# 更新课程
# http put localhost:3000/courses/1/1  name=new
# 删除课程
# http delete localhost:3000/courses/1/1

# 获取所有老师
# http localhost:3000/teachers/
# 新增老师
# http post localhost:3000/teachers/  name=teacher picture_url=https://baidu.com profile=123
# 更新老师
# http put localhost:3000/teachers/1  name=new_teacher
# 删除老师
# http delete localhost:3000/teachers/1
```

# 传统的 web 应用
```bash
cargo run -p webapp --bin service
```

# WASM
```bash
# 创建项目
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name wasm-client
cd wasm-client
npm init wasm-app www
cd www
```

```bash
# 编译运行
wasm-pack build

cd www
# www 添加依赖
# "dependencies": {
#     "wasm-client": "file:../pkg"
# }
npm install
npm run start
```
