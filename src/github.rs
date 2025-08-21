use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Repository {
    pub allow_forking: bool,
    pub archived: bool,
    pub created_at: u64,
    pub default_branch: String,
    pub description: Option<String>,
    pub disabled: bool,
    pub fork: bool,
    pub forks: u32,
    pub forks_count: u32,
    pub full_name: String,
    pub homepage: Option<String>,
    pub id: u64,
    pub language: String,
    pub license: Option<String>,
    pub master_branch: String,
    pub name: String,
    pub owner: User,
    pub private: bool,
    pub pushed_at: u64,
    pub size: u32,
    pub stargazers: u32,
    pub stargazers_count: u32,
    pub updated_at: String, //TODO: Time
    pub watchers: u32,
    pub watchers_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct User {
    avatar_url: String,
    email: String,
    gravatar_id: String,
    id: u64,
    login: String,
    name: String,
    site_admin: bool,
    #[serde(rename = "type")]
    type_: String,
    user_view_type: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct CommitAuthor {
    pub email: String,
    pub name: String,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Commit {
    pub added: Vec<String>,
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
    pub timestamp: String, //TODO: time
    pub tree_id: String,
}
