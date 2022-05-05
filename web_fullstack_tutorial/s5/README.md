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
```