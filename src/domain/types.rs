use std::fmt;
use std::fmt::{Display, Formatter};
use chrono::{DateTime, Local, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize, Default, Clone)]
pub enum StoryType {
    Bugfix,
    #[default]
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

impl StoryType {
    pub fn next(&self) -> Self {
        match self {
            StoryType::Bugfix => StoryType::Feature,
            StoryType::Feature => StoryType::Hotfix,
            StoryType::Hotfix => StoryType::Release,
            StoryType::Release => StoryType::Support,
            StoryType::Support => StoryType::Test,
            StoryType::Test => StoryType::Bugfix,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            StoryType::Bugfix => StoryType::Test,
            StoryType::Feature => StoryType::Bugfix,
            StoryType::Hotfix => StoryType::Feature,
            StoryType::Release => StoryType::Hotfix,
            StoryType::Support => StoryType::Release,
            StoryType::Test => StoryType::Support,
        }
    }
}

#[derive(Debug,Serialize, Deserialize, Default, Clone)]
pub enum CommitType {
    #[default]
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

impl CommitType {
    pub fn next(&self) -> Self {
        match self {
            CommitType::Feat => CommitType::Fix,
            CommitType::Fix => CommitType::Refactor,
            CommitType::Refactor => CommitType::Perf,
            CommitType::Perf => CommitType::Style,
            CommitType::Style => CommitType::Test,
            CommitType::Test => CommitType::Docs,
            CommitType::Docs => CommitType::Build,
            CommitType::Build => CommitType::Ops,
            CommitType::Ops => CommitType::Feat,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            CommitType::Feat => CommitType::Ops,
            CommitType::Fix => CommitType::Feat,
            CommitType::Refactor => CommitType::Fix,
            CommitType::Perf => CommitType::Refactor,
            CommitType::Style => CommitType::Perf,
            CommitType::Test => CommitType::Style,
            CommitType::Docs => CommitType::Test,
            CommitType::Build => CommitType::Docs,
            CommitType::Ops => CommitType::Build,
        }
    }
}

// represente les données du formulaire
#[derive(Debug,Serialize, Deserialize, Default, Clone)]
pub struct WorkItemInput {
    pub pi: Option<u32>,
    pub it: Option<u32>,
    pub story_type: Option<StoryType>,
    pub commit_type: Option<CommitType>,
    pub story_number: Option<String>,
    pub story_title: Option<String>,
    pub commit_message: Option<String>,
}

impl Display for WorkItemInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let pi = self.pi.map(|x| format!("PI-{x}")).unwrap_or_else(|| "PI-?".to_string());
        let it = self.it.map(|x| format!("IT-{x}")).unwrap_or_else(|| "IT-?".to_string());
        let story_type = self.story_type.as_ref().map(|x| x.to_string()).unwrap_or_else(|| "story-type?".to_string());
        let story_number = self.story_number.as_deref().unwrap_or("?");
        let story_title = self.story_title.as_deref().unwrap_or("?");

        write!(f, "{} {} | {} | story #{} | {}", pi,it, story_type, story_number, story_title)
    }
}
impl WorkItemInput {
    pub fn new(pi: u32, it: u32, story_type: StoryType, commit_type: CommitType, story_number: String, story_title: String, commit_message: String) -> Self {
        Self {
            pi: Some(pi),
            it: Some(it),
            story_type: Some(story_type),
            commit_type: Some(commit_type),
            story_number: Some(story_number),
            story_title: Some(story_title),
            commit_message: Some(commit_message),
        }
    }

    pub fn branch_name(&self, team: &str) -> Result<String, String> {
        let pi = self.pi.ok_or("PI is missing")?;
        let it = self.it.ok_or("IT is missing")?;
        let story_type = self.story_type.as_ref().ok_or("Story type is missing")?;
        let story_number = self.story_number.as_deref().ok_or("Story number is missing")?;
        let story_title = self.story_title.as_ref().ok_or("Story title is missing")?;
        Ok(format!("{}/{}-{}_{}_{}_{}", story_type, pi, it, team, story_number, Self::format_story_title(story_title)))
    }

    pub fn format_story_title(story_title: &str) -> String {
        story_title.trim().to_lowercase().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>().join("-")
    }

