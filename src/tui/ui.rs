use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use super::app::App;

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let header = Paragraph::new("🗡️ penkit — lazy hacker's swiss knife")
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[1]);

    let categories: Vec<ListItem> = app
        .categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let style = if i == app.selected_category {
                Style::default()
                    .bg(Color::Magenta)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(cat.label()).style(style)
        })
        .collect();
    let cat_list = List::new(categories)
        .block(Block::default().title("Categories").borders(Borders::ALL));
    f.render_widget(cat_list, main_chunks[0]);

    let commands: Vec<ListItem> = app
        .commands
        .iter()
        .enumerate()
        .map(|(i, cmd)| {
            let style = if i == app.selected_command {
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(format!("{} — {}", cmd.name, cmd.description)).style(style)
        })
        .collect();
    let cmd_list = List::new(commands)
        .block(Block::default().title("Commands").borders(Borders::ALL));
    f.render_widget(cmd_list, main_chunks[1]);

    let sudo_tag = if app.sudo_mode { " [sudo ON]" } else { "" };
    let footer_text = if app.input_mode {
        format!("Input: {} > {}", app.input_label, app.input_buffer)
    } else if app.final_command.is_some() {
        format!(
            "↑↓: Navigate | Enter: Params | r: Run{} | s: Toggle sudo | ←→: Categories | q: Quit",
            sudo_tag
        )
    } else {
        format!(
            "↑↓: Navigate | Enter: Select{} | s: Toggle sudo | ←→: Categories | q: Quit",
            sudo_tag
        )
    };
    let footer = Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);

    if app.input_mode {
        let popup_area = centered_rect(60, 20, f.area());
        f.render_widget(Clear, popup_area);
        let input_block = Paragraph::new(app.input_buffer.clone())
            .block(Block::default().title("Input").borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input_block, popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
