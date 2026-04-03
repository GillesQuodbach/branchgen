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