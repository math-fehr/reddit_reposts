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

impl<S, FPI> Clone for JSONItemIterator<S, FPI>
where
    FPI: Iterator<Item = String> + Clone,
    S: DeserializeOwned,
{
    fn clone(&self) -> Self {
        assert!(self.current_reader.is_none());
        JSONItemIterator {
            filepath_iterator: self.filepath_iterator.clone(),
            current_reader: None,
            json_struct_type: self.json_struct_type,
        }
    }
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
                return Some(serde_json::from_slice::<S>(line.as_bytes()).unwrap());
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
