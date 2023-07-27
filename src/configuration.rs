use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Topic {
    pub title: String,
    pub explanation: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Thematic {
    pub title: String,
    pub topics: Vec<Topic>,
}

pub fn load_env() {
    dotenv().ok();
}

pub fn get_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

pub fn load_roadmap() -> Vec<Thematic> {
    self::load_env();
    let roadmap_path = get_var("ROADMAP_FILE_PATH").unwrap();
    let mut file = File::open(roadmap_path).expect("Failed to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config file");
    serde_yaml::from_str(&contents).expect("Failed to parse YAML")
}

