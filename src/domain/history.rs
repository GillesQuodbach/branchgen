use crate::domain::types::{GeneratedOutput, HistoryItem, WorkItemInput};

pub fn build_history_item(input: &WorkItemInput, generated_output: &GeneratedOutput, team_name: &str) -> HistoryItem {
    HistoryItem::new(team_name.to_string(), input.clone(), generated_output.clone())
}