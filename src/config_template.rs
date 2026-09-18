pub const DEFAULT_CONFIG: &str = r#"
# Configuration par défaut de l'équipe ASPICOT
# Le fichier est directement utilisable après l'initialisation.
# Types disponibles : text, number, select
# Normalisations : none, spaces, uppercase, lowercase,
#                  lowercasefirstletter, uppercasefirstletter

[[fields]]
key        = "git"
label      = "Git"
type       = "select"
values     = ["ASPICOT"]
required   = true
persistent = true

[[fields]]
key      = "pi"
label    = "PI"
type     = "number"
required = true

[[fields]]
key      = "it"
label    = "IT"
type     = "text"
required = true

[[fields]]
key      = "story_type"
label    = "Type Story"
type     = "select"
values   = ["feature", "bug", "task"]
required = true

[[fields]]
key      = "commit_type"
label    = "Type commit"
type     = "select"
values   = ["feat", "fix", "refactor", "docs", "chore"]
required = true

[[fields]]
key        = "story_ref"
label      = "Story/Defect Ref (S-X...)"
type       = "text"
required   = true
normalize  = "lowercase"

[[fields]]
key      = "task_ref"
label    = "Task Ref (tk-xxxxxx)"
type     = "text"
required = false

[[fields]]
key       = "story_title"
label     = "Story title"
type      = "text"
required  = true
normalize = "spaces"

[formats]
branch   = "{story_type}/{pi}-{it}_{git}_{story_ref}_{story_title}"
commit   = "AS [{pi}-{it}] #{story_ref} - {commit_type}: details to update"
pr_title = "{commit_type}: {pi}-{it}_{git}_{story_ref}_{story_title}"
"#;