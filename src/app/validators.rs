use crate::domain::field::Field;
use crate::domain::types::WorkItemInput;

pub fn validate_current_field(field: Field, input: &WorkItemInput) -> Result<(), String> {
    match field {
        Field::Pi => {
            if input.pi.is_none(){
                Err("PI is missing".to_string())
            } else {
                Ok(())
            }
        }
        Field::It => {
            if input.it.is_none(){
                Err("IT is missing".to_string())
            } else {
                Ok(())
            }
        }
        Field::StoryNumber => {
            let value = input.story_number.as_deref().ok_or("Story number is missing")?;
            WorkItemInput::validate_story_number(value)
        }
        Field::StoryTitle => {
            let value = input.story_title.as_deref().ok_or("Story title is missing")?;
            if value.trim().is_empty() {
                Err("Story title is missing".to_string())
            } else {
                Ok(())
            }
        }
        Field::CommitMessage => {
            let value = input.commit_message.as_deref().ok_or("Commit message is missing")?;
            if value.trim().is_empty() {
                Err("Commit message is empty".to_string())
            } else {
                Ok(())
            }
        }
        Field::StoryType => {
            if input.story_type.is_none(){
                Err("Story type is missing".to_string())
            } else {
                Ok(())
            }
        }
        Field::CommitType => {
            if input.commit_type.is_none(){
                Err("Commit type is missing".to_string())
            } else {
                Ok(())
            }
        }
        Field::Validate | Field::Github | Field::History => Ok(())
    }
}

pub fn validate_form(input: &WorkItemInput) -> Result<(), String> {
    input.validate()
}