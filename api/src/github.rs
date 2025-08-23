use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
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
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub watchers: u32,
    pub watchers_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    avatar_url: String,
    email: Option<String>,
    gravatar_id: String,
    id: u64,
    login: String,
    name: Option<String>,
    site_admin: bool,
    #[serde(rename = "type")]
    type_: String,
    user_view_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitAuthor {
    pub email: String,
    pub name: String,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub added: Vec<String>,
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub tree_id: String,
}
