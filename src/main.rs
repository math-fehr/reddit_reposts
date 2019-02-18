mod reddit_comment;
mod reddit_post;
mod edit_state;
mod possible_types;
mod read_files;
mod data_analysis;

use crate::reddit_post::RedditPost;
use regex::Regex;
use crate::read_files::*;
use crate::data_analysis::*;

#[allow(dead_code)]
fn get_url_regex() -> Regex {
    Regex::new(r"https?://(www.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b[-a-zA-Z0-9@:%_+.~#?&/=;]*").unwrap()
}

fn main() {
    let filepaths = vec!["datasets/RS_2011-02".to_string()];
    let item_iterator = JSONItemIterator::<RedditPost,_>::new(filepaths.clone().into_iter());
    let map = get_links::<Box<RedditPost>, _>(item_iterator);
    let map = get_reposts_accross_subreddits(map);
    println!("{:#?}", map);
}
