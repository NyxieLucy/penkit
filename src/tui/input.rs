use crossterm::event::{KeyCode, KeyEvent};

use super::app::App;

pub fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    if app.input_mode {
        match key.code {
            KeyCode::Enter => app.submit_input(),
            KeyCode::Esc => app.input_mode = false,
            KeyCode::Char(c) => app.input_buffer.push(c),
            KeyCode::Backspace => {
                app.input_buffer.pop();
            }
            _ => {}
        }
        return false;
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => return true,
        KeyCode::Up => app.prev_command(),
        KeyCode::Down => app.next_command(),
        KeyCode::Left => app.prev_category(),
        KeyCode::Right => app.next_category(),
        KeyCode::Enter => app.start_input(),
        KeyCode::Char('r') | KeyCode::Char('R') => {
            if app.final_command.is_some() {
                app.wants_run = true;
            }
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.sudo_mode = !app.sudo_mode;
        }
        _ => {}
    }
    false
}
