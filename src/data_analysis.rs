//! Contains different functions that can give interesting information

#![allow(dead_code)]
use crate::reddit_post::*;
use crate::subreddit_stats::*;
use serde::{Deserialize, Serialize};
pub use std::collections::{HashMap, HashSet};
use std::io::prelude::*;

/// Structure representing posts grouped by urls
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubredditsFromUrls {
    pub urls: HashMap<String, Vec<(usize, i32)>>,
    pub subreddits: Vec<String>,
}

/// Get posts associated with the links
pub fn get_links<IT>(iterator: IT, subreddits: Option<&HashSet<String>>) -> SubredditsFromUrls
where
    IT: Iterator<Item = RedditPost>,
{
    let mut subreddits_vec = vec![];
    if let Some(subreddits) = subreddits {
        subreddits_vec = subreddits.iter().map(String::to_string).collect();
    }
    let mut subreddit_to_int: HashMap<_, _> = subreddits_vec
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, i))
        .collect();
    let mut map = HashMap::new();
    for post in iterator {
        if let Some(subreddits) = &subreddits {
            if !subreddits.contains(&post.subreddit) {
                continue;
            }
        }
        let url = post.get_linked_url();
        if let Some(url) = url {
            if !subreddit_to_int.contains_key(&post.subreddit) {
                subreddit_to_int.insert(post.subreddit.clone(), subreddits_vec.len());
                subreddits_vec.push(post.subreddit.clone());
            }
            let subreddit_id = *subreddit_to_int.get(&post.subreddit).unwrap();
            if !map.contains_key(&url) {
                map.insert(url.to_string(), vec![]);
            }
            map.get_mut(&url)
                .unwrap()
                .push((subreddit_id, post.created_utc));
        }
    }
    SubredditsFromUrls {
        urls: map,
        subreddits: subreddits_vec,
    }
}

