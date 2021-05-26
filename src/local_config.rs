pub fn read_profiles_configs<T: AsRef<str>>(profiles: &[T], config_dir_path: &str) -> Vec<String> {
    if !path_exists(&config_dir_path) {
        return Vec::new();
    }

    let config_file = |file: &str| format!("{}/{}", config_dir_path, file);

    profiles
        .iter()
        .map(|profile| config_file(&format!("application-{}.yml", profile.as_ref())))
        .fold(vec![config_file("application.yml")], |mut acc, file| {
            acc.push(file);
            acc
        })
        .iter()
        .filter(|file_path| std::fs::metadata(file_path).is_ok())
        .map(std::fs::read_to_string)
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect()
}

fn path_exists(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}
