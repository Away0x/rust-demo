use serde_derive::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    time: String,
    info: String,
}

fn get_all_info(seq: &str) -> String {
    let sys = System::new_all();
    let mut items = vec![];
    let host_name = sys.get_host_name().unwrap();
    let memory = format!("memory: {}/{}", sys.get_total_memory(), sys.get_used_memory());
    let processors_num = format!("process_number: {}", sys.get_processes().len());

    for item in [host_name, memory, processors_num].iter() {
        items.push(item.to_string());
    }

    items.join(seq)
}

fn main() {
    let output = std::env::args().nth(1).unwrap();
    let now = chrono::Utc::now().to_string();

    let out_json = Config {
        time: now,
        info: get_all_info("  &&  "),
    };

    std::fs::write(output, serde_json::to_string_pretty(&out_json).unwrap()).unwrap();
}
