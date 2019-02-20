mod reddit_comment;
mod reddit_post;
mod edit_state;
mod possible_types;
mod read_files;
mod data_analysis;
mod utils;
mod subreddit_stats;
mod subreddit_posts;

use crate::reddit_post::RedditPost;
use regex::Regex;
use crate::read_files::*;
use crate::data_analysis::*;
use crate::utils::*;
use subreddit_stats::*;
use subreddit_posts::*;

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

fn get_it() -> impl Iterator<Item=RedditPost> + Clone {
    let filepaths = vec!["datasets/RS_2017-01".to_string()];
    RedditPostItemIterator::new(filepaths.clone().into_iter())
}

fn main() {
    /*let links = get_links::<SimpleRedditPost,_>(get_it().take(6_000_000));
    println!("Got links");
    let accross_subreddits = get_reposts_accross_subreddits(links);
    println!("Got reposts");
    let subreddits = compute_subreddits_stats(get_it().take(6_000_000));
    println!("Got subreddits");

    let information_out = accross_subreddits.clone().into_iter().map(|(s,hm)| (s, hm.into_iter().fold(0, |sum, (_,i)| sum + i))).collect::<HashMap<_,_>>();
    let mut information_in = HashMap::<String,_>::new();
    for (s_in, hm) in accross_subreddits {
        for (s_out, i) in hm {
            if !information_in.contains_key(&s_out) {
                information_in.insert(s_out.to_string(), i);
            } else {
                *information_in.get_mut(&s_out).unwrap() += 1;
            }
        }
    }
    let mut vec = vec![];
    for (subreddit, stats) in subreddits {
        let n_in = *information_in.get(&subreddit).unwrap_or(&0) as f32;
        let n_out = *information_out.get(&subreddit).unwrap_or(&0) as f32;
        let n_posts = stats.n_posts as f32;
        vec.push((subreddit, n_in / n_posts, n_out / n_posts, n_posts));
    }
    vec.sort_by(|(_, in1, _, _), (_, in2, _, _)| {
        in1.partial_cmp(&in2).unwrap()
    });
    println!("{:#?}", vec);
    vec.sort_by(|(_, _, out1, _), (_, _, out2, _)| {
        out1.partial_cmp(&out2).unwrap()
    });
    println!("{:#?}", vec);*/
}
