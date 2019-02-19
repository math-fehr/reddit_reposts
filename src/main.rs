mod reddit_comment;
mod reddit_post;
mod edit_state;
mod possible_types;
mod read_files;
mod data_analysis;
mod utils;

use crate::reddit_post::RedditPost;
use regex::Regex;
use crate::read_files::*;
use crate::data_analysis::*;
use crate::utils::*;

#[allow(dead_code)]
fn get_url_regex() -> Regex {
    Regex::new(r"https?://(www.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b[-a-zA-Z0-9@:%_+.~#?&/=;]*").unwrap()
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct SimpleRedditPost {
    created_utc: i32,
    subreddit: String,
}

impl HasCreationDate for SimpleRedditPost {
    fn get_creation_date(&self) -> i32 {
        self.created_utc
    }
}

impl HasSubreddit for SimpleRedditPost {
    fn get_subreddit(&self) -> &str {
        &self.subreddit
    }
}

impl From<RedditPost> for SimpleRedditPost {
    fn from(post: RedditPost) -> Self {
        Self {
            created_utc: post.created_utc,
            subreddit: post.subreddit,
        }
    }
}

fn main() {
    let filepaths = vec!["datasets/RS_2011-02".to_string()];
    let item_iterator = JSONItemIterator::<RedditPost,_>::new(filepaths.clone().into_iter());
    let (exec_ms, mut map) = measure_time(|| get_links_inside_subreddits(item_iterator));
    let map: HashMap<_,_> = map.into_iter().map(|(_subreddit, urls)| {
        (_subreddit, urls.into_iter().filter(|(_url, n)| *n > 1).collect::<HashMap<_,_>>())
    })
        .filter(|(_subreddit, urls)| urls.len() != 0).collect();
    println!("{:#?}", map);
    println!("{:?}", exec_ms);
}
