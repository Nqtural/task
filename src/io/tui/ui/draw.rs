use anyhow::Result;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
};

use super::super::prompt::Prompt;
use super::super::selection::Level;
use super::super::state::State;
use super::Ui;
use super::style::UiStyle;

impl Ui {
    pub fn draw(&mut self, state: &State) -> Result<()> {
        self.terminal.draw(|frame| {
            let area = frame.area();

            let main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(area);

            frame.render_widget(
                statusbar(&state.selection.level, area.width),
                main_layout[1],
            );

            let panes = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_layout[0]);

            let (projetc_table, mut project_state) = projects(state, panes[0].width);
            frame.render_stateful_widget(projetc_table, panes[0], &mut project_state);
            match tasks(state, panes[1].width) {
                Some((task_table, mut task_state)) => {
                    frame.render_stateful_widget(task_table, panes[1], &mut task_state);
                }
                None => frame.render_widget(no_project_selected(), panes[1]),
            }

            match &state.prompt {
                Prompt::Confirm(message) => render_popup(
                    frame,
                    area,
                    vec![
                        Line::from(*message),
                        Line::from("Press (y) to confirm, (n) to cancel"),
                    ],
                ),
                Prompt::Text(text) => render_popup(
                    frame,
                    area,
                    vec![
                        Line::from("Enter text (ESC to cancel, Enter to submit):"),
                        Line::from(text.iter().collect::<String>()),
                    ],
                ),
                Prompt::None => {}
            }
        })?;

        Ok(())
    }
}

fn render_popup(frame: &mut Frame, area: Rect, text: Vec<Line>) {
    let popup_area = centered_rect(20, 60, area);

    // Clear background
    frame.render_widget(Clear, popup_area);

    let block = Block::default().title("Prompt").borders(Borders::ALL);

    let text = Paragraph::new(text).block(block);

    frame.render_widget(text, popup_area);
}

fn centered_rect(percent_y: u16, percent_x: u16, r: Rect) -> Rect {
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

fn statusbar(level: &'_ Level, width: u16) -> Paragraph<'_> {
    let text = match level {
        Level::Project => "HJKL: Movement | Q: quit | D: delete",
        Level::Task => {
            "HJKL: Movement | Q: quit | D: delete | F: finish | N: edit name | E: edit expiration"
        }
    };
    Paragraph::new(truncate(text, width as usize, false))
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
}

fn truncate(text: &str, width: usize, truncate_start: bool) -> String {
    let len = text.chars().count();

    if len <= width {
        return text.to_string();
    }

    if width <= 3 {
        return ".".repeat(width);
    }

    let visible = width - 3;

    if truncate_start {
        // keep the end
        let tail: String = text.chars().skip(len - visible).collect();
        format!("...{tail}")
    } else {
        // keep the start
        let head: String = text.chars().take(visible).collect();
        format!("{head}...")
    }
}

fn projects(state: &State, pane_width: u16) -> (Table<'_>, TableState) {
    let mut table_state = TableState::default();
    table_state.select(state.selection.project);

    let path_width = pane_width
        .saturating_sub(5) // task count column
        .saturating_sub(3) // highlight_symbol
        .saturating_sub(2) // borders
        .saturating_sub(1); // spacing

    let projects = state.projects.borrow();
    let rows = projects.iter().map(|project| {
        let display_path = truncate(&project.path, path_width as usize, true);

        Row::new(vec![
            Cell::from(display_path),
            Cell::new(Line::from(project.tasks.len().to_string()).alignment(Alignment::Right)),
        ])
    });

    let style = UiStyle::new(matches!(state.selection.level, Level::Project));
    let table = Table::new(rows, [Constraint::Min(4), Constraint::Length(5)])
        .header(
            Row::new(vec![
                Line::from("Path"),
                Line::from("Tasks").alignment(Alignment::Right),
            ])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .title("Projects")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(style.border_color)),
        )
        .row_highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(style.selected_background)
                .fg(style.selected_foreground),
        )
        .highlight_symbol(style.highlight_symbol);

    (table, table_state)
}

fn tasks(state: &State, pane_width: u16) -> Option<(Table<'_>, TableState)> {
    let mut table_state = TableState::default();
    table_state.select(state.selection.task);

    let task_width = pane_width
        .saturating_sub(15) // status column
        .saturating_sub(3) // highlight_symbol
        .saturating_sub(2) // borders
        .saturating_sub(1); // spacing

    let projects = state.projects.borrow();
    let rows = projects
        .get(state.selection.project?)?
        .tasks
        .iter()
        .map(|task| {
            let display_task = truncate(&task.name, task_width as usize, false);

            Row::new(vec![
                Cell::from(display_task),
                Cell::new(
                    Line::from(if task.finished {
                        String::from("DONE")
                    } else {
                        match task.expiration {
                            Some(expiration) => expiration.format_relative(),
                            None => String::new(),
                        }
                    })
                    .alignment(Alignment::Right),
                ),
            ])
        });

    let style = UiStyle::new(matches!(state.selection.level, Level::Task));
    let table = Table::new(rows, [Constraint::Min(4), Constraint::Length(15)])
        .header(
            Row::new(vec![
                Line::from("Task"),
                Line::from("Status").alignment(Alignment::Right),
            ])
            .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .block(
            Block::default()
                .title("Tasks")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(style.border_color)),
        )
        .row_highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(style.selected_background)
                .fg(style.selected_foreground),
        )
        .highlight_symbol(style.highlight_symbol);

    Some((table, table_state))
}

fn no_project_selected() -> Paragraph<'static> {
    Paragraph::new("No project selected")
}
