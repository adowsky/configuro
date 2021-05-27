static PROFILES_VAR: &str = "APP_PROFILES";

pub fn discover_profiles(default_profile: Option<&str>) -> Vec<String> {
    std::env::var(PROFILES_VAR)
        .unwrap_or_else(|_| default_profile.unwrap_or("").into())
        .split(',')
        .map(|profile| profile.trim())
        .filter(|profile| !profile.is_empty())
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::profiles::PROFILES_VAR;
    use std::env::{remove_var, set_var, var};

    use super::discover_profiles;

    fn empty_vec() -> Vec<String> {
        Vec::new()
    }
    #[test]
    fn should_return_empty_when_no_env_var() {
        remove_var(PROFILES_VAR);
        assert_eq!(discover_profiles(None), empty_vec())
    }

    #[test]
    fn should_return_default_when_no_env_var() {
        remove_var(PROFILES_VAR);
        assert_eq!(
            discover_profiles(Some("default")),
            vec!["default".to_string()]
        )
    }
    #[test]
    fn should_discover_single_profile() {
        set_var(PROFILES_VAR, "test_profile");
        assert_eq!(discover_profiles(None), vec!["test_profile".to_string()]);
    }

    #[test]
    fn should_discover_multiple_profiles() {
        set_var(PROFILES_VAR, "profile1,profile2");

        assert_eq!(
            discover_profiles(None),
            vec!["profile1".to_string(), "profile2".into()]
        );
    }
}
