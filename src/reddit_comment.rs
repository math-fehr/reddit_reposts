use serde::{Deserialize};

pub use crate::edit_state::EditState;

/**
 * A struct representing a reddit comment.
 */
#[derive(Deserialize, Debug, Clone)]
pub struct RedditComment {
    author: String,
    subreddit_id: String,
    parent_id: String,
    gilded: i32,
    created_utc: String,
    edited: EditState,
    archived: bool,
    link_id: String,
    body: String,
    author_flair_text: Option<String>,
    distinguished: Option<String>,
    controversiality: i32,
    ups: i32,
    score_hidden: bool,
    name: String,
    subreddit: String,
    downs: i32,
    score: i32,
    author_flair_css_class: Option<String>,
    retrieved_on: i32,
    id: String,
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
