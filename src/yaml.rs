use linked_hash_map::LinkedHashMap;
use yaml_rust::{Yaml, YamlLoader};

pub(crate) trait YamlMerger {
    fn merge(self, other: &mut Yaml) -> Yaml;
}

impl YamlMerger for Yaml {
    fn merge(self, other: &mut Yaml) -> Yaml {
        let mut result = if let Yaml::Hash(hash_map) = self {
            hash_map
        } else {
            LinkedHashMap::new()
        };

        let mut fallback = LinkedHashMap::new();
        if let Yaml::Hash(hash_map) = other {
            hash_map
        } else {
            &mut fallback
        }
        .iter_mut()
        .for_each(|(k, v)| {
            let final_value = result
                .get_mut(k)
                .filter(|current_v| match current_v {
                    Yaml::Hash(_) => true,
                    _ => false,
                })
                .map(|current_v| current_v.clone().merge(v))
                .unwrap_or_else(|| v.clone());

            result.insert(k.clone(), final_value);
        });

        Yaml::Hash(result)
    }
}

pub fn parse_yaml_config(sources: &[String]) -> Yaml {
    sources
        .iter()
        .map(|raw_config| {
            YamlLoader::load_from_str(&raw_config)
                .expect("Each source to be parsed properly as yaml")
        })
        .flatten()
        .reduce(|e1, mut e2| e1.merge(&mut e2))
        .unwrap_or_else(|| Yaml::from_str(""))
}
