//! Contains an iterator over JSON-parsable structs residing in different files

use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::marker::PhantomData;
use crate::reddit_post::RedditPost;
use std::borrow::Cow;

/// An iterator iterating through multiple files,
/// to deserialize JSON objects into a given struct
pub struct JSONItemIterator<S, FPI>
where
    FPI: Iterator<Item = String>,
    S: DeserializeOwned,
{
    filepath_iterator: FPI,
    current_reader: Option<Lines<BufReader<File>>>,
    json_struct_type: std::marker::PhantomData<S>,
}

impl<S, FPI> JSONItemIterator<S, FPI>
where
    FPI: Iterator<Item = String>,
    S: DeserializeOwned,
{
    /// Create a new iterator, given an iterator over file paths
    #[allow(dead_code)]
    pub fn new(filepath_iterator: FPI) -> Self {
        Self {
            filepath_iterator,
            current_reader: None,
            json_struct_type: PhantomData,
        }
    }
}

impl<S, FPI> Iterator for JSONItemIterator<S, FPI>
where
    FPI: Iterator<Item = String>,
    S: DeserializeOwned,
{
    type Item = S;

    fn next(&mut self) -> Option<S> {
        if let Some(reader) = &mut self.current_reader {
            if let Some(line) = reader.next() {
                let line = line.unwrap();
                return Some(serde_json::from_str::<S>(&line).unwrap());
            }
        }
        if let Some(filepath) = self.filepath_iterator.next() {
            let file = File::open(filepath).unwrap();
            let buf_reader = BufReader::new(file);
            self.current_reader = Some(buf_reader.lines());
            self.next()
        } else {
            None
        }
    }
}

/// An iterator iterating through multiple files,
/// to deserialize Reddit Posts JSON objects
pub struct RedditPostItemIterator<FPI>
where
    FPI: Iterator<Item = String>,
{
    filepath_iterator: FPI,
    current_reader: Option<Lines<BufReader<File>>>,
}

impl<FPI> Clone for RedditPostItemIterator<FPI>
where FPI: Iterator<Item = String> + Clone {
    fn clone(&self) -> Self {
        assert!(self.current_reader.is_none());
        RedditPostItemIterator {
            filepath_iterator: self.filepath_iterator.clone(),
            current_reader: None,
        }
    }
}

impl<FPI> RedditPostItemIterator<FPI>
where
    FPI: Iterator<Item = String>,
{
    /// Create a new iterator, given an iterator over file paths
    pub fn new(filepath_iterator: FPI) -> Self {
        Self {
            filepath_iterator,
            current_reader: None,
        }
    }
}

impl<FPI> Iterator for RedditPostItemIterator<FPI>
where
    FPI: Iterator<Item = String>,
{
    type Item = RedditPost;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(reader) = &mut self.current_reader {
            if let Some(line) = reader.next() {
                let line = line.unwrap();
                let json_struct = serde_json::from_str::<RedditPostJSON>(&line);
                if json_struct.is_err() {
                    println!("Reddit Post JSON: {}", line);
                    println!("Error while parsing JSON: {}", json_struct.unwrap_err());
                    panic!();
                }
                let json_struct = json_struct.unwrap();
                if let Some(post) = json_struct.into_reddit_post() {
                    return Some(post);
                } else {
                    return self.next();
                }
            }
        }
        if let Some(filepath) = self.filepath_iterator.next() {
            let file = File::open(filepath).unwrap();
            let buf_reader = BufReader::new(file);
            self.current_reader = Some(buf_reader.lines());
            self.next()
        } else {
            None
        }
    }
}

/// A struct representing a typed json reddit post.
#[derive(Deserialize, Debug, Clone)]
struct RedditPostJSON<'a> {
    #[serde(borrow)]
    pub href_url: Option<Cow<'a,str>>,
    pub num_comments: i32,
    #[serde(borrow)]
    pub promoted_url: Option<Cow<'a,str>>,
    pub score: i32,
    pub hidden: bool,
    pub gilded: i32,
    #[serde(borrow)]
    pub subreddit: Option<Cow<'a,str>>,
    pub id: &'a str,
    #[serde(borrow)]
    pub original_link: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub title: Cow<'a,str>,
    pub is_self: bool,
    #[serde(borrow)]
    pub selftext: Cow<'a,str>,
    #[serde(borrow)]
    pub domain: Cow<'a,str>,
    #[serde(borrow)]
    pub url: Cow<'a, str>,
    pub over_18: bool,
    pub author_cakeday: Option<bool>,
    #[serde(borrow)]
    pub permalink: Cow<'a, str>,
    pub author: &'a str,
    pub subreddit_id: Option<&'a str>,
    pub created_utc: i32,
}

fn str_to_opt_string(opt: Option<&str>) -> Option<String> {
    match opt {
        None => None,
        Some(s) => Some(s.to_string())
    }
}

fn cow_to_opt_string<'a>(opt: Option<Cow<'a, str>>) -> Option<String> {
    match opt {
        None => None,
        Some(s) => Some(s.to_string())
    }
}

impl<'a> RedditPostJSON<'a> {
    pub fn into_reddit_post(self) -> Option<RedditPost> {
        if self.subreddit_id.is_none() || self.subreddit.is_none() {
            None
        } else {
            Some(RedditPost{
                href_url: cow_to_opt_string(self.href_url),
                num_comments: self.num_comments,
                promoted_url: cow_to_opt_string(self.promoted_url),
                score: self.score,
                gilded: self.gilded,
                subreddit: self.subreddit.unwrap().to_string(),
                id: self.id.to_string(),
                original_link: cow_to_opt_string(self.original_link),
                title: self.title.to_string(),
                is_self: self.is_self,
                selftext: self.selftext.to_string(),
                domain: self.domain.to_string(),
                url: self.url.to_string(),
                over_18: self.over_18,
                author_cakeday: self.author_cakeday,
                permalink: self.permalink.to_string(),
                author: self.author.to_string(),
                subreddit_id: self.subreddit_id.unwrap().to_string(),
                created_utc: self.created_utc,
        })
        }
    }
}