/// Get links per subreddits
pub fn get_links_inside_subreddits<T, IT>(
    iterator: IT,
    subreddits: Option<HashSet<String>>,
) -> HashMap<String, HashMap<String, Vec<T>>>
where
    IT: Iterator<Item = RedditPost>,
    T: From<RedditPost>,
{
    let filter_subreddits = subreddits.is_some();
    let mut map = if filter_subreddits {
        subreddits
            .unwrap()
            .into_iter()
            .map(|subreddit| (subreddit, HashMap::new()))
            .collect()
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

/// Structure representing the number of reposts accross each pair of subreddits
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RepostsAcrossSubreddits {
    pub subreddits: Vec<String>,
    pub n_reposts: HashMap<usize, HashMap<usize, i32>>,
}

impl RepostsAcrossSubreddits {
    /// Project the data on a subset of subreddits
    pub fn project(&mut self, subreddits: Vec<String>) {
        let subreddits_to_int: HashMap<_, _> = subreddits
            .iter()
            .enumerate()
            .map(|(i, s)| (s.to_string(), i))
            .collect();
        self.n_reposts = self
            .n_reposts
            .clone()
            .into_iter()
            .filter(|(s, _)| subreddits_to_int.contains_key(&self.subreddits[*s]))
            .map(|(s, hm)| {
                let s = *subreddits_to_int.get(&self.subreddits[s]).unwrap();
                let hm = hm
                    .into_iter()
                    .filter(|(s, _)| subreddits_to_int.contains_key(&self.subreddits[*s]))
                    .map(|(s, v)| (*subreddits_to_int.get(&self.subreddits[s]).unwrap(), v))
                    .collect::<HashMap<_,_>>();
                (s, hm)
            })
            .filter(|(_, hm)| hm.len() > 0)
            .collect();
        self.subreddits = subreddits;
    }
}

/// Get the number of reposts accross subreddits.
/// This count the number of time a link was first posted in a subreddit,
/// then posted in another.
pub fn get_reposts_accross_subreddits(urls: SubredditsFromUrls) -> RepostsAcrossSubreddits {
    let mut subreddits_reposts = HashMap::new();
    for (_url, mut posts) in urls.urls.into_iter() {
        posts.sort_by(|post1, post2| post1.1.cmp(&post2.1));
        if posts.len() <= 1 {
            continue;
        }
        if !subreddits_reposts.contains_key(&posts[0].0) {
            subreddits_reposts.insert(posts[0].0, HashMap::new());
        }
        let map = subreddits_reposts.get_mut(&posts[0].0).unwrap();
        for j in 1..posts.len() {
            if posts[0].0 != posts[j].0 {
                if !map.contains_key(&posts[j].0) {
                    map.insert(posts[j].0, 1);
                } else {
                    let entry = map.get_mut(&posts[j].0).unwrap();
                    *entry += 1;
                }
            }
        }
    }
    RepostsAcrossSubreddits {
        subreddits: urls.subreddits,
        n_reposts: subreddits_reposts,
    }
}

/// Struct representing the number of urls shared between subreddits
pub struct UrlsBetweenSubreddits {
    subreddits: Vec<String>,
    n_shared_urls: HashMap<usize, HashMap<usize, u32>>,
}

/// Get the number of shared between subreddits.
/// This count the number of time a link was posted in two different subreddits.
pub fn get_shared_links_between_subreddits(urls: SubredditsFromUrls) -> UrlsBetweenSubreddits {
    let mut subreddits_links = HashMap::new();
    for (_url, posts) in urls.urls.into_iter() {
        if posts.len() <= 1 {
            continue;
        }
        let subreddits: HashSet<_> = posts.iter().map(|(s, _)| *s).collect();

        for subreddit1 in subreddits.iter() {
            if !subreddits_links.contains_key(subreddit1) {
                subreddits_links.insert(*subreddit1, HashMap::new());
            }
            let subreddit_links = subreddits_links.get_mut(subreddit1).unwrap();
            for subreddit2 in subreddits.iter() {
                if subreddit1 == subreddit2 {
                    continue;
                }
                if !subreddit_links.contains_key(subreddit2) {
                    subreddit_links.insert(*subreddit2, 0);
                }
                *subreddit_links.get_mut(subreddit2).unwrap() += 1;
            }
        }
    }
    UrlsBetweenSubreddits {
        subreddits: urls.subreddits,
        n_shared_urls: subreddits_links,
    }
}

/// Positive pointwise mutual information of the number of urls shared between the subreddits
pub struct UrlsPPMI {
    pub subreddits: Vec<String>,
    pub matrix: HashMap<usize, HashMap<usize, f32>>,
}

/// Compute the positive pointwise mutual information
pub fn compute_ppmi(urls: UrlsBetweenSubreddits) -> UrlsPPMI {
    let sum_col: HashMap<_, _> = urls
        .n_shared_urls
        .iter()
        .map(|(s, hm)| (s.clone(), hm.iter().fold(0f32, |s, (_, i)| s + *i as f32)))
        .collect();
    let sum_all = sum_col.iter().fold(0f32, |s, (_, i)| s + i);
    let matrix = urls
        .n_shared_urls
        .into_iter()
        .map(|(sub1, hm)| {
            let hm = hm
                .into_iter()
                .map(|(sub2, value)| {
                    let sum_col1 = sum_col.get(&sub1).unwrap();
                    let sum_col2 = sum_col.get(&sub2).unwrap();
                    let value = ((value as f32 * sum_all) / (sum_col1 * sum_col2))
                        .ln()
                        .max(0f32);
                    (sub2, value)
                })
                .collect();
            (sub1, hm)
        })
        .collect();
    UrlsPPMI {
        subreddits: urls.subreddits,
        matrix,
    }
}

/// Write the ppmi matrix in a file to plot it with python
pub fn write_ppmi_for_python_plot(
    filepath: &str,
    subreddit_stats: &HashMap<String, SubredditStats>,
    ppmi: &UrlsPPMI,
) {
    let n_subreddits = ppmi.subreddits.len();

    let file = std::fs::File::create(filepath).unwrap();
    let mut buf_writer = std::io::BufWriter::new(file);
    buf_writer
        .write_all(format!("{}\n", ppmi.subreddits.len()).as_bytes())
        .unwrap();
    for subreddit in ppmi.subreddits.iter() {
        buf_writer
            .write_all(format!("{} ", subreddit).as_bytes())
            .unwrap();
    }
    buf_writer.write_all("\n".as_bytes()).unwrap();
    for subreddit in ppmi.subreddits.iter() {
        let stats = subreddit_stats.get(subreddit).unwrap();
        let v = if stats.n_posts < 2 * stats.n_posts_over_18 {
            1
        } else {
            0
        };
        buf_writer.write_all(format!("{} ", v).as_bytes()).unwrap();
    }
    buf_writer.write_all("\n".as_bytes()).unwrap();
    for i in 0..n_subreddits {
        for j in 0..n_subreddits {
            let ppmi_val = ppmi
                .matrix
                .get(&i)
                .map_or(0f32, |col| *col.get(&j).unwrap_or(&0f32));
            buf_writer
                .write_all(format!("{} ", ppmi_val).as_bytes())
                .unwrap();
        }
        buf_writer.write_all("\n".as_bytes()).unwrap();
    }
}
