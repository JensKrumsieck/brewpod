use crate::github::{Commit, CommitAuthor, Repository, User};
use serde::{Deserialize, Serialize};

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
    pub pusher: CommitAuthor,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: User,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_deserialize_webhook_push() {
        let contents = fs::read_to_string("testdata/webhook_push.json").unwrap();
        let _push_event: WebhookEvent = serde_json::from_str(&contents).unwrap(); //if it does not fail it is a PASS
    }
}
