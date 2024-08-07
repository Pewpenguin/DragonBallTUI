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
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use data::{load_guide_from_file, save_guide_to_file, Series, Episode};

enum AppMode {
    List,
    Details(usize), // Store the index of the selected episode
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

    // Main loop
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                .split(size);

            match app_mode {
                AppMode::List => {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title("Dragon Ball Episode Guide");
                    f.render_widget(block, chunks[0]);

                    let items: Vec<_> = guide
                        .iter()
                        .flat_map(|series| &series.episodes)
                        .map(|ep| {
                            ListItem::new(format!(
                                "{}: {}", // Fixed format string
                                ep.episode_number,
                                ep.title
                            ))
                        })
                        .collect();
                    let list = List::new(items)
                        .block(Block::default().borders(Borders::ALL).title("Episodes"))
                        .highlight_style(Style::default().bg(Color::Yellow));
                    f.render_stateful_widget(list, chunks[1], &mut list_state);
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
                        f.render_widget(paragraph, chunks[1]);
                    }
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Enter => {
                    if let AppMode::List = app_mode {
                        if let Some(selected) = list_state.selected() {
                            app_mode = AppMode::Details(selected);
                        }
                    } else if let AppMode::Details(_) = app_mode {
                        app_mode = AppMode::List;
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
