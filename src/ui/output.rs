use ratatui::prelude::Line;
use crate::app::AppState;
use crate::domain::types::GeneratedOutput;

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