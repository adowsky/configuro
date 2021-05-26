use crate::yaml::YamlMerger;
use serde::de::DeserializeOwned;
use yaml_rust::{Yaml, YamlEmitter};

#[derive(Debug)]
pub enum ConfigProviderError {
    IoError,
    ParseError,
}

#[derive(Clone)]
pub struct ConfigProvider {
    raw_config: String,
}

impl ConfigProvider {
    pub fn form_yamls(yamls: &mut [Yaml]) -> ConfigProvider {
        let final_yaml: Yaml = yamls
            .iter_mut()
            .fold(Yaml::from_str(""), |acc, next| acc.merge(next));

        let mut raw_string_yaml = String::new();
        YamlEmitter::new(&mut raw_string_yaml)
            .dump(&final_yaml)
            .unwrap();

        ConfigProvider {
            raw_config: raw_string_yaml,
        }
    }

    pub fn config<T: DeserializeOwned>(&self) -> Result<T, ConfigProviderError> {
        serde_yaml::from_str::<T>(&self.raw_config).map_err(|_| ConfigProviderError::ParseError)
    }
}
