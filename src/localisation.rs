use std::collections::HashMap;
use std::fs;

const RES_I18N: &str = "./res/localisation/";

///Simple implementation of a localisation system
pub struct I18n {
    dict: HashMap<String, String>,
}

impl I18n {
    pub fn from(lang: &str) -> Result<I18n, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(&(RES_I18N.to_owned() + lang + ".ron"))?;

        let dict: HashMap<String, String> = ron::from_str(&file_content)?;

        return Ok(I18n { dict });
    }

    pub fn get_translation(&self, key: &str) -> &String {
        return &self.dict[key];
    }
}
