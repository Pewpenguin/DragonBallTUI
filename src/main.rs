mod data;
use std::io;
use std::path::Path;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Terminal,
};

use data::{
    load_guide_from_file, save_guide_to_file, load_movies_from_file, save_movies_to_file, Series, Episode, Movie
};

#[derive(Debug, PartialEq)]
enum AppMode {
    Characters,
    MoviesList,
    Details(usize, usize), // (series_index, episode_index) for episodes
    EpisodesSeries(usize), // series_index for episodes
    MovieDetails(usize),   // movie_index for movies
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let guide_path = "episodes.json";
    let movies_path = "movies.json";

    // Check if the files exist, create default ones if not
    if !Path::new(guide_path).exists() {
        println!("File not found. Creating a default file.");
        let default_guide = vec![Series {
            series: "Dragon Ball".to_string(),
            episodes: vec![Episode {
                episode_number: 1,
                title: "The Secret of the Dragon Balls".to_string(),
                description: "Bulma's search for six more Dragon Balls leads her to a remote valley...".to_string(),
                release_date: "February 26, 1986".to_string(),
                duration: "25m".to_string(),
                saga: "Emperor Pilaf Saga (1986)".to_string(),
            }],
        }];
        save_guide_to_file(&default_guide, guide_path)?;
    }

    if !Path::new(movies_path).exists() {
        println!("Movies file not found. Creating a default file.");
        let default_movies = vec![Movie {
            number: 1,
            title: "Dragon Ball: Curse of the Blood Rubies".to_string(),
            release_date: "December 20, 1986".to_string(),
            runtime: "50m".to_string(),
            description: "Goku and his friends must stop King Gurumes from destroying the city for blood rubies and gathering the seven Dragon Balls.".to_string(),
            director: "Daisuke Nishio".to_string(),
            genres: vec!["Action".to_string(), "Adventure".to_string(), "Fantasy".to_string()],
            trivia: "This movie is a retelling of the first story arc of the original Dragon Ball anime.".to_string(),
            plot_keywords: vec!["Dragon Balls".to_string(), "King Gurumes".to_string(), "Blood Rubies".to_string(), "Martial Arts".to_string()],
        }];
        save_movies_to_file(&default_movies, movies_path)?;
    }

