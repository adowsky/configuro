use core::str;

use yaml_rust::Yaml;

mod config_provider;
mod local_config;
mod yaml;

pub mod profiles;

pub use config_provider::ConfigProvider;

pub struct ReadYamlConfigFilesCommand<'a, T: AsRef<str>> {
    pub(crate) profiles: &'a [T],
    pub(crate) config_dir: &'a str,
}

impl<'a, T: AsRef<str>> ReadYamlConfigFilesCommand<'a, T> {
    pub fn new(profiles: &'a [T], config_dir: &'a str) -> ReadYamlConfigFilesCommand<'a, T> {
        ReadYamlConfigFilesCommand {
            profiles,
            config_dir,
        }
    }
}

pub fn read_yaml_config_from_files<'a, T: AsRef<str>>(
    command: &ReadYamlConfigFilesCommand<'a, T>,
) -> Yaml {
    let lc = local_config::read_profiles_configs(command.profiles, command.config_dir);
    yaml::parse_yaml_config(&lc)
}
