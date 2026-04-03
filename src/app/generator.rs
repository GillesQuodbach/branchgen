use crate::domain::types::{GeneratedOutput, WorkItemInput};

pub fn build_generated_output(input: &WorkItemInput, team_name: &str) -> Result<GeneratedOutput, String> {
    let branch_name = input.branch_name(&team_name)?;
    let checkout_cmd = GeneratedOutput::format_checkout_cmd(&branch_name);
    let commit_msg = input.commit_name(&team_name)?;
    let pr_title = input.pr_name(&team_name)?;

    Ok(GeneratedOutput::new(
        checkout_cmd,
        branch_name,
        commit_msg,
        pr_title,
    ))
}