use std::fmt;
use chrono::{Local};
use uuid::Uuid;

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

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CommitType::Feat => "feat",
            CommitType::Fix => "fix",
            CommitType::Refactor => "refactor",
            CommitType::Perf => "perf",
            CommitType::Style => "style",
            CommitType::Test => "test",
            CommitType::Docs => "docs",
            CommitType::Build => "build",
            CommitType::Ops => "ops",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug)]
pub struct WorkItemInput {
    pi: u32,
    it: u32,
    story_type: StoryType,
    commit_type: CommitType,
    story_number: String,
    story_title: String,
    commit_message: String,
}

impl WorkItemInput {
    pub fn new(pi: u32, it: u32, story_type: StoryType, commit_type: CommitType, story_number: String, story_title: String, commit_message: String) -> Self {
        Self {
            pi,
            it,
            story_type,
            commit_type,
            story_number,
            story_title,
            commit_message,
        }
    }

    pub fn branch_name(&self, team: &str) -> String {
        format!("{}/{}-{}_{}_{}_{}", self.story_type, self.pi, self.it, team, self.story_number, self.story_title)
    }

    pub fn format_story_title(story_title: &str) -> String {
        story_title.trim().to_lowercase().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>().join("-")
    }

    pub fn commit_name(&self, team: &str) -> String {
        format!("{} [{}-{}] #{} - {}: {}",team, self.pi, self.it, self.story_number, self.commit_type, self.commit_message)
    }

    pub fn pr_name(&self, team: &str) -> String {
        format!("{}: {}/{}-{}_{}_{}_{}", self.commit_type, self.story_type, self.pi, self.it, team, self.story_number, self.story_title)
    }
}
#[derive(Debug)]
pub struct GeneratedOutput {
    checkout_cmd: String,
    branch_name: String,
    commit_msg: String,
    pr_title: String,
}

impl GeneratedOutput {
    pub fn new(checkout_cmd: String, branch_name: String, commit_msg: String, pr_title: String) -> Self {
        Self {
            checkout_cmd,
            branch_name,
            commit_msg,
            pr_title
        }
    }

    pub fn format_checkout_cmd(branch_name: &str) -> String {
        format!("git checkout -b {}", branch_name)
    }
}

#[derive(Debug)]
pub struct HistoryItem {
    id: Uuid,
    created_at: String,
    team: String,
    input: WorkItemInput,
    output: GeneratedOutput,
}

impl HistoryItem {
    pub fn new(team: String, input: WorkItemInput, output: GeneratedOutput) -> Self {

        let id = Uuid::new_v4();

        let created_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

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