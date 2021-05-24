use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use toml::Value;

use crate::cache;
use crate::paths::GeneralPaths;
use crate::utils::{self, FileStatus};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub notes_parent_dir: PathBuf,
}

pub fn build_config(gen_paths: &GeneralPaths) -> anyhow::Result<Config> {
    let config_file_status = utils::create_file(&gen_paths.config_file)?;
    let cache_file_status = utils::create_file(&gen_paths.cache_file)?;

    if let FileStatus::Created = config_file_status {
        let config = Config {
            notes_parent_dir: PathBuf::from(&gen_paths.default_notes_parent_dir),
        };

        write_config(gen_paths, &config)?;
        cache::write_cache(gen_paths, &config)?;

        Ok(config)
    } else if let FileStatus::Exists = cache_file_status {
        let config_file_stat = fs::metadata(&gen_paths.config_file).unwrap();
        let cache_file_stat = fs::metadata(&gen_paths.cache_file).unwrap();
        let config_mod_time = config_file_stat.modified().unwrap().elapsed().unwrap();
        let cache_mod_time = cache_file_stat.modified().unwrap().elapsed().unwrap();

        if config_mod_time < cache_mod_time {
            cache::update_cache(gen_paths)
        } else {
            cache::read_cache(&gen_paths)
        }
    } else {
        cache::update_cache(gen_paths)
    }
}

pub fn parse_config_toml(gen_paths: &GeneralPaths, config_toml: Value) -> Config {
    let notes_parent_dir = if let Some(v) = config_toml.get("notes_parent_dir") {
        PathBuf::from(v.as_str().unwrap())
    } else {
        gen_paths.default_notes_parent_dir.clone()
    };

    Config { notes_parent_dir }
}

pub fn read_config(gen_paths: &GeneralPaths) -> anyhow::Result<Value> {
    let config = fs::read_to_string(&gen_paths.config_file)?.parse::<toml::Value>()?;
    Ok(config)
}

fn write_config(gen_paths: &GeneralPaths, config: &Config) -> anyhow::Result<()> {
    let content = format!(
        r##"# Specify the absolute path of the notes parent directory.
#
# Example usage:
# notes_parent_dir = "/home/john/Documents/"
# 
# This will create the directory "/home/john/Documents/notes"

notes_parent_dir = {:?}

"##,
        &config.notes_parent_dir
    );

    let mut file = File::create(&gen_paths.config_file).unwrap();

    file.write_all(content.as_bytes())?;

    Ok(())
}
