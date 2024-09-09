use tui::widgets::ListState;
use crate::data::{Series, Movie, load_guide_from_file, load_movies_from_file};
#[derive(Debug, Clone, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EpisodeSortMethod {
    EpisodeNumber,
    Title,
    ReleaseDate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MovieSortMethod {
    Number,
    Title,
    ReleaseDate,
}

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
    pub previous_mode: AppMode,
    pub episode_sort_method: EpisodeSortMethod,
    pub episode_sort_order: SortOrder,
    pub movie_sort_method: MovieSortMethod,
    pub movie_sort_order: SortOrder,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AppMode {
    Characters,
    MoviesList,
    Details(usize, usize),
    EpisodesSeries(usize),
    MovieDetails(usize),
    Search,
    Help,
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
            previous_mode: AppMode::EpisodesSeries(0),
            episode_sort_method: EpisodeSortMethod::EpisodeNumber,
            episode_sort_order: SortOrder::Ascending,
            movie_sort_method: MovieSortMethod::Number,
            movie_sort_order: SortOrder::Ascending,
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

    
    pub fn toggle_episode_sort_method(&mut self) {
        self.episode_sort_method = match self.episode_sort_method {
            EpisodeSortMethod::EpisodeNumber => EpisodeSortMethod::Title,
            EpisodeSortMethod::Title => EpisodeSortMethod::ReleaseDate,
            EpisodeSortMethod::ReleaseDate => EpisodeSortMethod::EpisodeNumber,
        };
        self.sort_episodes();
    }

    pub fn toggle_episode_sort_order(&mut self) {
        self.episode_sort_order = match self.episode_sort_order {
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Ascending,
        };
        self.sort_episodes();
    }

    pub fn toggle_movie_sort_method(&mut self) {
        self.movie_sort_method = match self.movie_sort_method {
            MovieSortMethod::Number => MovieSortMethod::Title,
            MovieSortMethod::Title => MovieSortMethod::ReleaseDate,
            MovieSortMethod::ReleaseDate => MovieSortMethod::Number,
        };
        self.sort_movies();
    }

    pub fn toggle_movie_sort_order(&mut self) {
        self.movie_sort_order = match self.movie_sort_order {
            SortOrder::Ascending => SortOrder::Descending,
            SortOrder::Descending => SortOrder::Ascending,
        };
        self.sort_movies();
    }

    fn sort_episodes(&mut self) {
        for series in &mut self.guide {
            series.episodes.sort_by(|a, b| {
                let cmp = match self.episode_sort_method {
                    EpisodeSortMethod::EpisodeNumber => a.episode_number.cmp(&b.episode_number),
                    EpisodeSortMethod::Title => a.title.cmp(&b.title),
                    EpisodeSortMethod::ReleaseDate => a.release_date.cmp(&b.release_date),
                };
                match self.episode_sort_order {
                    SortOrder::Ascending => cmp,
                    SortOrder::Descending => cmp.reverse(),
                }
            });
        }
    }

    fn sort_movies(&mut self) {
        self.movies.sort_by(|a, b| {
            let cmp = match self.movie_sort_method {
                MovieSortMethod::Number => a.number.cmp(&b.number),
                MovieSortMethod::Title => a.title.cmp(&b.title),
                MovieSortMethod::ReleaseDate => a.release_date.cmp(&b.release_date),
            };
            match self.movie_sort_order {
                SortOrder::Ascending => cmp,
                SortOrder::Descending => cmp.reverse(),
            }
        });
    }
}