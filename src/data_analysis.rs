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

/// Get posts associated with urls
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

/// Get posts associated with the urls given as input
pub fn get_posts_with_urls<IT>(iterator: IT, urls: &mut SubredditsFromUrls)
where IT: Iterator<Item = RedditPost>
{
    let mut subreddit_to_int: HashMap<_,_> = urls.subreddits.iter().enumerate().map(|(i,s)| (s.to_string(),i)).collect();
    let original_subreddits: HashSet<_> = subreddit_to_int.keys().map(String::to_string).collect();

    for post in iterator {
        if original_subreddits.contains(&post.subreddit) {
            continue;
        }
        let url = post.get_linked_url();
        if let Some(url) = url {
            if !urls.urls.contains_key(&url) {
                continue;
            }
            if !subreddit_to_int.contains_key(&post.subreddit) {
                subreddit_to_int.insert(post.subreddit.clone(), urls.subreddits.len());
                urls.subreddits.push(post.subreddit.clone());
            }
            let subreddit_id = *subreddit_to_int.get(&post.subreddit).unwrap();
            urls.urls.get_mut(&url).unwrap().push((subreddit_id, post.created_utc));
        }
    }
}

/// Reposts stats for a subreddit
#[derive(Clone, Debug)]
pub struct RepostStats {
    pub n_posts: u32,
    pub n_reposts_from_others: u32,
    pub n_reposted_by_others: u32,
    pub n_reposts_from_self: u32,
    pub reposts_from_others: HashMap<usize, u32>,
    pub reposts_by_others: HashMap<usize, u32>,
}

impl RepostStats {
    /// Sort the reposts_* fields, and keep only the most significants elements
    pub fn sort(self, n_samples: usize) -> RepostStatsSorted {
        let mut reposts_from_others: Vec<_> = self.reposts_from_others.into_iter().collect();
        reposts_from_others.sort_by(|(_,i1), (_,i2)| {
            i2.cmp(&i1)
        });
        let reposts_from_others = reposts_from_others.into_iter().take(n_samples).collect();
        let mut reposts_by_others: Vec<_> = self.reposts_by_others.into_iter().collect();
        reposts_by_others.sort_by(|(_,i1), (_,i2)| {
            i2.cmp(&i1)
        });
        let reposts_by_others = reposts_by_others.into_iter().take(n_samples).collect();
        RepostStatsSorted {
            n_posts: self.n_posts,
            n_reposts_from_others: self.n_reposts_from_others,
            n_reposted_by_others: self.n_reposted_by_others,
            n_reposts_from_self: self.n_reposts_from_self,
            reposts_from_others,
            reposts_by_others,
        }
    }
}

/// Reposts stats for a subreddit, where the reposts_* fields are sorted
#[derive(Clone, Debug)]
pub struct RepostStatsSorted {
    pub n_posts: u32,
    pub n_reposts_from_others: u32,
    pub n_reposted_by_others: u32,
    pub n_reposts_from_self: u32,
    pub reposts_from_others: Vec<(usize, u32)>,
    pub reposts_by_others: Vec<(usize, u32)>,
}

impl RepostStatsSorted {
    pub fn display(self, subreddits: Vec<String>) -> RepostStatsSortedDisplay {
        let reposts_from_others = self.reposts_from_others.into_iter().map(|(s,i)| (subreddits[s].to_string(), i)).collect();
        let reposts_by_others = self.reposts_by_others.into_iter().map(|(s,i)| (subreddits[s].to_string(), i)).collect();
        RepostStatsSortedDisplay {
            n_posts: self.n_posts,
            n_reposts_from_others: self.n_reposts_from_others,
            n_reposted_by_others: self.n_reposted_by_others,
            n_reposts_from_self: self.n_reposts_from_self,
            reposts_from_others,
            reposts_by_others,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RepostStatsSortedDisplay {
    pub n_posts: u32,
    pub n_reposts_from_others: u32,
    pub n_reposted_by_others: u32,
    pub n_reposts_from_self: u32,
    pub reposts_from_others: Vec<(String, u32)>,
    pub reposts_by_others: Vec<(String, u32)>,
}

/// Get reposts statistics for a subreddit
pub fn get_reposts_stats(subreddit: &str, urls: &SubredditsFromUrls) -> RepostStats {
    let mut n_reposts_from_others = 0;
    let mut n_reposted_by_others = 0;
    let mut n_reposts_from_self = 0;
    let mut n_posts = 0;

    let subreddit_id = urls.subreddits.iter().enumerate().find(|(_,s)| *s == subreddit).unwrap().0;

    let mut reposts_by_others = HashMap::new();
    let mut reposts_from_others = HashMap::new();
    for (_, posts) in urls.urls.iter() {
        let mut posts = posts.clone();
        posts.sort_by(|(_,utc1), (_,utc2)| {
            utc1.cmp(utc2)
        });
        if posts[0].0 != subreddit_id {
            n_reposts_from_self += 1;
            n_posts += 1;
        }
        let mut has_posted = posts[0].0 == subreddit_id;
        let mut already_seen = HashSet::new();
        for i in 1..posts.len() {
            if posts[i].0 != subreddit_id && already_seen.contains(&posts[i].0) {
                continue;
            }
            if posts[i].0 != subreddit_id {
                already_seen.insert(posts[i].0);
            }
            if posts[0].0 == subreddit_id {
                n_posts += 1;
            }
            if posts[0].0 == subreddit_id && posts[i].0 != subreddit_id {
                n_reposted_by_others += 1;
                *reposts_by_others.entry(posts[i].0).or_insert(0) += 1;
            }
            if !has_posted && posts[0].0 != subreddit_id && posts[i].0 == subreddit_id {
                n_reposts_from_others += 1;
                *reposts_from_others.entry(posts[0].0).or_insert(0) += 1;
            }
            if posts[i].0 == subreddit_id {
                has_posted = true;
            }
        }
    }

    RepostStats {
        n_posts,
        n_reposts_from_self,
        n_reposted_by_others,
        n_reposts_from_others,
        reposts_from_others,
        reposts_by_others,
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
