use serde::{Deserialize, Serialize};
use crate::github::{Author, Commit, Repository, User};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebhookEvent {
    Push(WebhookPush),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebhookPush {
    pub after: String, //new HEAD,
    pub before: String,
    pub commits: Vec<Commit>,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub head_commit: Commit,
    pub pusher: Author,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: User,
}
