//! Contains different functions that can give interesting information

#![allow(dead_code)]
use crate::reddit_post::*;
pub use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use crate::subreddit_stats::*;
use std::io::prelude::*;

/// Get posts associated with the links
pub fn get_links<T, IT>(iterator: IT, subreddits: Option<&HashSet<String>>) -> HashMap<String, HashSet<T>>
where
    IT: Iterator<Item = RedditPost>,
    T: From<RedditPost> + Hash + Eq
{
    let mut map = HashMap::new();
    for post in iterator {
        if let Some(subreddits) = &subreddits {
            if !subreddits.contains(&post.subreddit) {
                continue;
            }
        }
        let url = post.get_linked_url();
        if let Some(url) = url {
            if !map.contains_key(&url) {
                map.insert(url.to_string(), HashSet::new());
            }
            map.get_mut(&url).unwrap().insert(T::from(post));
        }
    }
    map
}

/// Get links per subreddits
pub fn get_links_inside_subreddits<T, IT>(iterator: IT, subreddits: Option<HashSet<String>>) -> HashMap<String, HashMap<String, Vec<T>>>
where
    IT: Iterator<Item = RedditPost>,
    T: From<RedditPost>,
{
    let filter_subreddits = subreddits.is_some();
    let mut map = if filter_subreddits {
        subreddits.unwrap().into_iter().map(|subreddit| (subreddit, HashMap::new())).collect()
    } else {
        HashMap::new()
    };
    for post in iterator {
        let url = post.get_linked_url();
        if let Some(url) = url {
            let subreddit = post.subreddit.clone();
            if !map.contains_key(&subreddit) {
                if filter_subreddits {
                    continue;
                } else {
                    map.insert(subreddit.clone(), HashMap::new());
                }
            }
            let subreddit_links = map.get_mut(&subreddit).unwrap();
            if !subreddit_links.contains_key(&url) {
                subreddit_links.insert(url.clone(), vec![T::from(post)]);
            } else {
                let entry = subreddit_links.get_mut(&url).unwrap();
                entry.push(T::from(post));
            }
        }
    }
    map
}

pub trait HasCreationDate {
    fn get_creation_date(&self) -> i32;
}

impl HasCreationDate for RedditPost {
    fn get_creation_date(&self) -> i32 {
        self.created_utc
    }
}

impl<T:HasCreationDate> HasCreationDate for Box<T> {
    fn get_creation_date(&self) -> i32 {
        (**self).get_creation_date()
    }
}

pub trait HasSubreddit {
    fn get_subreddit(&self) -> &str;
}

impl HasSubreddit for RedditPost {
    fn get_subreddit(&self) -> &str {
        &self.subreddit
    }
}

impl<T:HasSubreddit> HasSubreddit for Box<T> {
    fn get_subreddit(&self) -> &str {
        (**self).get_subreddit()
    }
}

/// Get the number of reposts accross subreddits.
/// This count the number of time a link was first posted in a subreddit,
/// then posted in another.
pub fn get_reposts_accross_subreddits<T>(links: HashMap<String, HashSet<T>>) -> HashMap<String, HashMap<String, i32>> where
                                         T: HasSubreddit + HasCreationDate + Eq + Hash {
    let mut subreddits_reposts = HashMap::new();
    for (_url, posts) in links.into_iter() {
        let mut posts = posts.into_iter().collect::<Vec<_>>();
        posts.sort_by(|post1, post2| {
            post1.get_creation_date().cmp(&post2.get_creation_date())
        });
        if posts.len() <= 1 {
            continue;
        }
        if !subreddits_reposts.contains_key(posts[0].get_subreddit()) {
            subreddits_reposts.insert(posts[0].get_subreddit().to_string(), HashMap::new());
        }
        let map = subreddits_reposts.get_mut(posts[0].get_subreddit()).unwrap();
        for j in 1..posts.len() {
            if posts[0].get_subreddit() != posts[j].get_subreddit() {
                if !map.contains_key(posts[j].get_subreddit()) {
                    map.insert(posts[j].get_subreddit().to_string(), 1);
                } else {
                    let entry = map.get_mut(posts[j].get_subreddit()).unwrap();
                    *entry += 1;
                }
            }
        }
    }
    subreddits_reposts
}

