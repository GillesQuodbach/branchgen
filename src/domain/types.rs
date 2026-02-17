use std::fmt;
use crate::config::app_config::AppConfig;

#[derive(Debug)]
pub enum StoryType {
    Bugfix,
    Feature,
    Hotfix,
    Release,
    Support,
    Test,
}

impl fmt::Display for StoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            StoryType::Bugfix => "bugfix",
            StoryType::Feature => "feature",
            StoryType::Hotfix => "hotfix",
            StoryType::Release => "release",
            StoryType::Support => "support",
            StoryType::Test => "test",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub enum CommitType {
    Feat,
    Fix,
    Refactor,
    Perf,
    Style,
    Test,
    Docs,
    Build,
    Ops,
}

#[derive(Debug)]
pub struct WorkItemInput {
    pi: u32,
    it: u32,
    story_type: StoryType,
    commit_type: CommitType,
    story_number: String,
    story_title: String,
}

impl WorkItemInput {
    pub fn new(pi: u32, it: u32, story_type: StoryType, commit_type: CommitType, story_number: String, story_title: String) -> Self {
        Self {
            pi,
            it,
            story_type,
            commit_type,
            story_number,
            story_title,
        }
    }

    pub fn create_branch(&self, team: &str) -> String {
        format!("{}/{}-{}_{}_{}_{}", self.story_title, self.pi, self.it, team, self.story_number, self.story_title)
    }
}

struct GeneratedOutput {
    checkout_cmd: String,
    branch_name: String,
    commit_msg: String,
    pr_title: String,
}

impl GeneratedOutput {
    fn new(checkout_cmd: String, branch_name: String, commit_msg: String, pr_title: String) -> Self {
        Self {
            checkout_cmd,
        branch_name,
        commit_msg,
        pr_title
    }
    }
}

struct HistoryItem {
    id: String,
    created_at: String,
    team: String,
    input: WorkItemInput,
    output: GeneratedOutput,
}

impl HistoryItem {
    fn new(id: String, created_at: String, team: String, input: WorkItemInput, output: GeneratedOutput) -> Self {
        Self {
            id,
            created_at,
            team,
            input,
            output,
        }
    }
}

struct HistoryFile {
    version: u32,
    items: Vec<HistoryItem>,
}

impl HistoryFile {
    fn new(version: u32, items: Vec<HistoryItem>) -> Self {
        Self { version, items }
    }
}