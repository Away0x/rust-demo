use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String, // 要查找的字符串
    // PathBuf: 像 String, 但用于跨平台工作的文件系统路径
    #[structopt(parse(from_os_str))] // 指明如何解析 PathBuf 类型
    path: std::path::PathBuf, // 要查找的文件
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
