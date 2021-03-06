//! Function simplifying the datasets by removing posts from the dataset,
//! and removing fields

use crate::reddit_post::*;
use serde::Deserialize;
use std::borrow::Cow;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Simplify a post dataset by removing the unused fields
/// Write the new dataset in a new file
#[allow(dead_code)]
pub fn simplify_post_dataset(filepath: &str, new_filepath: &str, keep_non_url_posts: bool) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut writer = csv::Writer::from_path(new_filepath).unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        let post = serde_json::from_str::<RedditPostJSON>(&line);
        if post.is_err() {
            println!("Reddit Post JSON: {}", line);
            println!("Error while parsing JSON: {}", post.unwrap_err());
            panic!();
        }
        let post = post.unwrap();
        if let Some(post) = post.into_reddit_post() {
            if !keep_non_url_posts && post.get_linked_url().is_none() {
                continue;
            }
            writer.serialize(post).unwrap();
        }
    }
}

/// A struct representing a typed json reddit post.
/// It contains a subset of the fields used by the reddit posts.
#[derive(Deserialize, Debug, Clone)]
struct RedditPostJSON<'a> {
    #[serde(borrow)]
    pub href_url: Option<Cow<'a, str>>,
    pub num_comments: i32,
    #[serde(borrow)]
    pub promoted_url: Option<Cow<'a, str>>,
    pub score: i32,
    pub hidden: Option<bool>,
    pub gilded: i32,
    #[serde(borrow)]
    pub subreddit: Option<Cow<'a, str>>,
    pub id: &'a str,
    #[serde(borrow)]
    pub title: Cow<'a, str>,
    #[serde(borrow)]
    pub url: Cow<'a, str>,
    pub over_18: bool,
    pub author_cakeday: Option<bool>,
    #[serde(borrow)]
    pub permalink: Cow<'a, str>,
    pub author: &'a str,
    pub created_utc: i32,
}

/// to_string mapped on an option
fn cow_to_opt_string<'a>(opt: Option<Cow<'a, str>>) -> Option<String> {
    match opt {
        None => None,
        Some(s) => Some(s.to_string()),
    }
}

impl<'a> RedditPostJSON<'a> {
    /// Transform the JSON Reddit Post into a Reddit Post
    /// The difference between the two is that we don't want to keep some
    /// Reddit Posts (like the promoted one)
    pub fn into_reddit_post(self) -> Option<RedditPost> {
        if self.subreddit.is_none()
            || self.promoted_url.is_some()
            || (self.hidden.is_some() && self.hidden.unwrap())
        {
            None
        } else {
            Some(RedditPost {
                href_url: cow_to_opt_string(self.href_url),
                num_comments: self.num_comments,
                score: self.score,
                gilded: self.gilded,
                subreddit: self.subreddit.unwrap().to_string(),
                id: self.id.to_string(),
                title: self.title.to_string(),
                url: self.url.to_string(),
                over_18: self.over_18,
                author_cakeday: self.author_cakeday,
                permalink: self.permalink.to_string(),
                author: self.author.to_string(),
                created_utc: self.created_utc,
            })
        }
    }
}
