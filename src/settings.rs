use super::gpu::ISSBO;
use serde::Deserialize;
use std::fs;

const RES_SETTINGS: &str = "./res/settings.ron";

#[derive(Deserialize)]
#[repr(C)]
pub struct Settings {
    pub resolution: (u32, u32),
    pub language: String,
}

impl ISSBO for Settings {}

impl Settings {
    pub fn load() -> Settings {
        match fs::read_to_string(RES_SETTINGS) {
            Ok(file_content) => match ron::from_str::<Settings>(&file_content) {
                Ok(settings) => settings,
                Err(e) => {
                    eprintln!("Couldn't load settings, using defaults: {}", e.to_string());
                    Settings::default()
                }
            },
            Err(e) => {
                eprintln!("Couldn't load settings, using defaults: {}", e.to_string());
                Settings::default()
            }
        }
    }

    fn default() -> Settings {
        Settings {
            resolution: (800, 600),
            language: String::from("en_GB"),
        }
    }
}
