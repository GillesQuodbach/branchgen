use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};

use crate::app::AppState;

pub fn render(frame: &mut Frame, state: &AppState) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    // --- Main panel
    let title = Line::from(" APS BranchGen ".bold());
    let instructions = Line::from(vec![
        " Decrement ".into(),
        "<Left>".blue().bold(),
        " Increment ".into(),
        "<Right>".blue().bold(),
        " Quit ".into(),
        "<q>".blue().bold(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        state.counter.to_string().yellow(),
    ])]);

    let main = Paragraph::new(counter_text).centered().block(block);
    frame.render_widget(main, layout[0]);

    // --- Footer/status
    let footer = Paragraph::new(state.status.as_str().dim());
    frame.render_widget(footer, layout[1]);
}