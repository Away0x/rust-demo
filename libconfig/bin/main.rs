use libconfig::PoemConfig;

fn main() {
    match PoemConfig::read_config() {
        Ok(conf) => println!("{:#?}", conf),
        Err(err) => println!("{}", err),
    }
}
