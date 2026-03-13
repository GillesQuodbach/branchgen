use ratatui::{Frame, layout::{Constraint, Direction, Layout}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph}, border};
use ratatui::style::{Color, Style};

use ratatui::text::ToSpan;
use ratatui::widgets::Widget;
use crate::app::AppState;
use crate::domain::field::Field;

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

    // block PI
    let pi_block = Block::bordered()
        .title(" PI ")
        .border_style(field_style(state, Field::Pi));
    let pi_inner = pi_block.inner(left_chunks[0]);

    let pi_text = state.work_item_input.pi.map(|v| format!("{v}")).unwrap_or_else(|| "Not set".to_string());
    Paragraph::new(pi_text).render(pi_inner, frame.buffer_mut());

    // block it
    let it_block = Block::bordered()
        .title(" IT ")
        .border_style(field_style(state, Field::It));
    let it_inner = it_block.inner(left_chunks[1]);

    let it_text = state.work_item_input.it.map(|v| format!("{v}")).unwrap_or_else(|| "Not set".to_string());
    Paragraph::new(it_text).render(it_inner, frame.buffer_mut());

    let story_type_block = Block::bordered()
        .title(" Story type ")
        .border_style(field_style(state, Field::StoryType));

    let commit_type_block = Block::bordered()
        .title(" Commit type ")
        .border_style(field_style(state, Field::CommitType));

    let story_number_block = Block::bordered()
        .title(" Story number ")
        .border_style(field_style(state, Field::StoryNumber));

    let story_title = Block::bordered()
        .title(" Story title ")
        .border_style(field_style(state, Field::StoryTitle));

    let commit_message_block = Block::bordered()
        .title(" Commit message ")
        .border_style(field_style(state, Field::CommitMessage));

    let github_block = Block::bordered()
        .title(" GitHub ")
        .border_style(field_style(state, Field::Github));

    let history_block = Block::bordered()
        .title(" History ")
        .border_style(field_style(state, Field::History));

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

// modification du style a la selection
fn field_style(state: &AppState, field: Field) -> Style {
    if state.selected_field == field {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::White)
    }
}