//! The configuration is a mix of multiple sources, cli arguments have
//! priority over environment variables which have priority over file
//! settings.
//!

use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use figment::{
    Figment,
    providers::{Env, Format, Serialized, Yaml},
};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Parser, Debug)]
struct ConfigCli {
    /// Path for the configuration file. Can also be set using the
    /// `{{ crate_name | shouty_snake_case }}_CONFIG_FILE` env var. If not provided, `config.yml` will be
    /// attempted.
    #[arg(long)]
    pub config_file: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
struct ConfigEnv {
    #[serde(rename = "{{ crate_name }}_config_file")]
    pub config_file: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct ConfigFile {
    // TODO: add file config fields
}

#[derive(Debug)]
pub struct Config {
    // TODO: add config fields
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let args = ConfigCli::parse();
        let env = envy::from_env::<ConfigEnv>()?;

        let config_path = args
            .config_file
            .or(env.config_file)
            .unwrap_or(PathBuf::from("config.yml"));

        info!(path = ?config_path, "Loading configuration file from");

        // Try to load the configuration
        // from the config file or the environment
        //
        // Later sources have priority over earlier ones.
        // Note: We exclude whitelist from automatic env parsing and handle it manually
        let mut file = Figment::from(Serialized::defaults(ConfigFile::default()))
            .merge(Yaml::file(config_path))
            .merge(Env::prefixed("{{ crate_name | shouty_snake_case }}_").split("__"))
            .extract::<ConfigFile>()
            .context("failed to parse config file")?;

        Ok(Config {
            // TODO: add default values
        })
    }
}

