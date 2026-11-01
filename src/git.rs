use crate::error::AppError;
use std::process::Command;

pub fn create_branch(branch_name: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(AppError::Git(error_msg))
    }
}

pub fn is_git_repo() -> bool {
    std::path::Path::new(".git").exists()
}

pub fn checkout_branch(branch_name: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .args(["checkout", branch_name])
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(AppError::Git(error_msg))
    }
}
