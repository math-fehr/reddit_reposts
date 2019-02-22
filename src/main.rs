mod data_analysis;
mod possible_types;
mod read_files;
mod reddit_post;
mod simplify_dataset;
mod subreddit_posts;
mod subreddit_stats;
mod utils;

use crate::data_analysis::*;
use crate::read_files::*;
use crate::reddit_post::*;
use regex::Regex;
use subreddit_stats::*;
use simplify_dataset::*;
use clap::{Arg, App, SubCommand};

#[allow(dead_code)]
fn get_url_regex() -> Regex {
    Regex::new(
        r"https?://(www.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b[-a-zA-Z0-9@:%_+.~#?&/=;]*",
    )
    .unwrap()
}

fn write_kernel<I: Iterator<Item = RedditPost>>(post_iterator: I, stats_filepath: &str, output_filepath: &str, n_subreddits: usize) {
    let stats = load_subreddits_stats(stats_filepath);
    let stats = get_most_popular_subreddits(n_subreddits, stats);
    let best_subreddits: HashSet<_> = stats.clone().into_iter().map(|(s, _)| s).collect();
    println!("Got subreddits");
    let links = get_links(post_iterator, Some(&best_subreddits));
    println!("Got links: {} links considered", links.urls.len());
    let links_between_subreddits = get_shared_links_between_subreddits(links);
    let ppmi = compute_ppmi(links_between_subreddits);
    println!("PPMI matrix computed");
    write_ppmi_for_python_plot(output_filepath, &stats, &ppmi);
    println!("PPMI matrix written");
}


fn main() {
    let matches = App::new("Reddit Repost")
        .subcommand(SubCommand::with_name("simplify")
                    .about("simplify a dataset")
                    .arg(Arg::with_name("INPUT")
                         .help("Set the input file path to simplify")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("KEEP_NON_URL_POSTS")
                         .help("Keep the post which don't contain url")
                         .default_value("true")
                         .index(2))
                    .arg(Arg::with_name("OUTPUT")
                         .help("Set the output file path")
                         .short("o")
                         .long("output")))
        .subcommand(SubCommand::with_name("substats")
                    .about("Computes general statistics of subreddits")
                    .arg(Arg::with_name("OUTPUT")
                         .help("Set the output file path")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("INPUTS")
                         .help("Set the input file paths to analyse")
                         .required(true)
                         .multiple(true)
                         .min_values(1)
                         .index(2)))
        .subcommand(SubCommand::with_name("ppmi")
                    .about("Computes the ppmi matrix found by comparing the shared links between subreddits")
                    .arg(Arg::with_name("OUTPUT")
                         .help("The output file that should be read by tsne.py")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("STATS_FILE")
                         .help("The file containing the subreddits stats")
                         .required(true)
                         .index(2))
                    .arg(Arg::with_name("N_SUBREDDITS")
                         .help("The number of subreddits to consider")
                         .required(true)
                         .index(3))
                    .arg(Arg::with_name("INPUTS")
                         .help("The input failes containing the posts in CSV format")
                         .required(true)
                         .multiple(true)
                         .min_values(1)
                         .index(4)))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("simplify") {
        let filepath = matches.value_of("INPUT").unwrap();
        let keep_non_url_posts: bool = matches.value_of("KEEP_NON_URL_POSTS").unwrap().parse().expect("Error: bool parameter expected in  KEEP_NON_URL_POSTS argument");
        let output_filepath_default = if keep_non_url_posts {
            filepath.to_string() + "_CSV"
        } else {
            filepath.to_string() + "_CSV_url"
        };
        let output_filepath = matches.value_of("OUTPUT").unwrap_or(&output_filepath_default);
        simplify_post_dataset(filepath, &output_filepath, keep_non_url_posts);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("substats") {
        let filepaths: Vec<_> = matches.values_of("INPUTS").unwrap().collect();
        let output_filepath = matches.value_of("OUTPUT").unwrap();
        let it = CSVItemIterator::<RedditPost,_>::new(filepaths.into_iter().map(|s| s.to_string()));
        let stats = compute_subreddits_stats(it);
        save_subreddits_stats(&stats, output_filepath);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("ppmi") {
        let output_filepath = matches.value_of("OUTPUT").unwrap();
        let stats_filepath = matches.value_of("STATS_FILE").unwrap();
        let inputs_filepath: Vec<_> = matches.values_of("INPUTS").unwrap().collect();
        let n_subreddits: usize = matches.value_of("N_SUBREDDITS").unwrap().parse().unwrap();
        let it = CSVItemIterator::<RedditPost,_>::new(inputs_filepath.into_iter().map(|s| s.to_string()));
        write_kernel(it, stats_filepath, output_filepath, n_subreddits);
        return;
    }
}

/*
#[allow(dead_code)]
fn show_reposts_accross_subreddits() {
    let n_subreddits = 100;
    let stats = load_subreddits_stats("datasets/subreddit_stats_2017-01");
    let stats = get_most_popular_subreddits(n_subreddits, stats);
    let best_subreddits: HashSet<_> = stats.clone().into_iter().map(|(s, _)| s).collect();
    println!("Got subreddits");
    let (time, links) = measure_time(|| get_links(get_it(), None));
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
}*/
