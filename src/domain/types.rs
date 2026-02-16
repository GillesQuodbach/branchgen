#[derive(Debug)]
pub enum StoryType {
    Bugfix,
    Feature,
    Hotfix,
    Release,
    Support,
    Test,
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
    pub pi: u32,
    pub it: u32,
    pub story_type: StoryType,
    pub commit_type: CommitType,
    pub story_number: String,
    pub story_title: String,
}

struct GeneratedOutput {
    checkout_cmd: String,
    branch_name: String,
    commit_msg: String,
    pr_title: String,
}

struct HistoryItem {
    id: String,
    created_at: String,
    team: String,
    input: WorkItemInput,
    output: GeneratedOutput,
}

struct HistoryFile {
    version: u32,
    items: Vec<HistoryItem>,
}