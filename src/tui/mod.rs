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
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(module, target, output);

    loop {
        terminal.draw(|f| ui::render(f, &mut app))?;

        if app.wants_doctor {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            let report = crate::doctor::run_check();
            crate::doctor::print_report(&report);

            println!("\x1b[1;33mPress Enter to return to penkit...\x1b[0m");
            let mut dummy = String::new();
            std::io::stdin().read_line(&mut dummy)?;

            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            let backend = CrosstermBackend::new(stdout);
            terminal = Terminal::new(backend)?;

            app.wants_doctor = false;
            continue;
        }

        if app.wants_run {
            if let Some(ref cmd) = app.final_command {
                let exec_cmd = if app.sudo_mode {
                    format!("sudo {}", cmd)
                } else {
                    cmd.to_string()
                };

                disable_raw_mode()?;
                execute!(
                    terminal.backend_mut(),
                    LeaveAlternateScreen,
                    DisableMouseCapture
                )?;
                terminal.show_cursor()?;

                println!("\n\x1b[1;35m[penkit]\x1b[0m Executing:\n");
                println!("\x1b[1;32m{}\x1b[0m\n", exec_cmd);

                let status = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&exec_cmd)
                    .status();

                match status {
                    Ok(s) => println!(
                        "\n\x1b[1;35m[penkit]\x1b[0m Exit code: {}\n",
                        s.code().map(|c| c.to_string()).unwrap_or_else(|| "signal".into())
                    ),
                    Err(e) => println!(
                        "\n\x1b[1;31m[penkit]\x1b[0m Failed to execute: {}\n",
                        e
                    ),
                }

                println!("\x1b[1;33mPress Enter to return to penkit...\x1b[0m");
                let mut dummy = String::new();
                std::io::stdin().read_line(&mut dummy)?;

                enable_raw_mode()?;
                let mut stdout = io::stdout();
                execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
                let backend = CrosstermBackend::new(stdout);
                terminal = Terminal::new(backend)?;
            }
            app.wants_run = false;
            app.sudo_mode = false;
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if input::handle_key(&mut app, key) {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

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