/// Get the number of shared between subreddits.
/// This count the number of time a link was posted in two different subreddits.
pub fn get_shared_links_between_subreddits<T>(links: HashMap<String, HashSet<T>>) -> HashMap<String, HashMap<String, i32>> where
                                         T: HasSubreddit + HasCreationDate + Eq + Hash {
    let mut subreddits_links = HashMap::new();
    for (_url, posts) in links.into_iter() {
        if posts.len() <= 1 {
            continue;
        }
        let subreddits: HashSet<_> = posts.iter().map(|p| p.get_subreddit()).collect();

        for subreddit1 in subreddits.iter() {
            if !subreddits_links.contains_key(*subreddit1) {
                subreddits_links.insert(subreddit1.to_string(), HashMap::new());
            }
            let subreddit_links = subreddits_links.get_mut(*subreddit1).unwrap();
            for subreddit2 in subreddits.iter() {
                if subreddit1 == subreddit2 {
                    continue;
                }
                if !subreddit_links.contains_key(*subreddit2) {
                    subreddit_links.insert(subreddit2.to_string(), 0);
                }
                *subreddit_links.get_mut(*subreddit2).unwrap() += 1;
            }
        }
    }
    subreddits_links
}

/// Compute the positive pointwise mutual information
pub fn compute_ppmi(links: HashMap<String, HashMap<String, i32>>) -> HashMap<String, HashMap<String, f32>> {
    let sum_col: HashMap<_,_> = links.iter().map(|(s,hm)| (s.clone(), hm.iter().fold(0f32, |s,(_,i)| s + *i as f32))).collect();
    let sum_all = sum_col.iter().fold(0f32, |s,(_,i)| s + i);
    links.into_iter().map(|(sub1, hm)| {
        let hm = hm.into_iter().map(|(sub2, value)| {
            let sum_col1 = sum_col.get(&sub1).unwrap();
            let sum_col2 = sum_col.get(&sub2).unwrap();
            let value = ((value as f32 * sum_all) / (sum_col1 * sum_col2)).ln().max(0f32);
            (sub2, value)
        }).collect();
        (sub1, hm)
    }).collect()
}

pub fn write_ppmi_for_python_plot(filepath: &str, subreddit_stats: &HashMap<String, SubredditStats>, ppmi: &HashMap<String, HashMap<String, f32>>) {
    let subreddit_vec: Vec<_> = subreddit_stats.iter().map(|(s,_)| s).collect();
    let mut ppmi_vec = vec![vec![0f32;100];100];
    for i in 0..100 {
        for j in 0..100 {
            let sub_i = subreddit_vec[i];
            let sub_j = subreddit_vec[j];
            if let Some(hm) = ppmi.get(sub_i) {
                if let Some(v) = hm.get(sub_j) {
                    ppmi_vec[i][j] = *v;
                } else {
                    ppmi_vec[i][j] = 0f32;
                }
            } else {
                ppmi_vec[i][j] = 0f32;
            }
        }
    }

    let file = std::fs::File::create(filepath).unwrap();
    let mut buf_writer = std::io::BufWriter::new(file);
    buf_writer.write_all(format!("{}\n", subreddit_vec.len()).as_bytes()).unwrap();
    for subreddit in subreddit_vec.iter() {
        buf_writer.write_all(format!("{} ", subreddit).as_bytes()).unwrap();
    }
    buf_writer.write_all("\n".as_bytes()).unwrap();
    for subreddit in subreddit_vec.iter() {
        let stats = subreddit_stats.get(*subreddit).unwrap();
        let v =  if stats.n_posts < 2 * stats.n_posts_over_18 {
            1
        } else {
            0
        };
        buf_writer.write_all(format!("{} ", v).as_bytes()).unwrap();
    }
    buf_writer.write_all("\n".as_bytes()).unwrap();
    for i in 0..100 {
        for j in 0..100 {
            buf_writer.write_all(format!("{} ", ppmi_vec[i][j]).as_bytes()).unwrap();
        }
        buf_writer.write_all("\n".as_bytes()).unwrap();
    }
}
