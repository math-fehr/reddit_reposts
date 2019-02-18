//! Get the possible fields taken by multiple JSON objects

use serde_json::{Value};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Different types a JSON value can take
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum ValueType {
    Null,
    Bool,
    Number,
    String,
}

/// Possible types taken by multiple JSON objects
#[derive(Debug)]
pub struct PossibleTypes {
    possible_primitive_types: HashSet<ValueType>,
    possible_fields: HashMap<String, PossibleTypes>,
}

impl PossibleTypes {
    /// Create a new set of possible types
    pub fn new() -> PossibleTypes {
        PossibleTypes {
            possible_primitive_types: HashSet::new(),
            possible_fields: HashMap::new(),
        }
    }

    /// Add a type to the list of types possibilities
    pub fn add_type(&mut self, val: Value) {
        match val {
            Value::Null => {
                self.possible_primitive_types.insert(ValueType::Null);
            }
            Value::Bool(_) => {
                self.possible_primitive_types.insert(ValueType::Bool);
            }
            Value::Number(_) => {
                self.possible_primitive_types.insert(ValueType::Number);
            }
            Value::String(_) => {
                self.possible_primitive_types.insert(ValueType::String);
            }
            Value::Array(_) => unimplemented!(),
            Value::Object(map) => {
                // Add null option for evey fields not contained by this example
                for (key, value) in self.possible_fields.iter_mut() {
                    //TODO only works if last element don't introduce new fields
                    if !map.contains_key(key) {
                        value.add_type(Value::Null);
                    }
                }
                for (key, value) in map.into_iter() {
                    if let Some(possible_vals_field) = self.possible_fields.get_mut(&key) {
                        possible_vals_field.add_type(value);
                    } else {
                        let mut possible_vals_field = PossibleTypes::new();
                        possible_vals_field.add_type(value);
                        self.possible_fields.insert(key, possible_vals_field);
                    }
                }
            }
        }
    }
}

/// Get the set of all possible types taken by JSON objects stored in the file
/// The file is expected to have one JSON object per line
#[allow(dead_code)]
pub fn get_all_possible_types(file: File) -> PossibleTypes {
    let f = BufReader::new(file);

    let mut possible_vals = PossibleTypes::new();
    for line in f.lines() {
        let line = line.unwrap();
        let result: Value = serde_json::from_str(&line).unwrap();
        possible_vals.add_type(result);
    }
    possible_vals
}
