use super::*;
use serde::Deserialize;
use toml::{map::Map, Value};

#[derive(Debug, Deserialize)]
pub struct BasicConfig {
    pub environment: Environment,
    pub address: String,
    pub port: u16,
    pub database: Option<Database>,
    pub workers: Option<u16>,
    pub(crate) config_file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,
}

impl BasicConfig {
    pub fn new(env: Environment) -> Self {
        Self::default(env)
    }

    pub(crate) fn default(env: Environment) -> Self {
        let default_workers = (num_cpus::get() * 2) as u16;
        let default_config = BasicConfig {
            environment: Development,
            address: "localhost".to_string(),
            port: 8000,
            database: None,
            workers: Some(default_workers),
            config_file_path: None,
            root_path: None,
        };

        match env {
            Development => BasicConfig {
                environment: Development,
                ..default_config
            },
            Staging => BasicConfig {
                environment: Staging,
                ..default_config
            },
            Production => BasicConfig {
                environment: Production,
                ..default_config
            },
        }
    }

    pub(crate) fn set_root<P: AsRef<Path>>(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    /// 设置 file path 的 default
    pub(crate) fn default_from<P>(env: Environment, path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut config = BasicConfig::default(env);

        let config_file_path = path.as_ref().to_path_buf();
        if let Some(parent) = config_file_path.parent() {
            config.set_root(parent);
        } else {
            let msg = "Configuration files must be rooted in a directory.";
            return Err(ConfigError::BadFilePath(config_file_path.clone(), msg));
        }

        config.config_file_path = Some(config_file_path);
        Ok(config)
    }

    pub(crate) fn parse_toml_map(&mut self, kv_pairs: &Map<String, Value>) {
        for (k, v) in kv_pairs {
            match k.as_str() {
                "address" => self.address = v.to_string(),
                "port" => match v.clone().try_into::<u16>() {
                    Ok(v) => self.port = v,
                    Err(_) => continue,
                },
                "workers" => match v.clone().try_into::<u16>() {
                    Ok(v) => self.workers = Some(v),
                    Err(_) => continue,
                },
                "database" => match v.clone().try_into::<Database>() {
                    Ok(v) => self.database = Some(v),
                    Err(_) => continue,
                },
                _ => continue,
            }
        }
    }
}

impl PartialEq for BasicConfig {
    fn eq(&self, other: &BasicConfig) -> bool {
        self.address == other.address && self.port == other.port && self.workers == other.workers
    }
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub(crate) adapter: String,
    pub(crate) db_name: String,
    pub(crate) pool: u32,
}
