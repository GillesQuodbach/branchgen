use std::fmt;
use std::fmt::{Display, Formatter};
use chrono::{Local};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub enum StoryType {
    Bugfix,
    Feature,
    Hotfix,
    Release,
    Support,
    Test,
}

impl Display for StoryType {
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

#[derive(Debug,Serialize, Deserialize)]
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

impl Display for CommitType {
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

#[derive(Debug,Serialize, Deserialize)]
pub struct WorkItemInput {
    pi: u32,
    it: u32,
    story_type: StoryType,
    commit_type: CommitType,
    story_number: String,
    story_title: String,
    commit_message: String,
}

impl Display for WorkItemInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PI-{} IT-{} | {} | story #{} | {}",
               self.pi,
               self.it,
               self.story_type,
               self.story_number,
               self.story_title,
        )
    }
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
#[derive(Debug,Serialize, Deserialize)]
pub struct GeneratedOutput {
    checkout_cmd: String,
    branch_name: String,
    commit_msg: String,
    pr_title: String,
}

impl Display for GeneratedOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "branch: {} | commit: {} | pr: {}",
            self.branch_name,
            self.commit_msg,
            self.pr_title
        )
    }
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

#[derive(Debug,Serialize, Deserialize)]
pub struct HistoryItem {
    id: Uuid,
    created_at: String,
    team: String,
    input: WorkItemInput,
    output: GeneratedOutput,
}

impl Display for HistoryItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}] {} | team: {}", self.created_at, self.id, self.team)?;
        writeln!(f, "Input -> {}", self.input)?;
        write!(f, "Output -> {}", self.output)
    }
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

#[derive(Debug,Serialize, Deserialize)]
pub struct HistoryFile {
    version: u32,
    items: Vec<HistoryItem>,
}

impl HistoryFile {
    pub fn new(version: u32, items: Vec<HistoryItem>) -> Self {
        Self { version, items }
    }
    
    pub fn push_item(&mut self, item: HistoryItem) {
        self.items.push(item);
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn items(&self) -> &[HistoryItem] {
        &self.items
    }
}