//! This crate is a configuration library that loads and merges configuration YAML files
//! using active profiles list passed by user.

use core::str;

use yaml_rust::Yaml;

mod config_provider;
mod local_config;
pub mod profiles;
mod yaml;

pub use config_provider::ConfigProvider;


/// Command which profiles to read and from
pub struct ReadConfigFilesCommand<'a, T: AsRef<str>> {
    pub(crate) profiles: &'a [T],
    pub(crate) config_dir: &'a str,
}

impl<'a, T: AsRef<str>> ReadConfigFilesCommand<'a, T> {
    pub fn new(profiles: &'a [T], config_dir: &'a str) -> ReadConfigFilesCommand<'a, T> {
        ReadConfigFilesCommand {
            profiles,
            config_dir,
        }
    }
}

/// Scans `config_dir` looking for files matching pattern `application-*.yml` where `*` is an active
/// profile specified in the command. Also `application.yml` is loaded as a base configuration that 
/// is later on updated and extended with config from passed profiles.
/// ```rust
/// let yaml = configuro::read_yaml_config_from_files(
///     &ReadConfigFilesCommand::new(&["profile1"], "config")
/// );
pub fn read_yaml_config_from_files<'a, T: AsRef<str>>(
    command: &ReadConfigFilesCommand<'a, T>,
) -> Yaml {
    let lc = local_config::read_profiles_configs(command.profiles, command.config_dir);
    yaml::parse_yaml_config(&lc)
}
