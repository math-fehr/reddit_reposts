//! Functions to get and save posts from chosen subreddits

pub use crate::reddit_post::RedditPost;
pub use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

/// Get all posts from chosen subreddits
#[allow(dead_code)]
pub fn get_subreddit_posts<IT>(
    iterator: IT,
    subreddit_names: HashSet<String>,
) -> HashMap<String, HashMap<String, HashSet<RedditPost>>>
where
    IT: Iterator<Item = RedditPost>,
{
    let mut map: HashMap<_, _> = subreddit_names
        .into_iter()
        .map(|name| (name, HashMap::new()))
        .collect();
    for post in iterator {
        let url = post.get_linked_url();
        if let Some(url) = url {
            let subreddit_map = map.get_mut(&post.subreddit);
            if let Some(subreddit_map) = subreddit_map {
                if !subreddit_map.contains_key(&url) {
                    subreddit_map.insert(url.to_string(), HashSet::new());
                }
                subreddit_map.get_mut(&url).unwrap().insert(post);
            }
        }
    }
    map
}

/// Save the subreddits posts in a file
#[allow(dead_code)]
pub fn save_subreddit_posts(posts: HashMap<String, HashSet<RedditPost>>, filepath: String) {
    let posts = serde_json::to_string(&posts).unwrap();
    let mut file = File::create(filepath).unwrap();
    file.write(posts.as_bytes()).unwrap();
}

/// Load the subreddits posts from a file
#[allow(dead_code)]
pub fn load_subreddits_posts(filepath: &str) -> HashMap<String, HashSet<RedditPost>> {
    let file = File::open(filepath).unwrap();
    serde_json::from_reader(file).unwrap()
}
