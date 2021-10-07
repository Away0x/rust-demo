use libconfig::{Environment, PoemConfig};

fn main() {
    let environment = match Environment::active() {
        Ok(e) => e,
        Err(e) => panic!("{}", e),
    };
    println!(
        "Currnet env: {}, is_dev: {}\n",
        environment,
        environment.is_dev()
    );

    let config = match PoemConfig::read_config() {
        Ok(c) => c,
        Err(err) => panic!("{}", err),
    };
    println!("All config: {:#?}\n", config);
    println!("Current env config: {:#?}\n", config.get(environment));
}
