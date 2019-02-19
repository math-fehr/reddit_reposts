//! Contains an iterator over JSON-parsable structs residing in different files

use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};
use std::marker::PhantomData;

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
    S: DeserializeOwned + std::fmt::Debug,
{
    type Item = S;

    fn next(&mut self) -> Option<S> {
        if let Some(reader) = &mut self.current_reader {
            if let Some(line) = reader.next() {
                let line = line.unwrap();
                let json = serde_json::from_str::<S>(&line);
                if json.is_err() {
                    println!("While parsing JSON {}", line);
                    println!("Error found: {}", json.unwrap_err());
                    panic!();
                } else {
                    return Some(json.unwrap());
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
