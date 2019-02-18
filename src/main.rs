mod reddit_comment;
mod reddit_post;
mod edit_state;
mod possible_types;
mod read_files;

use crate::reddit_post::RedditPost;
use regex::Regex;
use std::collections::{HashSet, HashMap};
use crate::read_files::*;

#[allow(dead_code)]
fn get_url_regex() -> Regex {
    Regex::new(r"https?://(www.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b[-a-zA-Z0-9@:%_+.~#?&/=;]*").unwrap()
}

fn main() {
    let filepaths = vec!["datasets/RS_2011-02".to_string()];
    let item_iterator = JSONItemIterator::<RedditPost,_>::new(filepaths.into_iter());
    let mut map = HashMap::new();
    for post in item_iterator {
        let url = post.get_linked_url();
        if let Some(url) = url {
            if !map.contains_key(&url) {
                map.insert(url.to_string(), HashSet::new());
            }
            map.get_mut(&url).unwrap().insert(Box::new(post));
        }
    }
    for (key, set) in map {
        if set.len() > 1 {
            println!("{:#?}", key);
            println!("{:#?}", set);
        }
    }
}
