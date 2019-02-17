use crate::edit_state::EditState;
use serde::Deserialize;

/**
 * A struct representing a reddit post.
 */
#[derive(Deserialize, Debug, Clone)]
pub struct RedditPost {
    num_reports: Option<i32>,
    title: String,
    subreddit_id: String,
    created: i32,
    over_18: bool,
    hidden: bool,
    author_flair_text: Option<String>,
    link_flair_text: Option<String>,
    id: String,
    distinguished: Option<String>,
    domain: String,
    author: Option<String>,
    link_flair_css_class: Option<String>,
    name: String,
    media: Option<Media>,
    selftext: String,
    likes: Option<()>,
    thumbnail: String,
    selftext_html: Option<String>,
    subreddit: String,
    banned_by: Option<()>,
    permalink: String,
    url: String,
    saved: bool,
    num_comments: i32,
    promoted: Option<bool>,
    clicked: bool,
    edited: EditState,
    author_flair_css_class: Option<String>,
    approved_by: Option<()>,
    is_self: bool,
    ups: i32,
    created_utc: i32,
    media_embed: MediaEmbed,
    downs: i32,
    score: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct Media {
    #[serde(rename = "type")]
    type_: String,
    content: Option<String>,
    width: Option<i32>,
    height: Option<i32>,
    oembed: Option<Oembed>,
}

#[derive(Deserialize, Debug, Clone)]
struct Oembed {
    thumbnail_width: Option<i32>,
    width: i32,
    author_url: Option<String>,
    height: i32,
    provider_url: String,
    title: Option<String>,
    description: Option<String>,
    thumbnail_height: Option<i32>,
    author_name: Option<String>,
    thumbnail_url: Option<String>,
    html: String,
    version: String,
    url: Option<String>,
    #[serde(rename = "type")]
    type_: String,
    provider_name: String,
    cache_age: Option<i32>,
    html5: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct MediaEmbed {
    scrolling: Option<bool>,
    height: Option<i32>,
    width: Option<i32>,
    content: Option<String>,
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
