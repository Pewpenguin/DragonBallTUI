use crate::app::{App, AppMode, SearchResultType};
use crossterm::event::{KeyCode, KeyEvent};

fn app_mode(app: &mut App) {
    app.app_mode = match app.selected_tab {
        0 => AppMode::EpisodesSeries(app.selected_series_tab),
        1 => AppMode::MoviesList,
        2 => AppMode::Characters,
        _ => app.app_mode.clone(),
    };
}

pub fn handle_key_event(key: KeyEvent, app: &mut App) -> Result<bool, Box<dyn std::error::Error>> {
    match app.app_mode {
        AppMode::Help => {
            if matches!(
                key.code,
                KeyCode::Esc | KeyCode::Char('H') | KeyCode::Char('h')
            ) {
                app.app_mode = app.previous_mode.clone();
            }
        }
        AppMode::Search => match key.code {
            KeyCode::Esc => {
                app_mode(app);
                app.search_results.clear();
            }
            KeyCode::Char(c) if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                if !app.search_results.is_empty() {
                    match c {
                        'p' | 'P' => {
                            app.search_pagination.prev_page();
                            app.list_state.select(Some(0));
                        },
                        'n' | 'N' => {
                            app.search_pagination.next_page();
                            app.list_state.select(Some(0));
                        },
                        'f' | 'F' => {
                            app.search_pagination.first_page();
                            app.list_state.select(Some(0));
                        },
                        'l' | 'L' => {
                            app.search_pagination.last_page();
                            app.list_state.select(Some(0));
                        },
                        _ => {}
                    }
                }
            }
            KeyCode::Char(c) => {
                app.search_query.push(c);
                app.perform_search();
                app.search_pagination.first_page();
            }
            KeyCode::Backspace => {
                app.search_query.pop();
                app.perform_search();
                // Reset pagination when search query changes
                app.search_pagination.first_page();
            }
            KeyCode::Enter => {
                if let Some(selected) = app.list_state.selected() {
                    // Calculate the actual index based on pagination
                    let start = app.search_pagination.current_page * app.search_pagination.items_per_page;
                    let actual_index = start + selected;
                    
                    if let Some(result) = app.search_results.get(actual_index) {
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
                            SearchResultType::Character(character_index) => {
                                app.app_mode = AppMode::CharacterDetails(character_index);
                                app.selected_tab = 2;
                            }
                        }
                        app.search_results.clear();
                    }
                }
            }
            KeyCode::Down => {
                let visible_items = app.search_pagination.get_visible_items(&app.search_results).len();
                if let Some(selected) = app.list_state.selected() {
                    if selected < visible_items - 1 {
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
        },
        _ => match key.code {
            KeyCode::Char(c) => {
                let lower_c = c.to_lowercase().next().unwrap_or(c);
                match lower_c {
                    'q' => return Ok(false),
                    'h' => {
                        app.previous_mode = app.app_mode.clone();
                        app.app_mode = AppMode::Help;
                    }
                    's' => {
                        app.app_mode = AppMode::Search;
                        app.search_query.clear();
                    }
                    'm' => {
                        if app.selected_tab == 0 {
                            app.toggle_episode_sort_method();
                        } else if app.selected_tab == 1 {
                            app.toggle_movie_sort_method();
                        }
                    },
                    'o' => {
                        if app.selected_tab == 0 {
                            app.toggle_episode_sort_order();
                        } else if app.selected_tab == 1 {
                            app.toggle_movie_sort_order();
                        } else if app.selected_tab == 2 {
                            app.toggle_character_sort_order();
                        }
                    },
                    'n' | 'N' => {
                        // Next page
                        match app.selected_tab {
                            0 => {
                                app.episodes_pagination.next_page();
                                app.list_state.select(Some(0));
                            },
                            1 => {
                                app.movies_pagination.next_page();
                                app.list_state.select(Some(0));
                            },
                            2 => {
                                app.characters_pagination.next_page();
                                app.list_state.select(Some(0));
                            },
                            _ => {}
                        }
                    },
                    'P' | 'p' => {
                        // Previous page
                        match app.selected_tab {
                            0 => {
                                app.episodes_pagination.prev_page();
                                app.list_state.select(Some(0));
                            },
                            1 => {
                                app.movies_pagination.prev_page();
                                app.list_state.select(Some(0));
                            },
                            2 => {
                                app.characters_pagination.prev_page();
                                app.list_state.select(Some(0));
                            },
                            _ => {}
                        }
                    },

                    'f' | 'F' => {
                        // First page
                        match app.selected_tab {
                            0 => {
                                app.episodes_pagination.first_page();
                                app.list_state.select(Some(0));
                            },
                            1 => {
                                app.movies_pagination.first_page();
                                app.list_state.select(Some(0));
                            },
                            2 => {
                                app.characters_pagination.first_page();
                                app.list_state.select(Some(0));
                            },
                            _ => {}
                        }
                    },
                    'l' | 'L' => {
                        // Last page
                        match app.selected_tab {
                            0 => {
                                app.episodes_pagination.last_page();
                                app.list_state.select(Some(0));
                            },
                            1 => {
                                app.movies_pagination.last_page();
                                app.list_state.select(Some(0));
                            },
                            2 => {
                                app.characters_pagination.last_page();
                                app.list_state.select(Some(0));
                            },
                            _ => {}
                        }
                    },
                    't' => {
                        // Cycle through themes
                        let themes = &app.config.themes;
                        let current_theme = &app.config.theme;
                        let current_index = themes.iter().position(|t| t.name == *current_theme).unwrap_or(0);
                        let next_index = (current_index + 1) % themes.len();
                        app.config.theme = themes[next_index].name.clone();
                    }
                    _ => {}
                }
            }
            KeyCode::Tab => {
                if !matches!(
                    app.app_mode,
                    AppMode::Details(_, _) | AppMode::MovieDetails(_)
                ) {
                    app.selected_tab = (app.selected_tab + 1) % 3;
                    app_mode(app);
                    app.reset_list_state_for_tab();
                }
            }
            KeyCode::Left | KeyCode::Right => {
                if app.selected_tab == 0 {
                    let num_series = app.guide.len();
                    app.selected_series_tab = if key.code == KeyCode::Left {
                        (app.selected_series_tab + num_series - 1) % num_series
                    } else {
                        (app.selected_series_tab + 1) % num_series
                    };
                    app.app_mode = AppMode::EpisodesSeries(app.selected_series_tab);
                    app.reset_list_state_for_tab();
                }
            }
            KeyCode::Down => {
                if let Some(selected) = app.list_state.selected() {
                    let visible_items_count = match app.selected_tab {
                        0 => app.episodes_pagination.get_visible_items(&app.guide[app.selected_series_tab].episodes).len(),
                        1 => app.movies_pagination.get_visible_items(&app.movies).len(),
                        2 => app.characters_pagination.get_visible_items(&app.characters).len(),
                        _ => 0,
                    };
                    if selected < visible_items_count - 1 {
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
            KeyCode::Esc => match app.app_mode {
                AppMode::Details(_, _) => {
                    app.app_mode = AppMode::EpisodesSeries(app.selected_series_tab);
                }
                AppMode::MovieDetails(_) => {
                    app.app_mode = AppMode::MoviesList;
                }
                AppMode::CharacterDetails(_) => {
                    app.app_mode = AppMode::Characters;
                }
                AppMode::Search => {
                    app_mode(app);
                }
                _ => {}
            },
            KeyCode::Enter => match app.app_mode {
                AppMode::EpisodesSeries(series_index) => {
                    if let Some(selected) = app.list_state.selected() {
                        let start = app.episodes_pagination.current_page * app.episodes_pagination.items_per_page;
                        let actual_index = start + selected;
                        
                        if actual_index < app.guide[series_index].episodes.len() {
                            app.app_mode = AppMode::Details(series_index, actual_index);
                        }
                    }
                }
                AppMode::MoviesList => {
                    if let Some(selected) = app.list_state.selected() {
                        let start = app.movies_pagination.current_page * app.movies_pagination.items_per_page;
                        let actual_index = start + selected;
                        
                        if actual_index < app.movies.len() {
                            app.app_mode = AppMode::MovieDetails(actual_index);
                        }
                    }
                }
                AppMode::Characters => {
                    if let Some(selected) = app.list_state.selected() {
                        let start = app.characters_pagination.current_page * app.characters_pagination.items_per_page;
                        let actual_index = start + selected;
                        
                        if actual_index < app.characters.len() {
                            app.app_mode = AppMode::CharacterDetails(actual_index);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        },
    }
    Ok(true)
}
