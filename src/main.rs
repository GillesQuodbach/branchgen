mod app;
mod domain;
mod ui;
mod storage;
mod error;
mod config;

use crate::config::app_config::{ load_or_init_config};
use dialoguer::{Input, Select};
use crate::domain::types::{CommitType, StoryType, WorkItemInput};

fn main() {

    let story_type_choices = vec!["Bugfix", "Feature", "Hotfix", "Release", "Support", "Test"];
    let commit_types_choices = vec!["feat", "fix", "refactor", "perf", "style", "test", "docs", "build", "ops"];

    println!("*********************");
    println!("Welcome to BranchGen");
    println!("*********************");

    let config = match load_or_init_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    println!("Config file read successfully. Team: {}", config.team);

    let pi: u32 = Input::new().with_prompt("Pi number:").interact_text().unwrap();

    println!("Pi number is {pi}");

    let it: u32 = Input::new().with_prompt("IT number:").interact_text().unwrap();
    println!("IT number is {it}");

    let story_type_choices_index = Select::new().with_prompt("Select story type:").items(&story_type_choices).default(0).interact().unwrap();

    let story_type = match story_type_choices_index {
        0 => StoryType::Bugfix,
        1 => StoryType::Feature,
        2 => StoryType::Hotfix,
        3 => StoryType::Release,
        4 => StoryType::Support,
        5 => StoryType::Test,
        _ => unreachable!(),
    };

    let commit_type_index = Select::new().with_prompt("Select your commit type").items(&commit_types_choices).default(0).interact().unwrap();

    let commit_type = match commit_type_index {
        0 => CommitType::Feat,
        1 => CommitType::Fix,
        2 => CommitType::Refactor,
        3 => CommitType::Perf,
        4 => CommitType::Style,
        5 => CommitType::Test,
        6 => CommitType::Docs,
        7 => CommitType::Build,
        8 => CommitType::Ops,
        _ => unreachable!(),
    };

    let story_number: String = Input::new().with_prompt("Story number (S-00000 or D-00000):").validate_with(|input:&String| {
        let chars: Vec<char> = input.chars().collect();
        if chars.len() != 8 {
            return Err(String::from("Story number should be 8 chars"));
        }

        if chars[0] != 's' && chars[0] != 'd'  {
            return Err(String::from("Story number must begin with s (story) or d (defect)"));
        }

        if chars[1] != '-' {
            return Err(String::from("Missing '-' in after s or d"));
        }

        if !chars[2..].iter().all(|c| c.is_ascii_digit()) {
            return Err(String::from("Last 6 characters must be digits"));
        }
        Ok(())
    })
        .interact_text()
        .unwrap();
    println!("Story number is {story_number}");

    let story_title: String = Input::new().with_prompt("Story title:").interact_text().unwrap();
    println!("Story title is {story_title}");

    let work_item = WorkItemInput::new(
        pi,
        it,
        story_type,
        commit_type,
        story_number,
        story_title,
        );

    println!("WorkItem input:{:?}",work_item);
}
