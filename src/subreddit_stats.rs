/// Functions used to compute subreddits statistics

use std::collections::HashMap;
use crate::reddit_post::RedditPost;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

/// Structure containing the interesting stats about a subreddit
#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct SubredditStats {
    pub n_posts: i32,
    pub n_comments: i32,
}

/// Get the all the present subreddits.
/// Get only the n_subreddits subreddit with the most posts if given
#[allow(dead_code)]
pub fn compute_subreddits_stats<IT>(iterator: IT) -> HashMap<String, SubredditStats>
where
    IT: Iterator<Item = RedditPost>,
{
    let mut subreddits = HashMap::<String, SubredditStats>::new();
    for post in iterator {
        let n_comments = post.num_comments;
        if let Some(stats) = subreddits.get_mut(&post.subreddit) {
            stats.n_posts += 1;
            stats.n_comments += n_comments;
        } else {
            subreddits.insert(post.subreddit, SubredditStats {
                n_posts: 1,
                n_comments,
            });
        }
    }
    subreddits
}

/// Save the subreddits stats in a file
#[allow(dead_code)]
pub fn save_subreddits_stats(stats: &HashMap<String, SubredditStats>, filepath: &str) {
    let stats = serde_json::to_string(&stats).unwrap();
    let mut file = File::create(filepath).unwrap();
    file.write(stats.as_bytes()).unwrap();
}

/// Load the subreddits stats from a file
#[allow(dead_code)]
pub fn load_subreddits_stats(filepath: &str) -> HashMap<String, SubredditStats> {
    let file = File::open(filepath).unwrap();
    serde_json::from_reader(file).unwrap()
}
