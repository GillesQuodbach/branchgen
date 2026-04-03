use ratatui::prelude::Line;
use crate::app::AppState;
use crate::domain::types::GeneratedOutput;

pub fn build_generated_output(state: &AppState) -> Result<GeneratedOutput, String> {
    let branch_name = state.work_item_input.branch_name(&state.team_name)?;
    let checkout_cmd = GeneratedOutput::format_checkout_cmd(&branch_name);
    let commit_msg = state.work_item_input.commit_name(&state.team_name)?;
    let pr_title = state.work_item_input.pr_name(&state.team_name)?;

    Ok(GeneratedOutput::new(
        checkout_cmd,
        branch_name,
        commit_msg,
        pr_title,
    ))
}
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