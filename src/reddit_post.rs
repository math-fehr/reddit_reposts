use crate::edit_state::EditState;
use serde::Deserialize;
use std::hash::{Hash, Hasher};

/**
 * A struct representing a reddit post.
 */
#[derive(Deserialize, Debug, Clone)]
pub struct RedditPost {
    pub num_reports: Option<i32>,
    pub title: String,
    pub subreddit_id: String,
    pub created: i32,
    pub over_18: bool,
    pub hidden: bool,
    pub author_flair_text: Option<String>,
    pub link_flair_text: Option<String>,
    pub id: String,
    pub distinguished: Option<String>,
    pub domain: String,
    pub author: Option<String>,
    pub link_flair_css_class: Option<String>,
    pub name: String,
    pub media: Option<Media>,
    pub selftext: String,
    pub likes: Option<()>,
    pub thumbnail: String,
    pub selftext_html: Option<String>,
    pub subreddit: String,
    pub banned_by: Option<()>,
    pub permalink: String,
    pub url: String,
    pub saved: bool,
    pub num_comments: i32,
    pub promoted: Option<bool>,
    pub clicked: bool,
    pub edited: EditState,
    pub author_flair_css_class: Option<String>,
    pub approved_by: Option<()>,
    pub is_self: bool,
    pub ups: i32,
    pub created_utc: i32,
    pub media_embed: MediaEmbed,
    pub downs: i32,
    pub score: i32,
}

impl RedditPost {
    pub fn get_linked_url(&self) -> Option<String> {
        if self.url == self.permalink {
            None
        } else {
            Some(self.url.clone())
        }
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

#[derive(Deserialize, Debug, Clone)]
pub struct Media {
    #[serde(rename = "type")]
    pub type_: String,
    pub content: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub oembed: Option<Oembed>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Oembed {
    pub thumbnail_width: Option<i32>,
    pub width: i32,
    pub author_url: Option<String>,
    pub height: i32,
    pub provider_url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail_height: Option<i32>,
    pub author_name: Option<String>,
    pub thumbnail_url: Option<String>,
    pub html: String,
    pub version: String,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub provider_name: String,
    pub cache_age: Option<i32>,
    pub html5: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaEmbed {
    pub scrolling: Option<bool>,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub content: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::RedditPost;
    use serde_json::Result;
    #[test]
    fn serialize_comment() {
        let data = r#"{"downs":0,"link_flair_text":null,"distinguished":null,"media":{"oembed":{"width":600,"author_name":"mechudo2008","author_url":"http://www.youtube.com/user/mechudo2008","version":"1.0","provider_url":"http://www.youtube.com/","provider_name":"YouTube","thumbnail_width":480,"thumbnail_url":"http://i3.ytimg.com/vi/ZL4MGwlZuAc/hqdefault.jpg","height":363,"description":"deftones change\r\n\r\n1,000,000 VIEWS!!!","thumbnail_height":360,"html":"&lt;object width=\"600\" height=\"363\"&gt;&lt;param name=\"movie\" value=\"http://www.youtube.com/v/ZL4MGwlZuAc?fs=1\"&gt;&lt;/param&gt;&lt;param name=\"allowFullScreen\" value=\"true\"&gt;&lt;/param&gt;&lt;param name=\"allowscriptaccess\" value=\"always\"&gt;&lt;/param&gt;&lt;embed src=\"http://www.youtube.com/v/ZL4MGwlZuAc?fs=1\" type=\"application/x-shockwave-flash\" width=\"600\" height=\"363\" allowscriptaccess=\"always\" allowfullscreen=\"true\"&gt;&lt;/embed&gt;&lt;/object&gt;","url":"http://www.youtube.com/watch?v=ZL4MGwlZuAc","type":"video","title":"deftones-change"},"type":"youtube.com"},"url":"http://www.youtube.com/watch?v=ZL4MGwlZuAc&amp;feature=more_related","link_flair_css_class":null,"id":"euuri","edited":false,"num_reports":null,"created_utc":1293952912,"banned_by":null,"name":"t3_euuri","subreddit":"pirateradio","title":"Deftones - Change","author_flair_text":null,"is_self":false,"author":"adorabledork","media_embed":{"width":600,"scrolling":false,"content":"&lt;object width=\"600\" height=\"363\"&gt;&lt;param name=\"movie\" value=\"http://www.youtube.com/v/ZL4MGwlZuAc?fs=1\"&gt;&lt;/param&gt;&lt;param name=\"allowFullScreen\" value=\"true\"&gt;&lt;/param&gt;&lt;param name=\"allowscriptaccess\" value=\"always\"&gt;&lt;/param&gt;&lt;embed src=\"http://www.youtube.com/v/ZL4MGwlZuAc?fs=1\" type=\"application/x-shockwave-flash\" width=\"600\" height=\"363\" allowscriptaccess=\"always\" allowfullscreen=\"true\"&gt;&lt;/embed&gt;&lt;/object&gt;","height":363},"permalink":"/r/pirateradio/comments/euuri/deftones_change/","author_flair_css_class":null,"selftext":"","domain":"youtube.com","num_comments":0,"likes":null,"clicked":false,"thumbnail":"http://thumbs.reddit.com/t3_euuri.png","saved":false,"subreddit_id":"t5_2s923","ups":2,"approved_by":null,"score":2,"selftext_html":null,"created":1293952912,"hidden":false,"over_18":false}"#;
        let c: Result<RedditPost> = serde_json::from_str(data);
        //assert!(c.is_ok());
        c.unwrap();
    }
}
