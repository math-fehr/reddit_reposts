//! Contain structs representing a reddit post.

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};



/// A struct representing the interesting fields of a reddit post.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RedditPost {
    pub href_url: Option<String>,
    pub num_comments: i32,
    pub promoted_url: Option<String>,
    pub score: i32,
    pub gilded: i32,
    pub subreddit: String,
    pub id: String,
    pub original_link: Option<String>,
    pub title: String,
    pub is_self: bool,
    pub selftext: String,
    pub domain: String,
    pub url: String,
    pub over_18: bool,
    pub author_cakeday: Option<bool>,
    pub permalink: String,
    pub author: String,
    pub subreddit_id: String,
    pub created_utc: i32,
}

impl RedditPost {
    pub fn get_linked_url(&self) -> Option<String> {
        if self.url.as_bytes().len() == 22 + self.permalink.as_bytes().len() {
            if self.url.as_bytes()[22..] == self.permalink.as_bytes()[..] {
                return None;
            }
        }
        Some(self.url.clone())
    }
}

impl PartialEq for RedditPost {
    fn eq(&self, other: &RedditPost) -> bool {
        self.id == other.id
    }
}

impl Eq for RedditPost {}

impl Hash for RedditPost {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::RedditPost;
    use serde_json::Result;

    /// Try to deserialize a comment sampled from the dataset.
    #[test]
    fn serialize_comment() {
        let data = r#"{"downs":0,"link_flair_text":null,"distinguished":null,"media":{"oembed":{"width":600,"author_name":"mechudo2008","author_url":"http://www.youtube.com/user/mechudo2008","version":"1.0","provider_url":"http://www.youtube.com/","provider_name":"YouTube","thumbnail_width":480,"thumbnail_url":"http://i3.ytimg.com/vi/ZL4MGwlZuAc/hqdefault.jpg","height":363,"description":"deftones change\r\n\r\n1,000,000 VIEWS!!!","thumbnail_height":360,"html":"&lt;object width=\"600\" height=\"363\"&gt;&lt;param name=\"movie\" value=\"http://www.youtube.com/v/ZL4MGwlZuAc?fs=1\"&gt;&lt;/param&gt;&lt;param name=\"allowFullScreen\" value=\"true\"&gt;&lt;/param&gt;&lt;param name=\"allowscriptaccess\" value=\"always\"&gt;&lt;/param&gt;&lt;embed src=\"http://www.youtube.com/v/ZL4MGwlZuAc?fs=1\" type=\"application/x-shockwave-flash\" width=\"600\" height=\"363\" allowscriptaccess=\"always\" allowfullscreen=\"true\"&gt;&lt;/embed&gt;&lt;/object&gt;","url":"http://www.youtube.com/watch?v=ZL4MGwlZuAc","type":"video","title":"deftones-change"},"type":"youtube.com"},"url":"http://www.youtube.com/watch?v=ZL4MGwlZuAc&amp;feature=more_related","link_flair_css_class":null,"id":"euuri","edited":false,"num_reports":null,"created_utc":1293952912,"banned_by":null,"name":"t3_euuri","subreddit":"pirateradio","title":"Deftones - Change","author_flair_text":null,"is_self":false,"author":"adorabledork","media_embed":{"width":600,"scrolling":false,"content":"&lt;object width=\"600\" height=\"363\"&gt;&lt;param name=\"movie\" value=\"http://www.youtube.com/v/ZL4MGwlZuAc?fs=1\"&gt;&lt;/param&gt;&lt;param name=\"allowFullScreen\" value=\"true\"&gt;&lt;/param&gt;&lt;param name=\"allowscriptaccess\" value=\"always\"&gt;&lt;/param&gt;&lt;embed src=\"http://www.youtube.com/v/ZL4MGwlZuAc?fs=1\" type=\"application/x-shockwave-flash\" width=\"600\" height=\"363\" allowscriptaccess=\"always\" allowfullscreen=\"true\"&gt;&lt;/embed&gt;&lt;/object&gt;","height":363},"permalink":"/r/pirateradio/comments/euuri/deftones_change/","author_flair_css_class":null,"selftext":"","domain":"youtube.com","num_comments":0,"likes":null,"clicked":false,"thumbnail":"http://thumbs.reddit.com/t3_euuri.png","saved":false,"subreddit_id":"t5_2s923","ups":2,"approved_by":null,"score":2,"selftext_html":null,"created":1293952912,"hidden":false,"over_18":false}"#;
        let c: Result<RedditPost> = serde_json::from_str(data);
        //assert!(c.is_ok());
        c.unwrap();
    }
}
