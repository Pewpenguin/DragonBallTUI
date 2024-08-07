use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Episode {
    pub episode_number: u32,
    pub title: String,
    pub description: String,
    pub release_date: String,
    pub duration: String,
    pub saga: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Series {
    pub series: String,
    pub episodes: Vec<Episode>,
}

pub type DragonBallGuide = Vec<Series>; // Adjusted to match JSON structure

pub fn load_guide_from_file(filename: &str) -> Result<DragonBallGuide, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let guide = serde_json::from_reader(reader)?;
    Ok(guide)
}

pub fn save_guide_to_file(guide: &DragonBallGuide, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, guide)?;
    Ok(())
}
