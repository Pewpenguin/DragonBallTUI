use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, AppMode, SearchResultType};

pub fn handle_key_event(key: KeyEvent, app: &mut App) -> Result<bool, Box<dyn std::error::Error>> {
    match app.app_mode {
        AppMode::Help => {
            if key.code == KeyCode::Esc || key.code == KeyCode::Char('h') {
                app.app_mode = app.previous_mode.clone();
            }
        }
        AppMode::Search => {
            match key.code {
                KeyCode::Esc => {
                    // Exit search mode
                    app.app_mode = match app.selected_tab {
                        0 => AppMode::EpisodesSeries(app.selected_series_tab),
                        1 => AppMode::MoviesList,
                        2 => AppMode::Characters,
                        _ => app.app_mode.clone(),
                    };
                    app.search_results.clear();
                }
                KeyCode::Char(c) => {
                    app.search_query.push(c);
                    app.perform_search();
                }
                KeyCode::Backspace => {
                    app.search_query.pop();
                    app.perform_search();
                }
                KeyCode::Enter => {
                    if let Some(selected) = app.list_state.selected() {
                        if let Some(result) = app.search_results.get(selected) {
                            match result.result_type {
                                SearchResultType::Episode(series_index, episode_index) => {
                                    app.app_mode = AppMode::Details(series_index, episode_index);
                                    app.selected_tab = 0;
                                    app.selected_series_tab = series_index;
                                }
                                SearchResultType::Movie(movie_index) => {
                                    app.app_mode = AppMode::MovieDetails(movie_index);
                                    app.selected_tab = 1;
                                }
                            }
                            app.search_results.clear();
                        }
                    }
                }
                KeyCode::Down => {
                    if let Some(selected) = app.list_state.selected() {
                        if selected < app.search_results.len() - 1 {
                            app.list_state.select(Some(selected + 1));
                        }
                    } else {
                        app.list_state.select(Some(0));
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = app.list_state.selected() {
                        if selected > 0 {
                            app.list_state.select(Some(selected - 1));
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {
            match key.code {
                KeyCode::Char('q') => return Ok(false),
                KeyCode::Char('h') => {
                    app.previous_mode = app.app_mode.clone();
                    app.app_mode = AppMode::Help;
                }
                KeyCode::Tab => {
                    if !matches!(app.app_mode, AppMode::Details(_, _) | AppMode::MovieDetails(_)) {
                        app.selected_tab = (app.selected_tab + 1) % 3;
                        app.app_mode = match app.selected_tab {
                            0 => AppMode::EpisodesSeries(app.selected_series_tab),
                            1 => AppMode::MoviesList,
                            2 => AppMode::Characters,
                            _ => app.app_mode.clone(),
                        };
                        app.reset_list_state_for_tab();
                    }
                }
                KeyCode::Left | KeyCode::Right => {
                    if !matches!(app.app_mode, AppMode::Details(_, _) | AppMode::MovieDetails(_)) {
                    if app.selected_tab == 0 {
                        let num_series = app.guide.len();
                        if key.code == KeyCode::Left {
                            app.selected_series_tab = (app.selected_series_tab + num_series - 1) % num_series;
                        } else {
                            app.selected_series_tab = (app.selected_series_tab + 1) % num_series;
                        }
                        app.app_mode = AppMode::EpisodesSeries(app.selected_series_tab);
                        app.reset_list_state_for_tab();
                    }
                    }
                }
                KeyCode::Down => {
                    if let Some(selected) = app.list_state.selected() {
                        let count = match app.selected_tab {
                            0 => app.guide[app.selected_series_tab].episodes.len(),
                            1 => app.movies.len(),
                            _ => 0,
                        };
                        if selected < count - 1 {
                            app.list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = app.list_state.selected() {
                        if selected > 0 {
                            app.list_state.select(Some(selected - 1));
                        }
                    }
                }
                KeyCode::Esc => {
                    match app.app_mode {
                        AppMode::Details(_, _) => {
                            app.app_mode = AppMode::EpisodesSeries(app.selected_series_tab);
                        }
                        AppMode::MovieDetails(_) => {
                            app.app_mode = AppMode::MoviesList;
                        }
                        AppMode::Search => {
                            app.app_mode = match app.selected_tab {
                                0 => AppMode::EpisodesSeries(app.selected_series_tab),
                                1 => AppMode::MoviesList,
                                2 => AppMode::Characters,
                                _ => app.app_mode.clone(),
                            };
                        }
                        _ => {}
                    }
                }
                KeyCode::Enter => {
                    match app.app_mode {
                        AppMode::EpisodesSeries(series_index) => {
                            if let Some(episode_index) = app.list_state.selected() {
                                app.app_mode = AppMode::Details(series_index, episode_index);
                            }
                        }
                        AppMode::MoviesList => {
                            if let Some(movie_index) = app.list_state.selected() {
                                app.app_mode = AppMode::MovieDetails(movie_index);
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Char('s') => {
                    app.app_mode = AppMode::Search;
                    app.search_query.clear();
                }
                _ => {
                    if let AppMode::Search = app.app_mode {
                        if let KeyCode::Char(c) = key.code {
                            app.search_query.push(c);
                        } else if let KeyCode::Backspace = key.code {
                            app.search_query.pop();
                        }
                    }
                }
            }
        }
    }
    Ok(true)
}