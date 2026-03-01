use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph}, border};
use ratatui::style::{Color, Style};

use ratatui::text::ToSpan;
use ratatui::widgets::Widget;
use crate::app::AppState;

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    // block principale
    let main_block = Block::bordered().fg(Color::Blue).title(" APS Branch Gen ".to_span().into_centered_line());

    // calcule de la zone
    let inner_area = main_block.inner(area);

    // on dessine le block
    main_block.render(area, frame.buffer_mut());

    // decoupe du block princpal
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [Constraint::Percentage(25),
            Constraint::Percentage(75),]
        )
        .split(inner_area);

    // decoupe colone de droite
    let right_vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Fill(1)]).split(horizontal_chunks[1]);

    // panneau gauche
    Block::bordered()
        .title(" Infos ")
        .border_style(Style::default().fg(Color::Cyan))
        .render(horizontal_chunks[0], frame.buffer_mut());

    // panneau git
    Block::bordered()
        .title(" GitHub ")
        .border_style(Style::default().fg(Color::Green))
        .render(right_vertical_chunks[0], frame.buffer_mut());

    // panneau history
    Block::bordered()
        .title(" History ")
        .border_style(Style::default().fg(Color::Yellow))
        .render(right_vertical_chunks[1], frame.buffer_mut());


}