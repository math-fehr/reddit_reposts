use serde::{Deserialize};
use std::hash::{Hash, Hasher};

pub use crate::edit_state::EditState;

/**
 * A struct representing a reddit comment.
 */
#[derive(Deserialize, Debug, Clone)]
pub struct RedditComment {
    pub author: String,
    pub subreddit_id: String,
    pub parent_id: String,
    pub gilded: i32,
    pub created_utc: String,
    pub edited: EditState,
    pub archived: bool,
    pub link_id: String,
    pub body: String,
    pub author_flair_text: Option<String>,
    pub distinguished: Option<String>,
    pub controversiality: i32,
    pub ups: i32,
    pub score_hidden: bool,
    pub name: String,
    pub subreddit: String,
    pub downs: i32,
    pub score: i32,
    pub author_flair_css_class: Option<String>,
    pub retrieved_on: i32,
    pub id: String,
}

impl PartialEq for RedditComment {
    fn eq(&self, other: &RedditComment) -> bool {
        self.id == other.id
    }
}

impl Eq for RedditComment {}

impl Hash for RedditComment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::RedditComment;
    use serde_json::Result;
    #[test]
    fn serialize_comment() {
        let data = r#"{"archived":true,"downs":0,"link_id":"t3_etyqc","score_hidden":false,"id":"c1b06fp","author_flair_css_class":null,"body":"They should add that to the instructions on the box :p","ups":1,"distinguished":null,"gilded":0,"edited":false,"retrieved_on":1426664469,"parent_id":"t1_c1azvxa","created_utc":"1293840000","subreddit":"sex","controversiality":0,"author_flair_text":null,"score":1,"name":"t1_c1b06fp","author":"SandRider","subreddit_id":"t5_2qh3p"}"#;
        let c: Result<RedditComment> = serde_json::from_str(data);
        assert!(c.is_ok());
    }
}
