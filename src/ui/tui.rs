use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Stylize,
    text::{Text, ToSpan},
    widgets::{Block, Paragraph, Widget},
};
use ratatui::style::{Color, Style};

use crate::app::AppState;
use crate::domain::field::Field;
use crate::ui::output::{github_lines, history_item_lines};

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    let title = format!(" APS BranchGen - Team {} [Ctrl+Q to quit] ", state.team_name);

    let main_block = Block::bordered()
        .fg(Color::Blue)
        .title(title.to_span().into_centered_line());

    let inner_area = main_block.inner(area);
    main_block.render(area, frame.buffer_mut());

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(inner_area);

    let right_vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Fill(1), Constraint::Length(3)])
        .split(horizontal_chunks[1]);

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
        ])
        .split(left_main_block_inner);

    let pi_text = state
        .work_item_input
        .pi
        .map(|v| v.to_string())
        .unwrap_or_default();

    let it_text = state
        .work_item_input
        .it
        .map(|v| format!("{v:02}"))
        .unwrap_or_default();

    let story_type_text = state
        .work_item_input
        .story_type
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_default();

    let commit_type_text = state
        .work_item_input
        .commit_type
        .as_ref()
        .map(|v| v.to_string())
        .unwrap_or_default();

    let story_number_text = state
        .work_item_input
        .story_number
        .clone()
        .unwrap_or_default();

    let story_title_text = state
        .work_item_input
        .story_title
        .clone()
        .unwrap_or_default();

    let commit_message_text = state
        .work_item_input
        .commit_message
        .clone()
        .unwrap_or_default();

    let pi_block = Block::bordered()
        .title(" PI ")
        .border_style(field_style(state, Field::Pi));
    let pi_inner = pi_block.inner(left_chunks[0]);
    pi_block.render(left_chunks[0], frame.buffer_mut());
    Paragraph::new(pi_text.as_str()).render(pi_inner, frame.buffer_mut());
    place_cursor_if_selected(
        frame,
        state,
        Field::Pi,
        pi_inner,
        &pi_text,
        state.text_editors.pi.cursor,
    );

    let it_block = Block::bordered()
        .title(" IT ")
        .border_style(field_style(state, Field::It));
    let it_inner = it_block.inner(left_chunks[1]);
    it_block.render(left_chunks[1], frame.buffer_mut());
    Paragraph::new(it_text.as_str()).render(it_inner, frame.buffer_mut());

    let story_type_block = Block::bordered()
        .title(" Story type ")
        .border_style(field_style(state, Field::StoryType));
    let story_type_inner = story_type_block.inner(left_chunks[2]);
    story_type_block.render(left_chunks[2], frame.buffer_mut());
    Paragraph::new(story_type_text.as_str()).render(story_type_inner, frame.buffer_mut());

    let commit_type_block = Block::bordered()
        .title(" Commit type ")
        .border_style(field_style(state, Field::CommitType));
    let commit_type_inner = commit_type_block.inner(left_chunks[3]);
    commit_type_block.render(left_chunks[3], frame.buffer_mut());
    Paragraph::new(commit_type_text.as_str()).render(commit_type_inner, frame.buffer_mut());

    let story_number_block = Block::bordered()
        .title(" Story number ")
        .border_style(field_style(state, Field::StoryNumber));
    let story_number_inner = story_number_block.inner(left_chunks[4]);
    story_number_block.render(left_chunks[4], frame.buffer_mut());
    Paragraph::new(story_number_text.as_str()).render(story_number_inner, frame.buffer_mut());
    place_cursor_if_selected(
        frame,
        state,
        Field::StoryNumber,
        story_number_inner,
        &story_number_text,
        state.text_editors.story_number.cursor,
    );

    let story_title_block = Block::bordered()
        .title(" Story title ")
        .border_style(field_style(state, Field::StoryTitle));
    let story_title_inner = story_title_block.inner(left_chunks[5]);
    story_title_block.render(left_chunks[5], frame.buffer_mut());
    Paragraph::new(story_title_text.as_str()).render(story_title_inner, frame.buffer_mut());
    place_cursor_if_selected(
        frame,
        state,
        Field::StoryTitle,
        story_title_inner,
        &story_title_text,
        state.text_editors.story_title.cursor,
    );

    let commit_message_block = Block::bordered()
        .title(" Commit message ")
        .border_style(field_style(state, Field::CommitMessage));
    let commit_message_inner = commit_message_block.inner(left_chunks[6]);
    commit_message_block.render(left_chunks[6], frame.buffer_mut());
    Paragraph::new(commit_message_text.as_str()).render(commit_message_inner, frame.buffer_mut());
    place_cursor_if_selected(
        frame,
        state,
        Field::CommitMessage,
        commit_message_inner,
        &commit_message_text,
        state.text_editors.commit_message.cursor,
    );

    let validate_block = Block::bordered().border_style(field_style(state, Field::Validate));
    let validate_inner = validate_block.inner(left_chunks[7]);
    validate_block.render(left_chunks[7], frame.buffer_mut());
    Paragraph::new("VALIDATE")
        .alignment(Alignment::Center)
        .render(validate_inner, frame.buffer_mut());

    let github_block = Block::bordered()
        .title(" GitHub ")
        .border_style(field_style(state, Field::Github));
    let github_inner = github_block.inner(right_vertical_chunks[0]);
    github_block.render(right_vertical_chunks[0], frame.buffer_mut());
    Paragraph::new(Text::from(github_lines(state.generated_output.as_ref())))
        .render(github_inner, frame.buffer_mut());

    let history_block = Block::bordered()
        .title(" History ")
        .border_style(field_style(state, Field::History));
    let history_inner = history_block.inner(right_vertical_chunks[1]);
    history_block.render(right_vertical_chunks[1], frame.buffer_mut());
    Paragraph::new(Text::from(history_item_lines(state.history.items().first())))
        .render(history_inner, frame.buffer_mut());

    let error_block = Block::bordered()
        .title(" Error ")
        .border_style(Style::default().fg(Color::Red));
    let error_inner = error_block.inner(right_vertical_chunks[2]);
    let error_text = state.error_message.as_deref().unwrap_or("");
    error_block.render(right_vertical_chunks[2], frame.buffer_mut());
    Paragraph::new(error_text).render(error_inner, frame.buffer_mut());
}

fn field_style(state: &AppState, field: Field) -> Style {
    if state.selected_field == field {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::White)
    }
}

fn place_cursor_if_selected(
    frame: &mut Frame,
    state: &AppState,
    field: Field,
    area: Rect,
    text: &str,
    cursor: usize,
) {
    if state.selected_field != field || area.width == 0 || area.height == 0 {
        return;
    }

    let logical_cursor = cursor.min(text.chars().count());
    let max_offset = area.width.saturating_sub(1) as usize;
    let x = area.x + logical_cursor.min(max_offset) as u16;
    let y = area.y;

    frame.set_cursor_position((x, y));
}