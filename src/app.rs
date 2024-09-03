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
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AppMode {
    Characters,
    MoviesList,
    Details(usize, usize),
    EpisodesSeries(usize),
    MovieDetails(usize),
    Search,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub result_type: SearchResultType,
    pub title: String,
}

#[derive(Debug, Clone)]
pub enum SearchResultType {
    Episode(usize, usize), // (series_index, episode_index)
    Movie(usize),          // movie_index
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
            search_query: String::new(),
            search_results: Vec::new(),
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

    pub fn perform_search(&mut self) {
        self.search_results.clear();
        let query = self.search_query.to_lowercase();

        // Search episodes
        for (series_index, series) in self.guide.iter().enumerate() {
            for (episode_index, episode) in series.episodes.iter().enumerate() {
                if episode.title.to_lowercase().contains(&query) || episode.description.to_lowercase().contains(&query) {
                    self.search_results.push(SearchResult {
                        result_type: SearchResultType::Episode(series_index, episode_index),
                        title: format!("{} - {}", series.series, episode.title),
                    });
                }
            }
        }

        // Search movies
        for (movie_index, movie) in self.movies.iter().enumerate() {
            if movie.title.to_lowercase().contains(&query) || movie.description.to_lowercase().contains(&query) {
                self.search_results.push(SearchResult {
                    result_type: SearchResultType::Movie(movie_index),
                    title: movie.title.clone(),
                });
            }
        }
    }
}