mod app;
mod ui;
mod input;

pub use app::App;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

pub async fn run(
    module: Option<String>,
    target: Option<String>,
    output: Option<String>,
) -> anyhow::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(module, target, output);

    // Main event loop
    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if input::handle_key(&mut app, key) {
                break; // app requested quit
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // If a command was generated, print it to stdout
    if let Some(cmd) = &app.final_command {
        println!("\n\x1b[1;35m[penkit]\x1b[0m Generated command:\n");
        println!("\x1b[1;32m{}\x1b[0m\n", cmd);

        if let Some(path) = &app.output_file {
            std::fs::write(path, cmd)?;
            println!("\x1b[1;34m[saved to {}]\x1b[0m\n", path);
        }
    }

    Ok(())
}
