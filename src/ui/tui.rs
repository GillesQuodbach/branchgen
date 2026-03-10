use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph}, border};
use ratatui::style::{Color, Style};

use ratatui::text::ToSpan;
use ratatui::widgets::Widget;
use crate::app::AppState;

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    // titre de l'equipe
    let title = format!(" APS BranchGen - Team {} ", state.team_name);

    // block principale
    let main_block = Block::bordered().fg(Color::Blue).title(title.to_span().into_centered_line());

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
        .constraints([Constraint::Length(6), Constraint::Fill(1)]).split(horizontal_chunks[1]);

    // panneau gauche
    let left_main_block = Block::bordered()
        .title(" Infos ")
        .border_style(Style::default().fg(Color::Cyan));

    let left_main_block_inner = left_main_block.inner(horizontal_chunks[0]);

    left_main_block.render(horizontal_chunks[0], frame.buffer_mut());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ]).split(left_main_block_inner);


    let pi_block = Block::bordered()
        .title(" PI ")
        .border_style(Style::default().fg(Color::Green));

    let it_block = Block::bordered()
        .title(" IT ")
        .border_style(Style::default().fg(Color::Green));

    let story_type_block = Block::bordered()
        .title(" Story type ")
        .border_style(Style::default().fg(Color::Green));

    let commit_type_block = Block::bordered()
        .title(" Commit type ")
        .border_style(Style::default().fg(Color::Green));

    let story_number_block = Block::bordered()
        .title(" Story number ")
        .border_style(Style::default().fg(Color::Green));

    let story_title = Block::bordered()
        .title(" Story title ")
        .border_style(Style::default().fg(Color::Green));

    let commit_message_block = Block::bordered()
        .title(" Commit message ")
        .border_style(Style::default().fg(Color::Green));

    let github_block = Block::bordered()
        .title(" GitHub ")
        .border_style(Style::default().fg(Color::Green));

    let history_block = Block::bordered()
        .title(" History ")
        .border_style(Style::default().fg(Color::Yellow));

    pi_block.render(left_chunks[0], frame.buffer_mut());
    it_block.render(left_chunks[1], frame.buffer_mut());
    story_type_block.render(left_chunks[2], frame.buffer_mut());
    commit_type_block.render(left_chunks[3], frame.buffer_mut());
    story_number_block.render(left_chunks[4], frame.buffer_mut());
    story_title.render(left_chunks[5], frame.buffer_mut());
    commit_message_block.render(left_chunks[6], frame.buffer_mut());
    github_block.render(right_vertical_chunks[0], frame.buffer_mut());
    history_block.render(right_vertical_chunks[1], frame.buffer_mut());


}