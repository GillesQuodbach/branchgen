enum StoryType {
    Bugfix,
    Feature,
    Hotfix,
    Release,
    Support,
    Test,
}

enum CommitType {
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

struct WorkItemInput {
    pi: u32,
    it: u32,
    story_type: StoryType,
    commit_type: CommitType,
    story_number: String,
    story_title: String,
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