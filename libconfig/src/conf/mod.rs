// 该文件引入了 conf mod 下需要共用的东西, 所以其他子文件中只需要 use super::*; 即可使用
// 把该文件移动为 src/conf.rs 等同于 src/conf/mod.rs 的作用

// pub(crate) 表示该 mod 的 pub 行为只在当前 crate 生效 == pub(in crate)
// 同理还有 pub(super), pub(in super), pub(self), pub(in self)
pub(crate) mod basic_config;
pub(crate) mod error;
pub(crate) mod poem_config;

use super::*;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use toml;

pub use self::basic_config::BasicConfig;
pub use self::error::ConfigError;
pub use self::poem_config::PoemConfig;

use crate::environment::{Environment, Environment::*};
use std::collections::HashMap;

const CONFIG_FILENAME: &str = "config/Poem.toml";
pub type Result<T> = ::std::result::Result<T, ConfigError>;
