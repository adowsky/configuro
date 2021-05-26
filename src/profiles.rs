pub fn discover_profiles(default_profile: Option<&str>) -> Vec<String> {
    std::env::var("APP_PROFILES")
        .unwrap_or_else(|_| default_profile.unwrap_or("").into())
        .split(',')
        .map(|profile| profile.trim())
        .map(String::from)
        .collect()
}
