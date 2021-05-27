use std::str;

use configuro::ReadYamlConfigFilesCommand;
use serde::Deserialize;

static CONFIG_DIR: &str = "tests/config";

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct TestConfig {
    pub param: Param,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct Param {
    value1: String,
    value2: String,
    int_value: i32,
    missing_value: Option<String>,
}
#[test]
fn test_parsing_parsing_config_with_profile() {
    assert_eq!(
        read_config(&["profile1"]),
        TestConfig {
            param: Param {
                value1: "such a value".to_string(),
                value2: "value 2".to_string(),
                int_value: 32,
                missing_value: None
            }
        }
    )
}

#[test]
fn test_parsing_default_profile() {
    assert_eq!(
        read_config(&[]),
        TestConfig {
            param: Param {
                value1: "base value1".to_string(),
                value2: "base value 2".to_string(),
                int_value: 0,
                missing_value: None
            }
        }
    )
}

fn read_config(profiles: &[&str]) -> TestConfig {
    let yaml = configuro::read_yaml_config_from_files(&ReadYamlConfigFilesCommand::new(
        profiles, CONFIG_DIR,
    ));
    let config_provider = configuro::ConfigProvider::form_yamls(&mut [yaml]);
    return config_provider.create_config::<TestConfig>().unwrap();
}
