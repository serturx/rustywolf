use serde::Deserialize;
use std::{ffi::OsString, fs};

#[derive(Deserialize)]
pub struct SpriteAsset {
    pub file: String,
    pub columns: u32,
    pub rows: u32,
}

impl SpriteAsset {
    pub fn load(path: &OsString) -> Result<SpriteAsset, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(path)?;
        let mut asset = ron::from_str::<SpriteAsset>(&file_content)?;

        let total_path = std::path::Path::new(path);
        let parent = total_path.parent().unwrap().to_str().unwrap().to_string();
        asset.file = parent + "/" + &asset.file;

        Ok(asset)
    }
}
