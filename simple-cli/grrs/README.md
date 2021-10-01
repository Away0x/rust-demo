```bash
# run 后加 -- 可以传递参数
cargo run -- some-pattern some-file
cargo run -- main src/main.rs
```

## 解析参数
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");
    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    // 使用 structop 简化参数解析
    let args = Cli::from_args();
    Ok(())
}
```

## 错误处理
### unwrap
```rust
fn main() {
    let args = Cli::from_args();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content }
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };
}
```
可用 unwrap 简化为 

```rust
fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string("test.txt").unwrap();
}
```

### ? 操作符
- `Box<dyn std::error::Error>`: 它是一个可以包含任意实现了标准 Error trait 的类型 的 Box 智能指针。这意味着基本上所有 error 都能放进该 Box ，所以可以将 `?` 用于返回 Result 的所有常用函数

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content }
        Err(error) => { return Err(error.into()); }
    };

    Ok(())
}
```
可使用 `?` 操作符简化为

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path)?;

    Ok(())
}
```

### 自定义错误
```rust
#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;

    println!("file content: {}", content);
    Ok(())

    // Error: CustomError("Error reading `test.txt`: No such file or directory (os error 2)")
}
```

#### 使用 anyhow
- 它的 Context trait 可以用来添加描述。除此之外，它还保留了原始 error ，因此我们会得到指向错误根源的 error

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;

    println!("file content: {}", content);
    Ok(())
}

// Error: could not read file `test.txt`
// Caused by:
//     No such file or directory (os error 2)
```

***

## Print
### println!
> [文档](https://doc.rust-lang.org/1.39.0/std/fmt/index.html)

- "用户友好的" 打印是使用 Display trait 完成的
- debug 输出（人类可读但面向开发者）使用 Debug trait

```rust
// 打印简单类型
let x = 42;
println!("My lucky number is {}.", x);
// 打印实现了 Debug trait 的类型

let xs = vec![1, 2, 3];
println!("The list is: {:?}", xs);
```

### 打印 errors
- 通过 stderr 完成错误打印，使得用户和其他工具更容易将其输出传输到文件或更多工具

在大多数操作系统，程序可以写入两个输出流，stdout 和 stderr。stdout 是用于程序的实际输出，而 stderr 允许将错误和其他消息与 stdout 分开保存。
这样，在向用户显示错误时，输出就可以存储到文件或传输到另一个程序。

在 Rust 中这是用 println! 和 eprintln! 实现的，前者打印到 stdout，后者打印到 stderr

```rust
println!("This is information");
eprintln!("This is an error! :(");
```

### 提升打印性能
```rust
use std::io::{self, Write};

let stdout = io::stdout(); // 获取全局 stdout 实体
let mut handle = io::BufWriter::new(stdout); // 可选: 将句柄包装进缓冲区中
writeln!(handle, "foo: {}", 42); // 如果你关心此处的 error，添加 `?`

// 可调用 handle.flush() 立即打印
```

### 命令行显示进度条
- [indicatif](https://crates.io/crates/indicatif)

```rust
fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
```

***

## Log
- [log crate](https://crates.io/crates/log)
- [env_logger 日志适配器](https://crates.io/crates/env_logger)

```rust
use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}

// 运行时可这样
// env RUST_LOG=output_log=info cargo run --bin output-log
```

***

## Test
- 单元测试写文件里，集成测试写在 tests 文件夹下

```rust
// 可运行 cargo test 进行测试，其会发现项目内所有有 #[test] 的用例
#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
```

***