    pub fn commit_name(&self, team: &str) -> Result<String, String> {
        let pi = self.pi.ok_or("PI is missing")?;
        let it = self.it.ok_or("IT is missing")?;
        let story_number = self.story_number.as_deref().ok_or("Story number is missing")?;
        let commit_type = self.commit_type.as_ref().ok_or("Commit type is missing")?;
        let commit_message = self.commit_message.as_ref().ok_or("Commit message is missing")?;
        Ok(format!("{} [{}-{}] #{} - {}: {}",team, pi, it, story_number, commit_type, commit_message))
    }

    pub fn pr_name(&self, team: &str) -> Result<String, String> {
        let pi = self.pi.ok_or("PI is missing")?;
        let it = self.it.ok_or("IT is missing")?;
        let commit_type = self.commit_type.as_ref().ok_or("Commit type is missing")?;
        let story_type = self.story_type.as_ref().ok_or("Story type is missing")?;
        let story_number = self.story_number.as_deref().ok_or("Story number is missing")?;
        let story_title = self.story_title.as_deref().ok_or("Story title is missing")?;
        Ok(format!("{}: {}/{}-{}_{}_{}_{}", commit_type, story_type, pi, it, team, story_number, story_title))
    }

    pub fn validate_story_number(value: &str) -> Result<(), String> {
        let normalized = value.trim().to_lowercase();
        if normalized.len() != 8 {
            return Err("Story number should be 8 characters long".to_string());
        }
        let chars: Vec<char> = normalized.chars().collect();
        if chars[0] != 's' && chars[0] != 'd' {
            return Err("Story number must begin with s- (story) or d- (defect)".to_string());
        }
        if chars[1] != '-' {
            return Err("Missing '-' after s or d".to_string());
        }
        if !chars[2..].iter().all(|c| c.is_ascii_digit()) {
            return Err("Last 6 characters must be digits".to_string());
        }

        Ok(())
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.pi.is_none() {
            return Err("Pi is missing".to_string());
        }
        if self.it.is_none() {
            return Err("IT is missing".to_string());
        }
        if self.story_type.is_none() {
            return Err("Story type is missing".to_string());
        }
        if self.commit_type.is_none() {
            return Err("Commit type is missing".to_string());
        }

        let story_number = self.story_number.as_ref().ok_or("Story number is missing")?;
        Self::validate_story_number(story_number)?;

        let story_title = self.story_title.as_ref().ok_or("Story title is missing")?;
        if story_title.trim().is_empty() {
            return Err("Story title cannot be empty".to_string());
        }

        let commit_message = self.commit_message.as_ref().ok_or("Commit message is missing")?;
        if commit_message.trim().is_empty() {
            return Err("Commit message cannot be empty".to_string());
        }
        Ok(())





    }
}
#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct GeneratedOutput {
    pub checkout_cmd: String,
    pub branch_name: String,
    pub commit_msg: String,
    pub pr_title: String,
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

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct HistoryItem {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub team: String,
    input: WorkItemInput,
    pub output: GeneratedOutput,
}

impl Display for HistoryItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let local_created_at = self.created_at.with_timezone(&Local);
        writeln!(f, "[{}] {} | team: {}", local_created_at.format("%Y-%m-%d %H:%M:%S"), self.id, self.team)?;
        writeln!(f, "Input -> {}", self.input)?;
        write!(f, "Output -> {}", self.output)
    }
}

impl HistoryItem {
    pub fn new(team: String, input: WorkItemInput, output: GeneratedOutput) -> Self {

        let id = Uuid::new_v4();

        let created_at = Utc::now();

        Self {
            id,
            created_at,
            team,
            input,
            output,
        }
    }

}

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct HistoryFile {
    version: u32,
    items: Vec<HistoryItem>,
}

impl HistoryFile {
    pub fn new(items: Vec<HistoryItem>) -> Self {
        Self { version: 1, items }
    }
    
    pub fn push_item(&mut self, item: HistoryItem) {
        self.items.insert(0, item);
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn items(&self) -> &[HistoryItem] {
        &self.items
    }

    pub fn items_mut(&mut self) -> &mut Vec<HistoryItem> {
        &mut self.items
    }
}

impl Default for HistoryFile {
    fn default() -> Self {
        Self::new(vec![])
    }
}