use crate::gpu::{self, SSBO};

use crate::gpu::ISSBO;
use serde::Deserialize;
use std::fs;

const RES_SETTINGS: &str = "./res/settings.ron";

pub struct Settings {
    core: SettingsCore,
    _ssbo: SSBO,
}

#[derive(Deserialize)]
#[repr(C)]
pub struct SettingsCore {
    resolution: (u32, u32),
    language: String,
}

impl ISSBO for SettingsCore {}

impl Settings {
    pub fn load() -> Settings {
        let core = match fs::read_to_string(RES_SETTINGS) {
            Ok(file_content) => match ron::from_str::<SettingsCore>(&file_content) {
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
        };

        Settings {
            core,
            _ssbo: gpu::SSBO::null(),
        }
    }

    fn default() -> SettingsCore {
        SettingsCore {
            resolution: (800, 600),
            language: String::from("en_GB"),
        }
    }

    pub fn copy_to_gpu(&mut self) {
        let ssbo = gpu::SSBO::from(1, &self.core, gl::STATIC_DRAW);
        self._ssbo = ssbo;
    }

    pub fn resolution(&self) -> &(u32, u32) {
        return &self.core.resolution;
    }

    pub fn language(&self) -> &String {
        return &self.core.language;
    }
}