    // Load data
    let guide = load_guide_from_file(guide_path)?;
    let movies = load_movies_from_file(movies_path)?;

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(crossterm::cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut list_state = tui::widgets::ListState::default();
    list_state.select(Some(0));

    let mut app_mode = AppMode::EpisodesSeries(0);
    let mut selected_tab = 0;
    let mut selected_series_tab = 0;
    let mut previous_tab = 0;

    // Function to reset the list state for each tab
    fn reset_list_state_for_tab(
        selected_tab: usize,
        list_state: &mut tui::widgets::ListState,
        guide_len: usize,
        movies_len: usize,
        selected_series_tab: usize,
    ) {
        match selected_tab {
            0 => {
                // Reset for Episodes tab
                if selected_series_tab < guide_len {
                    list_state.select(Some(0));
                }
            }
            1 => {
                // Reset for Movies tab
                if movies_len > 0 {
                    list_state.select(Some(0));
                }
            }
            _ => list_state.select(None), // Characters tab or others
        }
    }

    // Main loop
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let layout_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // Main tabs
                    Constraint::Min(1),     // Content
                ].as_ref())
                .split(size);

            // Main Tabs header
            let tab_titles = ["Episodes", "Movies", "Characters"];
            let spans: Vec<Spans> = tab_titles.iter().map(|&t| {
                Spans::from(vec![Span::styled(t, Style::default().fg(Color::White))])
            }).collect();

            let tabs = Tabs::new(spans)
                .block(Block::default().borders(Borders::BOTTOM).title("Main Tabs"))
                .style(Style::default().bg(Color::Black).fg(Color::White))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
                .divider(Span::raw(" | "))
                .select(selected_tab);

            f.render_widget(tabs, layout_chunks[0]);

            // Reset list state when changing tabs
            if selected_tab != previous_tab {
                reset_list_state_for_tab(selected_tab, &mut list_state, guide.len(), movies.len(), selected_series_tab);
            }

            previous_tab = selected_tab;

            // Handle Episodes Tab
            if selected_tab == 0 {
                let layout_with_series_tabs = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),  // Main tabs
                        Constraint::Length(3),  // Series tabs
                        Constraint::Min(1),     // Content
                    ].as_ref())
                    .split(size);

                let series_names: Vec<String> = guide.iter()
                    .map(|series| series.series.clone())
                    .collect();

                let series_tabs: Vec<Spans> = series_names.iter().map(|name| {
                    Spans::from(vec![Span::styled(name, Style::default().fg(Color::White))])
                }).collect();

                let series_tabs_widget = Tabs::new(series_tabs)
                    .block(Block::default().borders(Borders::BOTTOM).title("Series Tabs"))
                    .style(Style::default().bg(Color::Black).fg(Color::White))
                    .highlight_style(Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD))
                    .divider(Span::raw(" | "))
                    .select(selected_series_tab);

                f.render_widget(series_tabs_widget, layout_with_series_tabs[1]);

                match app_mode {
                    AppMode::EpisodesSeries(series_index) => {
                        if let Some(series) = guide.get(series_index) {
                            let items: Vec<_> = series.episodes.iter()
                                .map(|ep| ListItem::new(format!(
                                    "{}: {}",
                                    ep.episode_number,
                                    ep.title
                                )))
                                .collect();
                            let list = List::new(items)
                                .block(Block::default().borders(Borders::ALL).title("Episodes"))
                                .highlight_style(Style::default().bg(Color::Yellow));
                            f.render_stateful_widget(list, layout_with_series_tabs[2], &mut list_state);
                        }
                    }
                    AppMode::Details(series_index, episode_index) => {
                        if let Some(series) = guide.get(series_index) {
                            if let Some(episode) = series.episodes.get(episode_index) {
                                let block = Block::default()
                                    .borders(Borders::ALL)
                                    .title(format!("Episode Details: {}", episode.title));
                                let details = format!(
                                    "Episode Number: {}\nDescription: {}\nRelease Date: {}\nDuration: {}\nSaga: {}",
                                    episode.episode_number,
                                    episode.description,
                                    episode.release_date,
                                    episode.duration,
                                    episode.saga
                                );
                                let paragraph = Paragraph::new(details)
                                    .block(block)
                                    .wrap(tui::widgets::Wrap { trim: true });
                                f.render_widget(paragraph, layout_with_series_tabs[2]);
                            }
                        }
                    }
                    _ => {}
                }
            } else if selected_tab == 1 {
                // Display Movies List
                let layout_with_movies = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),  // Main tabs
                        Constraint::Min(1),     // Content
                    ].as_ref())
                    .split(size);

                match app_mode {
                    AppMode::MoviesList => {
                        let movie_items: Vec<_> = movies.iter()
                            .map(|movie| ListItem::new(format!(
                                "{}: {} ",
                                movie.number,
                                movie.title,
                            )))
                            .collect();
                        
                        let movies_list = List::new(movie_items)
                            .block(Block::default().borders(Borders::ALL).title("Movies"))
                            .highlight_style(Style::default().bg(Color::Yellow));

                        f.render_stateful_widget(movies_list, layout_with_movies[1], &mut list_state);
                    }
                    AppMode::MovieDetails(movie_index) => {
                        if let Some(movie) = movies.get(movie_index) {
                            let block = Block::default()
                                .borders(Borders::ALL)
                                .title(format!("Movie Details: {}", movie.title));
                            let details = format!(
                                "Number: {}\nRelease Date: {}\nRuntime: {}\nDescription: {}Director: {}\nGenres: {}\nTrivia: {}\nPlot Keywords: {}",
                                movie.number,
                                movie.release_date,
                                movie.runtime,
                                movie.description,
                                movie.director,
                                movie.genres.join(", "),
                                movie.trivia,
                                movie.plot_keywords.join(", ")
                            );
                            let paragraph = Paragraph::new(details)
                                .block(block)
                                .wrap(tui::widgets::Wrap { trim: true });
                            f.render_widget(paragraph, layout_with_movies[1]);
                        }
                    }
                    _ => {}
                }
            } else if selected_tab == 2 {
                // Display Characters Tab
                let layout_with_characters = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),  // Main tabs
                        Constraint::Min(1),     // Content
                    ].as_ref())
                    .split(size);

                let characters_text = match app_mode {
                    AppMode::Characters => "Characters details go here.",
                    _ => "",
                };

                let paragraph = Paragraph::new(characters_text)
                    .block(Block::default().borders(Borders::ALL).title("Characters"));
                f.render_widget(paragraph, layout_with_characters[1]);
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Tab => {
                    selected_tab = (selected_tab + 1) % 3;
                    app_mode = match selected_tab {
                        0 => AppMode::EpisodesSeries(selected_series_tab),
                        1 => AppMode::MoviesList,
                        2 => AppMode::Characters,
                        _ => app_mode,
                    };
                    reset_list_state_for_tab(selected_tab, &mut list_state, guide.len(), movies.len(), selected_series_tab);
                }
                KeyCode::Left => {
                    if selected_tab == 0 {
                        selected_series_tab = if selected_series_tab > 0 {
                            selected_series_tab - 1
                        } else {
                            guide.len().saturating_sub(1)
                        };
                        app_mode = AppMode::EpisodesSeries(selected_series_tab);
                        reset_list_state_for_tab(selected_tab, &mut list_state, guide.len(), movies.len(), selected_series_tab);
                    }
                }
                KeyCode::Right => {
                    if selected_tab == 0 {
                        selected_series_tab = (selected_series_tab + 1) % guide.len();
                        app_mode = AppMode::EpisodesSeries(selected_series_tab);
                        reset_list_state_for_tab(selected_tab, &mut list_state, guide.len(), movies.len(), selected_series_tab);
                    }
                }
                KeyCode::Down => {
                    if let Some(selected) = list_state.selected() {
                        let count = match selected_tab {
                            0 => guide[selected_series_tab].episodes.len(),
                            1 => movies.len(),
                            _ => 0,
                        };
                        if selected < count - 1 {
                            list_state.select(Some(selected + 1));
                        }
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = list_state.selected() {
                        if selected > 0 {
                            list_state.select(Some(selected - 1));
                        }
                    }
                }
                KeyCode::Esc => {
                    if let AppMode::Details(_, _) = app_mode {
                        app_mode = AppMode::EpisodesSeries(selected_series_tab);
                    } else if let AppMode::MovieDetails(_) = app_mode {
                        app_mode = AppMode::MoviesList;
                    }
                }
                KeyCode::Enter => {
                    match app_mode {
                        AppMode::EpisodesSeries(series_index) => {
                            if let Some(episode_index) = list_state.selected() {
                                app_mode = AppMode::Details(series_index, episode_index);
                            }
                        }
                        AppMode::MoviesList => {
                            if let Some(movie_index) = list_state.selected() {
                                app_mode = AppMode::MovieDetails(movie_index);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(crossterm::cursor::Show)?;

    Ok(())
}
