use super::*;

#[doc(hidden)]
#[derive(Debug, PartialEq)]
pub struct PoemConfig {
    pub active_env: Environment,
    config: HashMap<Environment, BasicConfig>,
}

impl PoemConfig {
    pub fn read_config() -> super::Result<PoemConfig> {
        let file = PoemConfig::find()?;

        // Try to open the config file for reading.
        let mut handle = File::open(&file).map_err(|_| ConfigError::IoError)?;

        let mut contents = String::new();
        handle
            .read_to_string(&mut contents)
            .map_err(|_| ConfigError::IoError)?;

        PoemConfig::parse(contents, &file)
    }

    /// 获取配置文件的绝对路径
    fn find() -> super::Result<PathBuf> {
        let cwd = env::current_dir().map_err(|_| ConfigError::NotFound)?; // 项目根目录
        let mut current = cwd.as_path();

        loop {
            let manifest = current.join(super::CONFIG_FILENAME);
            //  文件存在
            if fs::metadata(&manifest).is_ok() {
                return Ok(manifest);
            }
            // 文件不存在则去上级目录找, 一直到找到为止
            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }

        Err(ConfigError::NotFound)
    }

    /// 获取对应环境的配置
    pub fn get(&self, env: Environment) -> &BasicConfig {
        match self.config.get(&env) {
            Some(config) => config,
            None => panic!("set(): {} config is missing.", env),
        }
    }

    /// 获取对应环境的配置 mut
    pub fn get_mut(&mut self, env: Environment) -> &mut BasicConfig {
        match self.config.get_mut(&env) {
            Some(config) => config,
            None => panic!("set(): {} config is missing.", env),
        }
    }
    
    /// 获取环境配置默认值 (设置配置路径信息)
    pub(crate) fn active_default_from(filename: Option<&Path>) -> super::Result<PoemConfig> {
        let mut defaults = HashMap::new();
        if let Some(path) = filename {
            defaults.insert(Development, BasicConfig::default_from(Development, &path)?);
            defaults.insert(Staging, BasicConfig::default_from(Staging, &path)?);
            defaults.insert(Production, BasicConfig::default_from(Production, &path)?);
        } else {
            defaults.insert(Development, BasicConfig::default(Development));
            defaults.insert(Staging, BasicConfig::default(Staging));
            defaults.insert(Production, BasicConfig::default(Production));
        }

        let config = PoemConfig {
            active_env: Environment::active()?,
            config: defaults,
        };

        Ok(config)
    }

    /// 获取环境配置默认值
    #[allow(dead_code)]
    pub(crate) fn active() -> super::Result<BasicConfig> {
        Ok(BasicConfig::new(Environment::active()?))
    }

    /// 解析配置
    fn parse<P: AsRef<Path>>(src: String, filename: P) -> super::Result<PoemConfig> {
        let path = filename.as_ref().to_path_buf();
        let table = match src.parse::<toml::Value>() {
            Ok(toml::Value::Table(table)) => table,
            Ok(value) => {
                let err = format!("expected a table, found {}", value.type_str());
                return Err(ConfigError::ParseError(src, path, err, Some((1, 1))));
            }
            Err(e) => {
                return Err(ConfigError::ParseError(
                    src,
                    path,
                    e.to_string(),
                    e.line_col(),
                ))
            }
        };

        // Create a config with the defaults; set the env to the active one.
        let mut config = PoemConfig::active_default_from(Some(filename.as_ref()))?;

        // Parse the values from the TOML file.
        for (entry, value) in table {
            let kv_pairs = match value.as_table() {
                Some(table) => table,
                None => {
                    return Err(ConfigError::BadType(
                        entry,
                        "a table",
                        value.type_str(),
                        Some(path.clone()),
                    ))
                }
            };
            // TODO: 需要把值设置到 config 里面 (这里使用最 low 的遍历设置, 后期优化)
            let default_env_config = match entry.parse::<Environment>() {
                Ok(env) => config.get_mut(env),
                Err(_) => continue,
            };

            for (k, v) in kv_pairs {
                match k.as_str() {
                    "address" => default_env_config.address = v.to_string(),
                    "port" => default_env_config.port = v.clone().try_into::<u16>().unwrap(),
                    _ => continue,
                }
                
            }
        }

        Ok(config)
    }
}
