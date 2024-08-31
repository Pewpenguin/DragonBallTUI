mod ui;
mod app;
mod handlers;
mod data;

use std::io;
use crossterm::{
    event::{self, Event},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};

use app::App;
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

    // Initialize app state
    let mut app = App::new()?;

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