use crate::yaml::YamlMerger;
use serde::de::DeserializeOwned;
use yaml_rust::{Yaml, YamlEmitter};

#[derive(Debug)]
pub enum ConfigProviderError {
    IoError,
    ParseError,
}

/// Tool to transform read configuration to Configuration DTO .
#[derive(Clone)]
pub struct ConfigProvider {
    raw_config: String,
}

impl ConfigProvider {
    /// Construction function that accepts array of YAML configurations,
    /// merges it and transforms it into `ConfigProvider`.
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

    /// Config factory that creates serde's Deserialized DTO from stored configuration.
    /// ```rust
    /// #[derive(Debug, Deserialize, PartialEq, Eq)]
    /// struct ConfigDto {
    ///     pub my_int: i32,
    /// }
    ///
    /// # main() {
    /// let cp = ConfigProvider::form_yamls(&[Yaml::from_str("")]);
    /// let config_dto = cp.create_config<ConfigDto>();
    /// # } 
    pub fn create_config<T: DeserializeOwned>(&self) -> Result<T, ConfigProviderError> {
        serde_yaml::from_str::<T>(&self.raw_config).map_err(|_| ConfigProviderError::ParseError)
    }
}
