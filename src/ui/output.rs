use ratatui::prelude::Line;
use crate::domain::types::{GeneratedOutput, HistoryItem};

pub fn github_lines(generated_output: Option<&GeneratedOutput>) -> Vec<Line<'static>>{
    match generated_output {
        Some(output) => vec![
            Line::from(format!("checkout: {}", output.checkout_cmd)),
            Line::from(format!("branch: {}", output.branch_name)),
            Line::from(format!("commit: {}", output.commit_msg)),
            Line::from(format!("pr: {}", output.pr_title)),
        ],
        None => vec![
            Line::from("No output yet"),
        ]
    }

}

pub fn history_item_lines(history_item: Option<&HistoryItem>) -> Vec<Line<'static>> {
    match history_item {
        Some(item) => vec! [
            Line::from(format!("history: {}", item.team)),
            Line::from(format!("history: {}", item.created_at)),
            Line::from(format!("history: {}", item.output)),
        ],
        None => vec! [
            Line::from("No output yet"),
        ]
        }
    }
