use std::io;
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

mod data;
use data::{DragonBallGuide, Episode};

enum AppMode {
    List,
    Details(usize), // Store the index of the selected episode
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(crossterm::cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Sample data
    let  guide = DragonBallGuide {
        episodes: vec![
            Episode {
                id: 1,
                title: "The Beginning".to_string(),
                summary: "Goku's adventure begins.".to_string(),
                key_events: vec!["Goku meets Bulma".to_string()],
                character_appearances: vec!["Goku".to_string(), "Bulma".to_string()],
                user_rating: 4.5,
                watched: false,
            },
            Episode {
                id: 2,
                title: "The Search Begins".to_string(),
                summary: "Goku and Bulma start their journey.".to_string(),
                key_events: vec!["Meeting Yamcha".to_string()],
                character_appearances: vec!["Goku".to_string(), "Bulma".to_string(), "Yamcha".to_string()],
                user_rating: 4.7,
                watched: false,
            },
        ],
        movies: vec![], // Add movie data if needed
    };

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
                        .episodes
                        .iter()
                        .map(|ep| {
                            ListItem::new(format!(
                                "{}{}: {}",
                                if ep.watched { "[x] " } else { "[ ] " },
                                ep.id,
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
                    if let Some(episode) = guide.episodes.get(index) {
                        let block = Block::default()
                            .borders(Borders::ALL)
                            .title(format!("Episode Details: {}", episode.title));
                        let details = format!(
                            "ID: {}\nSummary: {}\nKey Events: {}\nCharacter Appearances: {}\nRating: {}\nWatched: {}",
                            episode.id,
                            episode.summary,
                            episode.key_events.join(", "),
                            episode.character_appearances.join(", "),
                            episode.user_rating,
                            if episode.watched { "Yes" } else { "No" }
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
                    if next < guide.episodes.len() {
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
