#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Pi,
    It,
    StoryType,
    CommitType,
    StoryNumber,
    StoryTitle,
    CommitMessage,
    Validate,
    Github,
    History,
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
            Field::CommitMessage => Field::Validate,
            Field::Validate => Field::Github,
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
            Field::Validate => Field::CommitMessage,
            Field::Github => Field::Validate,
            Field::History => Field::Github,
        }
    }

    pub fn is_text_input(&self) -> bool {
        matches!(self, Field::Pi | Field::StoryNumber | Field::StoryTitle | Field::CommitMessage)
    }

    pub fn is_enum_input(&self) -> bool {
        matches!(self, Field::It | Field::StoryType | Field::CommitType)
    }
    
    pub fn is_validate(&self) -> bool {
        matches!(self, Field::Validate)
    }
}