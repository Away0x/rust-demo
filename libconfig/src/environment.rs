use super::*;
use std::str::FromStr;

use self::Environment::*;
use crate::conf::ConfigError;
use serde::Deserialize;

pub const CONFIG_ENV: &str = "POEM_ENV";

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Deserialize)]
pub enum Environment {
    /// The development environment. for Debug mode.
    Development,
    /// The staging environment. for Debug mode.
    Staging,
    /// The production environment. for Release mode.
    Production,
}

impl Environment {
    /// List of all of the possible environments.
    #[allow(dead_code)]
    pub(crate) const ALL: [Environment; 3] = [Development, Staging, Production];

    /// String of all valid environments.
    #[allow(dead_code)]
    pub(crate) const VALID: &'static str = "development, staging, production";

    /// 获取当前的 Environment
    pub fn active() -> Result<Environment, ConfigError> {
        match env::var(CONFIG_ENV) {
            Ok(s) => s.parse().map_err(|_| ConfigError::BadEnv(s)),
            // 没有设置 CONFIG_ENV 时, 走以下分支
            // for Debug mode, "cargo build && target/debug/main"
            #[cfg(debug_assertions)]
            _ => Ok(Development),
            // for Release mode, "cargo build --release && target/release/main"
            #[cfg(not(debug_assertions))]
            _ => Ok(Production),
        }
    }

    #[inline]
    pub fn is_dev(self) -> bool {
        self == Development
    }

    #[inline]
    pub fn is_stage(self) -> bool {
        self == Staging
    }

    #[inline]
    pub fn is_prod(self) -> bool {
        self == Production
    }
}

/// 定义 "".parse<Environment>() 的行为
impl FromStr for Environment {
    type Err = ();

    /// Parsing a production environment
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let env = match s {
            "d" | "dev" | "devel" | "development" => Development,
            "s" | "stage" | "staging" => Staging,
            "p" | "prod" | "production" => Production,
            _ => return Err(()),
        };

        Ok(env)
    }
}

/// 定义 print 时的行为
impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Development => write!(f, "development"),
            Staging => write!(f, "staging"),
            Production => write!(f, "production"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;

    #[test]
    fn parse_env() {
        let env = "p".parse::<Environment>();
        assert_eq!(env.unwrap(), Environment::Production);

        let env = "prod".parse::<Environment>();
        assert_eq!(env.unwrap(), Environment::Production);
    }
}
