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

fn read_file_content(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(From::from(format!("File not found: {}", file_path)));
    }
    fs::read_to_string(path).map_err(|e| e.into())
}

fn write_file_content(file_path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes()).map_err(|e| e.into())
}

pub fn load_guide_from_file(file_path: &str) -> Result<Vec<Series>, Box<dyn std::error::Error>> {
    let file_content = read_file_content(file_path)?;
    serde_json::from_str(&file_content).map_err(|e| e.into())
}

#[allow(dead_code)]
pub fn save_guide_to_file(guide: &[Series], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = serde_json::to_string_pretty(guide)?;
    write_file_content(file_path, &file_content)
}

pub fn load_movies_from_file(file_path: &str) -> Result<Vec<Movie>, Box<dyn std::error::Error>> {
    let file_content = read_file_content(file_path)?;
    serde_json::from_str(&file_content).map_err(|e| e.into())
}

#[allow(dead_code)]
pub fn save_movies_to_file(movies: &[Movie], file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = serde_json::to_string_pretty(movies)?;
    write_file_content(file_path, &file_content)
}