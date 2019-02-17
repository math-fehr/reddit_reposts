mod reddit_comment;
mod reddit_post;
mod edit_state;
mod possible_types;

use crate::possible_types::*;
use crate::reddit_comment::RedditComment;
use crate::reddit_post::RedditPost;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let f = File::open("datasets/RC_2011-02").unwrap();
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.unwrap();
        let result = serde_json::from_str::<RedditComment>(&line);
        if result.is_err() {
            println!("{}", line);
        }
        result.unwrap();
    }
}
