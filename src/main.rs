mod data_analysis;
mod possible_types;
mod read_files;
mod reddit_post;
mod simplify_dataset;
mod subreddit_posts;
mod subreddit_stats;
mod utils;
mod subreddit_urls;

use crate::data_analysis::*;
use crate::read_files::*;
use crate::reddit_post::*;
use crate::utils::*;
use regex::Regex;
use subreddit_stats::*;

#[allow(dead_code)]
fn get_url_regex() -> Regex {
    Regex::new(
        r"https?://(www.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b[-a-zA-Z0-9@:%_+.~#?&/=;]*",
    )
    .unwrap()
}

fn get_it() -> impl Iterator<Item = RedditPost> + Clone {
    let filepaths = vec!["datasets/RS_2017-01_simp_url".to_string()];
    JSONItemIterator::new(filepaths.clone().into_iter())
}

fn write_kernel() {
    let stats = load_subreddits_stats("datasets/subreddit_stats_2017-01");
    let stats = get_most_popular_subreddits(100, stats);
    let best_subreddits: HashSet<_> = stats.clone().into_iter().map(|(s, _)| s).collect();
    println!("Got subreddits");
    let (time, links) =
        measure_time(|| get_links(get_it(), Some(&best_subreddits)));
    println!("Got links: {:?}", time);
    println!("{}", links.urls.len());
    let links_between_subreddits = get_shared_links_between_subreddits(links);
    let ppmi = compute_ppmi(links_between_subreddits);
    write_ppmi_for_python_plot("plot/kernel", &stats, &ppmi);
}

fn main() {
    let stats = compute_subreddits_stats(JSONItemIterator::new(vec!["datasets/RS_2017-01_simp".to_string()].into_iter()));
    save_subreddits_stats(&stats, "datasets/subreddit_stats_2017-01");
    //write_kernel();
    show_reposts_accross_subreddits();
}

#[allow(dead_code)]
fn show_reposts_accross_subreddits() {
    let n_subreddits = 100;
    let stats = load_subreddits_stats("datasets/subreddit_stats_2017-01");
    let stats = get_most_popular_subreddits(n_subreddits, stats);
    let best_subreddits: HashSet<_> = stats.clone().into_iter().map(|(s, _)| s).collect();
    println!("Got subreddits");
    let (time, links) =
        measure_time(|| get_links(get_it(), None));
    println!("Got links: {:?}", time);
    let (time, mut accross_subreddits) = measure_time(|| get_reposts_accross_subreddits(links));
    println!("Got reposts: {:?}", time);
    let best_subreddits_vec: Vec<_> = best_subreddits.into_iter().collect();
    accross_subreddits.project(best_subreddits_vec);

    let mut information_out = vec![0; n_subreddits];
    for (subreddit, hm) in accross_subreddits.n_reposts.iter() {
        information_out[*subreddit] = hm.iter().fold(0, |sum, (_, i)| sum + i);
    }
    let mut information_in = vec![0; n_subreddits];
    for (_, hm) in accross_subreddits.n_reposts {
        for (s_out, i) in hm {
            information_in[s_out] += i;
        }
    }
    let mut vec = vec![];
    for i in 0..n_subreddits {
        let subreddit = accross_subreddits.subreddits.get(i).unwrap();
        let stats = stats.get(subreddit).unwrap();
        let n_in = information_in[i] as f32;
        let n_out = information_out[i] as f32;
        let n_posts = stats.n_posts as f32;
        vec.push((subreddit, n_in / n_posts, n_out / n_posts, n_posts));
    }
    vec.sort_by(|(_, in1, _, _), (_, in2, _, _)| in1.partial_cmp(&in2).unwrap());
    println!("{:#?}", vec);
    vec.sort_by(|(_, _, out1, _), (_, _, out2, _)| out1.partial_cmp(&out2).unwrap());
    println!("{:#?}", vec);
}
