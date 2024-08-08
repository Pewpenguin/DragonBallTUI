use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Episode {
    pub episode_number: u32,
    pub title: String,
    pub description: String,
    pub release_date: String,
    pub duration: String,
    pub saga: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Series {
    pub series: String,
    pub episodes: Vec<Episode>,
}

pub fn load_guide_from_file(file_path: &str) -> Result<Vec<Series>, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(From::from(format!("File not found: {}", file_path)));
    }

    let file_content = fs::read_to_string(path)?;
    let guide: Vec<Series> = serde_json::from_str(&file_content)?;
    Ok(guide)
}

pub fn save_guide_to_file(guide: &Vec<Series>, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let file_content = serde_json::to_string_pretty(&guide)?;
    let mut file = fs::File::create(path)?;
    file.write_all(file_content.as_bytes())?;
    Ok(())
}
