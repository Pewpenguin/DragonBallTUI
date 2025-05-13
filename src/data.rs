use serde::{Deserialize, Serialize};
use std::fs;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Movie {
    pub number: u32,
    pub title: String,
    pub release_date: String,
    pub runtime: String,
    pub description: String,
    pub director: String,
    pub genres: Vec<String>,
    pub trivia: String,
    pub plot_keywords: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Character {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub series: Vec<String>,
    pub race: String,
    pub powers: Vec<String>,
    pub occupation: String,
    pub family: Vec<String>,
    pub key_events: Vec<String>,
}

fn read_file_content(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(From::from(format!("File not found: {}", file_path)));
    }
    fs::read_to_string(path).map_err(|e| e.into())
}

pub fn load_guide_from_file(file_path: &str) -> Result<Vec<Series>, Box<dyn std::error::Error>> {
    let file_content = read_file_content(file_path)?;
    serde_json::from_str(&file_content).map_err(|e| e.into())
}

pub fn load_movies_from_file(file_path: &str) -> Result<Vec<Movie>, Box<dyn std::error::Error>> {
    let file_content = read_file_content(file_path)?;
    serde_json::from_str(&file_content).map_err(|e| e.into())
}

pub fn load_characters_from_file(
    file_path: &str,
) -> Result<Vec<Character>, Box<dyn std::error::Error>> {
    let file_content = read_file_content(file_path)?;
    serde_json::from_str(&file_content).map_err(|e| e.into())
}
