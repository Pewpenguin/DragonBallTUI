use tui::widgets::ListState;
use crate::data::{Series, Movie, load_guide_from_file, load_movies_from_file};

pub struct App {
    pub guide: Vec<Series>,
    pub movies: Vec<Movie>,
    pub list_state: ListState,
    pub app_mode: AppMode,
    pub selected_tab: usize,
    pub selected_series_tab: usize,
    pub previous_tab: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AppMode {
    Characters,
    MoviesList,
    Details(usize, usize),
    EpisodesSeries(usize),
    MovieDetails(usize),
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let guide = load_guide_from_file("data/episodes.json")?;
        let movies = load_movies_from_file("data/movies.json")?;

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(Self {
            guide,
            movies,
            list_state,
            app_mode: AppMode::EpisodesSeries(0),
            selected_tab: 0,
            selected_series_tab: 0,
            previous_tab: 0,
        })
    }

    pub fn reset_list_state_for_tab(&mut self) {
        match self.selected_tab {
            0 => {
                if self.selected_series_tab < self.guide.len() {
                    self.list_state.select(Some(0));
                }
            }
            1 => {
                if !self.movies.is_empty() {
                    self.list_state.select(Some(0));
                }
            }
            _ => self.list_state.select(None),
        }
    }
}