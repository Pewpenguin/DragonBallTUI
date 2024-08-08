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

// Ensure these imports match your `data.rs` file
use data::{load_guide_from_file, save_guide_to_file, Series, Episode};

enum AppMode {
    Characters,
    Movies,
    Details(usize, usize), // (series_index, episode_index)
    EpisodesSeries(usize), // series_index
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let guide_path = "episodes.json";

    // Check if the file exists, create a default one if not
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

    // Load data
    let guide = load_guide_from_file(guide_path)?;

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

            // Adjust layout if on Episodes tab
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
                    AppMode::Details(series_index, _episode_index) => {
                        if let Some(series) = guide.get(series_index) {
                            if let Some(episode) = series.episodes.get(_episode_index) {
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
            } else {
                match app_mode {
                    AppMode::Characters => {
                        let block = Block::default()
                            .borders(Borders::ALL)
                            .title("Characters");
                        let characters_content = "Characters content goes here.";
                        let paragraph = Paragraph::new(characters_content)
                            .block(block)
                            .wrap(tui::widgets::Wrap { trim: true });
                        f.render_widget(paragraph, layout_chunks[1]);
                    }
                    AppMode::Movies => {
                        let block = Block::default()
                            .borders(Borders::ALL)
                            .title("Movies");
                        let movies_content = "Movies content goes here.";
                        let paragraph = Paragraph::new(movies_content)
                            .block(block)
                            .wrap(tui::widgets::Wrap { trim: true });
                        f.render_widget(paragraph, layout_chunks[1]);
                    }
                    _ => {}
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Tab => {
                    selected_tab = (selected_tab + 1) % 3;
                    app_mode = match selected_tab {
                        0 => AppMode::EpisodesSeries(selected_series_tab),
                        1 => AppMode::Movies,
                        2 => AppMode::Characters,
                        _ => app_mode,
                    };

                    if selected_tab == 0 {
                        selected_series_tab = 0;
                        list_state.select(Some(0));
                    } else {
                        selected_series_tab = 0;
                    }
                }
                KeyCode::Enter => {
                    match app_mode {
                        AppMode::Details(series_index, _episode_index) => {
                            // Move back to the EpisodesSeries view with the current series tab
                            app_mode = AppMode::EpisodesSeries(series_index);
                        }
                        AppMode::EpisodesSeries(series_index) => {
                            if let Some(selected) = list_state.selected() {
                                app_mode = AppMode::Details(series_index, selected);
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Down => {
                    match app_mode {
                        AppMode::EpisodesSeries(series_index) => {
                            if let Some(selected) = list_state.selected() {
                                if let Some(series) = guide.get(series_index) {
                                    let max_index = series.episodes.len();
                                    if selected + 1 < max_index {
                                        list_state.select(Some(selected + 1));
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                KeyCode::Up => {
                    match app_mode {
                        AppMode::EpisodesSeries(_) => {
                            let prev = list_state.selected().map_or(0, |i| i.saturating_sub(1));
                            list_state.select(Some(prev));
                        }
                        _ => {}
                    }
                }
                KeyCode::Right => {
                    if selected_tab == 0 {
                        let max_index = guide.len();
                        if selected_series_tab < max_index - 1 {
                            selected_series_tab += 1;
                            list_state.select(Some(0));
                            app_mode = AppMode::EpisodesSeries(selected_series_tab);
                        }
                    }
                }
                KeyCode::Left => {
                    if selected_tab == 0 {
                        if selected_series_tab > 0 {
                            selected_series_tab -= 1;
                            list_state.select(Some(0));
                            app_mode = AppMode::EpisodesSeries(selected_series_tab);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(crossterm::cursor::Show)?;

    Ok(())
}
