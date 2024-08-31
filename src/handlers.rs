use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, AppMode};

pub fn handle_key_event(key: KeyEvent, app: &mut App) -> Result<bool, Box<dyn std::error::Error>> {
    match key.code {
        KeyCode::Char('q') => return Ok(false),
        KeyCode::Tab => {
            app.selected_tab = (app.selected_tab + 1) % 3;
            app.app_mode = match app.selected_tab {
                0 => AppMode::EpisodesSeries(app.selected_series_tab),
                1 => AppMode::MoviesList,
                2 => AppMode::Characters,
                _ => app.app_mode.clone(),
            };
            app.reset_list_state_for_tab();
        }
        KeyCode::Left => {
            if app.selected_tab == 0 {
                app.selected_series_tab = if app.selected_series_tab > 0 {
                    app.selected_series_tab - 1
                } else {
                    app.guide.len().saturating_sub(1)
                };
                app.app_mode = AppMode::EpisodesSeries(app.selected_series_tab);
                app.reset_list_state_for_tab();
            }
        }
        KeyCode::Right => {
            if app.selected_tab == 0 {
                app.selected_series_tab = (app.selected_series_tab + 1) % app.guide.len();
                app.app_mode = AppMode::EpisodesSeries(app.selected_series_tab);
                app.reset_list_state_for_tab();
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
        _ => {}
    }
    Ok(true)
}