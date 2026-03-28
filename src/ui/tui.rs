use ratatui::{Frame, layout::{Constraint, Direction, Layout, Alignment}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph}, border};
use ratatui::style::{Color, Style};

use ratatui::text::ToSpan;
use ratatui::widgets::Widget;
use crate::app::AppState;
use crate::app::input_mode::InputMode;
use crate::domain::field::Field;

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    let mode_text = match state.input_mode {
        InputMode::Navigation => "Navigation",
        InputMode::Edition => "Edition",
    };

    // titre de l'equipe
    let title = format!(" APS BranchGen - Team {} [{}]", state.team_name, mode_text);

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
        .constraints([Constraint::Length(6),Constraint::Fill(1), Constraint::Length(3)])
        .split(horizontal_chunks[1]);

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
            Constraint::Length(3),
        ]).split(left_main_block_inner);

    // block PI
    let pi_block = Block::bordered()
        .title(" PI ")
        .border_style(field_style(state, Field::Pi));
    let pi_inner = pi_block.inner(left_chunks[0]);
    let pi_text = state.work_item_input.pi.map(|v| format!("{v}")).unwrap_or_else(|| "".to_string());
    pi_block.render(left_chunks[0], frame.buffer_mut());
    Paragraph::new(pi_text).render(pi_inner, frame.buffer_mut());

    // block it
    let it_block = Block::bordered()
        .title(" IT ")
        .border_style(field_style(state, Field::It));
    let it_inner = it_block.inner(left_chunks[1]);
    let it_text = state.work_item_input.it.map(|v| format!("{v:02}")).unwrap_or_else(|| "".to_string());
    it_block.render(left_chunks[1], frame.buffer_mut());
    Paragraph::new(it_text).render(it_inner, frame.buffer_mut());

    let story_type_block = Block::bordered()
        .title(" Story type ")
        .border_style(field_style(state, Field::StoryType));
    let story_type_inner = story_type_block.inner(left_chunks[2]);
    let story_type_text = state.work_item_input.story_type.as_ref().map(|v| format!("{v}")).unwrap_or_else(|| "".to_string());
    story_type_block.render(left_chunks[2], frame.buffer_mut());
    Paragraph::new(story_type_text).render(story_type_inner, frame.buffer_mut());



    let commit_type_block = Block::bordered()
        .title(" Commit type ")
        .border_style(field_style(state, Field::CommitType));
    let commit_type_inner = commit_type_block.inner(left_chunks[3]);
    let commit_type_text = state.work_item_input.commit_type.as_ref().map(|v| format!("{v}")).unwrap_or_else(|| "".to_string());
    commit_type_block.render(left_chunks[3], frame.buffer_mut());
    Paragraph::new(commit_type_text).render(commit_type_inner, frame.buffer_mut());

    let story_number_block = Block::bordered()
        .title(" Story number ")
        .border_style(field_style(state, Field::StoryNumber));
    let story_number_inner = story_number_block.inner(left_chunks[4]);
    let story_number_text = state.work_item_input.story_number.as_deref().unwrap_or("");
    story_number_block.render(left_chunks[4], frame.buffer_mut());
    Paragraph::new(story_number_text).render(story_number_inner, frame.buffer_mut());


    let story_title_block = Block::bordered()
        .title(" Story title ")
        .border_style(field_style(state, Field::StoryTitle));
    let story_title_text = state.work_item_input.story_title.as_deref().unwrap_or("");
    let story_title_inner = story_title_block.inner(left_chunks[5]);
    story_title_block.render(left_chunks[5], frame.buffer_mut());
    Paragraph::new(story_title_text).render(story_title_inner, frame.buffer_mut());


    let commit_message_block = Block::bordered()
        .title(" Commit message ")
        .border_style(field_style(state, Field::CommitMessage));
    let commit_message_text = state.work_item_input.commit_message.as_deref().unwrap_or("");
    let commit_message_inner = commit_message_block.inner(left_chunks[6]);
    commit_message_block.render(left_chunks[6], frame.buffer_mut());
    Paragraph::new(commit_message_text).render(commit_message_inner, frame.buffer_mut());

    let validate_block = Block::bordered()
        .border_style(field_style(state, Field::Validate));
    let validate_text = "VALIDATE";
    let validate_inner = validate_block.inner(left_chunks[7]);
    validate_block.render(left_chunks[7], frame.buffer_mut());
    Paragraph::new(validate_text).alignment(Alignment::Center).render(validate_inner, frame.buffer_mut());

    let github_block = Block::bordered()
        .title(" GitHub ")
        .border_style(field_style(state, Field::Github));
    let github_inner = github_block.inner(right_vertical_chunks[0]);
    github_block.render(right_vertical_chunks[0], frame.buffer_mut());

    Paragraph::new(Text::from(state.github_lines())).render(github_inner, frame.buffer_mut());



    let history_block = Block::bordered()
        .title(" History ")
        .border_style(field_style(state, Field::History));
    let history_inner = history_block.inner(right_vertical_chunks[1]);
    history_block.render(right_vertical_chunks[1], frame.buffer_mut());

    let error_block = Block::bordered().title(" Error ").border_style(Style::default().fg(Color::Red));
    let error_inner = error_block.inner(right_vertical_chunks[2]);
    let error_text = state.error_message.as_deref().unwrap_or("");
    error_block.render(right_vertical_chunks[2], frame.buffer_mut());
    Paragraph::new(error_text).render(error_inner, frame.buffer_mut());
}

// modification du style a la selection
fn field_style(state: &AppState, field: Field) -> Style {
    if state.selected_field == field && state.input_mode == InputMode::Edition {
        Style::default().fg(Color::Red)
    } else if state.selected_field == field {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::White)
    }
}