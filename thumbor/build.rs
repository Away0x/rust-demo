// 该文件会在编译 cargo 项目时, 做而外的编译处理

fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
