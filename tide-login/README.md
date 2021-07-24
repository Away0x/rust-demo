> demo 没有使用 tide 的一些插件(sqlx, log, routes ...)，使用比较底层，比较复杂的写法

# Database
```sql
-- uuid pg 支持
create extension "uuid-ossp";
select uuid_generate_v4();

-- create user table
create table users (
    id uuid primary key,
    username varchar(50) not null,
    hashed_password varchar(255) not null,
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);
create unique index users_username on users(username);
```
