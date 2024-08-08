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
    List,
    Characters,
    Movies,
    Details(usize), // Store the index of the selected episode
    EpisodesSeries(usize), // Store the index of the selected series
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

    let mut app_mode = AppMode::List;
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
                    Constraint::Length(3),  // Series tabs
                    Constraint::Min(1),     // Content
                ].as_ref())
                .split(size);

            // Main Tabs header
            let tab_titles = ["Episodes", "Characters", "Movies"];
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

            // Render Series Tabs only if on Episodes tab
            if selected_tab == 0 {
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

                f.render_widget(series_tabs_widget, layout_chunks[1]);
            }

            // Render Content based on AppMode
            match app_mode {
                AppMode::List => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Dragon Ball Episode Guide");
                    f.render_widget(block, layout_chunks[2]);

                    let items: Vec<_> = guide
                        .iter()
                        .flat_map(|series| &series.episodes)
                        .map(|ep| {
                            ListItem::new(format!(
                                "{}: {}",
                                ep.episode_number,
                                ep.title
                            ))
                        })
                        .collect();
                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Episodes"))
                        .highlight_style(Style::default().bg(Color::Yellow));
                    f.render_stateful_widget(list, layout_chunks[2], &mut list_state);
                }
                AppMode::Characters => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Characters");
                    let characters_content = "Characters content goes here.";
                    let paragraph = Paragraph::new(characters_content)
                        .block(block)
                        .wrap(tui::widgets::Wrap { trim: true });
                    f.render_widget(paragraph, layout_chunks[2]);
                }
                AppMode::Movies => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Movies");
                    let movies_content = "Movies content goes here.";
                    let paragraph = Paragraph::new(movies_content)
                        .block(block)
                        .wrap(tui::widgets::Wrap { trim: true });
                    f.render_widget(paragraph, layout_chunks[2]);
                }
                AppMode::Details(index) => {
                    let episode = guide.iter()
                        .flat_map(|series| &series.episodes)
                        .nth(index);

                    if let Some(episode) = episode {
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
                        f.render_widget(paragraph, layout_chunks[2]);
                    }
                }
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
                        f.render_stateful_widget(list, layout_chunks[2], &mut list_state);
                    }
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Tab => {
                    selected_tab = (selected_tab + 1) % 3;
                    app_mode = match selected_tab {
                        0 => {
                            // Keep the episode details if switching back to Episodes tab
                            if let AppMode::Details(selected) = app_mode {
                                AppMode::Details(selected)
                            } else {
                                AppMode::List
                            }
                        }
                        1 => AppMode::Characters,
                        2 => AppMode::Movies,
                        _ => app_mode,
                    };

                    if selected_tab == 0 {
                        // Reset the selected series tab and the list state when switching to Episodes tab
                        selected_series_tab = 0;
                        list_state.select(Some(0));
                    } else {
                        // Hide series tabs if not on Episodes tab
                        selected_series_tab = 0;
                    }
                }
                KeyCode::Char('c') => { // Use 'c' for confirming actions in series tabs
                    // Example action for 'c' - change this to your needs
                    if selected_tab == 0 {
                        app_mode = AppMode::EpisodesSeries(selected_series_tab);
                    }
                }
                KeyCode::Enter => { // Use Enter to open episode details
                    match app_mode {
                        AppMode::List => {
                            if let Some(selected) = list_state.selected() {
                                app_mode = AppMode::Details(selected);
                            }
                        }
                        AppMode::Details(_) => app_mode = AppMode::List,
                        AppMode::EpisodesSeries(series_index) => {
                            // Stay in EpisodesSeries mode when Enter is pressed
                            app_mode = AppMode::EpisodesSeries(series_index);
                        }
                        _ => {}
                    }
                }
                KeyCode::Down => {
                    let next = list_state.selected().map_or(0, |i| i + 1);
                    let max_index = guide.iter()
                        .flat_map(|series| &series.episodes)
                        .count();
                    if next < max_index {
                        list_state.select(Some(next));
                    }
                }
                KeyCode::Up => {
                    let prev = list_state.selected().map_or(0, |i| i.saturating_sub(1));
                    list_state.select(Some(prev));
                }
                KeyCode::Right => {
                    if selected_tab == 0 {
                        let max_index = guide.len();
                        if selected_series_tab < max_index - 1 {
                            selected_series_tab += 1;
                            // Reset the list state to the top when switching series tabs
                            list_state.select(Some(0));
                            app_mode = AppMode::EpisodesSeries(selected_series_tab);
                        }
                    }
                }
                KeyCode::Left => {
                    if selected_tab == 0 {
                        if selected_series_tab > 0 {
                            selected_series_tab -= 1;
                            // Reset the list state to the top when switching series tabs
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