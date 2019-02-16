use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};

/**
 * A struct representing a reddit comment.
 */
#[derive(Deserialize, Debug)]
pub struct RedditComment {
    controversiality: u32,
    body: String,
    subreddit_id: String,
    link_id: String,
    subreddit: String,
    score: i32,
    ups: i32,
    author_flair_css_class: Option<String>,
    created_utc: String,
    author_flair_text: Option<String>,
    author: String,
    id: String,
    edited: EditState,
    parent_id: String,
    gilded: u32,
    distinguished: Option<String>,
    retrieved_on: u32,
}

/**
 * A struct representing the edited field in a reddit comment
 */
#[derive(Debug)]
pub enum EditState {
    Bool(bool),
    UTC(u32),
}

/**
* Visitor pattern to deserialize EditState
*/
struct EditStateVisitor;

impl<'de> Visitor<'de> for EditStateVisitor {
    type Value = EditState;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean or an unsigned 32-bits integer")
    }

    fn visit_bool<E>(self, value: bool) -> Result<EditState, E>
    where
        E: de::Error,
    {
        Ok(EditState::Bool(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<EditState, E>
    where
        E: de::Error,
    {
        Ok(EditState::UTC(value as u32))
    }
}

impl<'de> Deserialize<'de> for EditState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EditStateVisitor)
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
