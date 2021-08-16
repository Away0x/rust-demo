use std::io::{BufRead, Read, Write, BufReader};
fn main() -> std::io::Result<()> {
    // 打开文件
    let mut f = std::fs::OpenOptions::new().append(true).open("test.txt")?;

    writeln!(&mut f, "hello world")?;
    Ok(())
}
