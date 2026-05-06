mod app;
mod config;
mod data;
mod handlers;
mod pagination;
mod search;
mod ui;

use crossterm::{
    event::{self, Event},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io;
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use config::Config;
use handlers::handle_key_event;
use ui::draw_ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(crossterm::cursor::Hide)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load configuration
    let config = match Config::load_from_file("config.json") {
        Ok(config) => config,
        Err(_) => {
            let default_config = Config::default();
            default_config.save_to_file("config.json")?;
            default_config
        }
    };

    // Initialize app state with configuration
    let mut app = App::new_with_config(config)?;

    // Main loop
    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if !handle_key_event(key, &mut app)? {
                break;
            }
        }
    }

    // Cleanup
    terminal::disable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(crossterm::cursor::Show)?;

    Ok(())
}
