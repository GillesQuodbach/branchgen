#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Pi,
    It,
    StoryType,
    CommitType,
    StoryNumber,
    StoryTitle,
    CommitMessage,
    Github,
    History
}

impl Default for Field {
    fn default() -> Self {
        Field::StoryNumber
    }
}

impl Field {
    pub fn next(self) -> Self {
        match self {
            Field::Pi => Field::It,
            Field::It => Field::StoryType,
            Field::StoryType => Field::CommitType,
            Field::CommitType => Field::StoryNumber,
            Field::StoryNumber => Field::StoryTitle,
            Field::StoryTitle => Field::CommitMessage,
            Field::CommitMessage => Field::Github,
            Field::Github => Field::History,
            Field::History => Field::Pi,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Field::Pi => Field::History,
            Field::It => Field::Pi,
            Field::StoryType => Field::It,
            Field::CommitType => Field::StoryType,
            Field::StoryNumber => Field::CommitType,
            Field::StoryTitle => Field::StoryNumber,
            Field::CommitMessage => Field::StoryTitle,
            Field::Github => Field::CommitMessage,
            Field::History => Field::Github,
        }
    }

    pub fn is_editable(&self) -> bool {
        matches!(self, Field::Pi | Field::It | Field::StoryNumber | Field::StoryTitle | Field::CommitMessage)
    }
}