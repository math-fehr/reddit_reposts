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
use subreddit_stats::*;
use simplify_dataset::*;
use clap::{Arg, App, SubCommand};

/// Compute and write to a file the PPMI matrix
fn write_ppmi_matrix<I: Iterator<Item = RedditPost>>(post_iterator: I, stats_filepath: &str, output_filepath: &str, n_subreddits: usize) {
    let stats = load_subreddits_stats(stats_filepath);
    let stats = get_most_popular_subreddits(n_subreddits, stats);
    let best_subreddits: HashSet<_> = stats.clone().into_iter().map(|(s, _)| s).collect();
    println!("Got subreddits");
    let urls = get_urls(post_iterator, Some(&best_subreddits));
    println!("Got urls: {} urls considered", urls.urls.len());
    let urls_between_subreddits = get_shared_urls_between_subreddits(urls);
    let ppmi = compute_ppmi(urls_between_subreddits);
    println!("PPMI matrix computed");
    write_ppmi_for_python_plot(output_filepath, &stats, &ppmi);
    println!("PPMI matrix written");
}

/// Get the subreddits stats for some subreddits, by loading from a file the stats
fn get_subreddit_stats(stats_filepath: &str, subreddits: Vec<&str>) {
    let stats = load_subreddits_stats(stats_filepath);
    for subreddit in subreddits {
        if let Some(subreddit_stats) = stats.get(subreddit) {
            println!("Subreddit {} :\n{:#?}", subreddit, subreddit_stats);
        } else {
            println!("Subreddit {} :\nNot found!", subreddit);
        }
    }
}

/// Get the reposts that comes from and to a particular subreddit.
fn get_reposts(subreddit: &str, inputs_filepath: Vec<&str>) {
    let it = CSVItemIterator::<RedditPost,_>::new(inputs_filepath.clone().into_iter().map(|s| s.to_string()));
    let mut subreddit_singleton = HashSet::new();
    subreddit_singleton.insert(subreddit.to_string());
    println!("Fetching urls...");
    let mut urls = get_urls(it, Some(&subreddit_singleton));
    println!("Subreddit urls fetched!");
    let it = CSVItemIterator::<RedditPost,_>::new(inputs_filepath.clone().into_iter().map(|s| s.to_string()));
    println!("Fetching other surbeddits...");
    get_posts_with_urls(it, &mut urls);
    println!("Other subreddits found");
    let reposts_stats = get_reposts_stats(subreddit, &urls).sort(10).display(urls.subreddits);
    println!("{:#?}", reposts_stats);
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
        .subcommand(SubCommand::with_name("compute_stats")
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
                    .about("Computes the ppmi matrix found by comparing the shared urls between subreddits")
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
        .subcommand(SubCommand::with_name("get_stats")
                    .about("Get the stats of a subreddit from a previously pre-computed file")
                    .arg(Arg::with_name("STATS_FILE")
                         .help("The stats file computed by the compute_subs_stats command")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("SUBREDDITS")
                         .help("The name of the subreddits")
                         .required(true)
                         .index(2)
                         .multiple(true)))
        .subcommand(SubCommand::with_name("get_reposts")
                    .about("Get the number of post reposted by the sub, and by other subs over a url sent first on that sub")
                    .arg(Arg::with_name("SUBREDDIT")
                         .help("The name of the subreddit to analyse")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("INPUTS")
                         .help("The dataset files created by the simplify command")
                         .required(true)
                         .index(2)
                         .min_values(1)
                         .multiple(true)))
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

    if let Some(matches) = matches.subcommand_matches("compute_stats") {
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
        write_ppmi_matrix(it, stats_filepath, output_filepath, n_subreddits);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("get_stats") {
        let stats_filepath = matches.value_of("STATS_FILE").unwrap();
        let subreddits: Vec<_> = matches.values_of("SUBREDDITS").unwrap().collect();
        get_subreddit_stats(stats_filepath, subreddits);
        return;
    }

    if let Some(matches) = matches.subcommand_matches("get_reposts") {
        let subreddit = matches.value_of("SUBREDDIT").unwrap();
        let inputs_filepath = matches.values_of("INPUTS").unwrap().collect();
        get_reposts(subreddit, inputs_filepath);
    }
}
