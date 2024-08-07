use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Episode {
    pub id: u32,
    pub title: String,
    pub summary: String,
    pub key_events: Vec<String>,
    pub character_appearances: Vec<String>,
    pub user_rating: f32,
    pub watched: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub summary: String,
    pub key_events: Vec<String>,
    pub character_appearances: Vec<String>,
    pub user_rating: f32,
    pub watched: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DragonBallGuide {
    pub episodes: Vec<Episode>,
    pub movies: Vec<Movie>,
}
