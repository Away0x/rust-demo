use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Custom {
    pub aaa: String,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Config {
    pub custom: Custom,
}

// 从 Rocket.toml 读取自定义配置
pub fn read_config() -> Result<(), Box<dyn std::error::Error>> {
    // 打印自定义配置
    // 读取 Rocket.toml 里面 [custom] 的配置
    let custom: Custom = rocket::Config::figment().select("custom").extract()?;
    println!("{:#?}", custom);

    // 读取 Rocket.toml 里面 [default.custom] 的配置
    let config: Config = rocket::Config::figment().extract()?;
    println!("{:#?}", config);

    Ok(())
}
