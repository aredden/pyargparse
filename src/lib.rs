use core::panic;
use pyo3::{prelude::*, types::PyDict};
use std::collections::{HashMap, HashSet};

enum ArgType {
    Boolean,
    String,
    Integer,
    Float,
    ListBoolean,
    ListInteger,
    ListFloat,
    BracedListString,
    BracedListInteger,
    BracedListFloat,
}

#[derive(Debug, Clone)]
struct ArgTypesCollection {
    booleans: HashMap<String, bool>,
    strings: HashMap<String, String>,
    integers: HashMap<String, i64>,
    floats: HashMap<String, f64>,
    list_strings: HashMap<String, Vec<String>>,
    list_integers: HashMap<String, Vec<i64>>,
    list_floats: HashMap<String, Vec<f64>>,
    list_booleans: HashMap<String, Vec<bool>>,
}

impl ArgTypesCollection {
    fn new() -> ArgTypesCollection {
        ArgTypesCollection {
            booleans: HashMap::new(),
            strings: HashMap::new(),
            integers: HashMap::new(),
            floats: HashMap::new(),
            list_strings: HashMap::new(),
            list_integers: HashMap::new(),
            list_floats: HashMap::new(),
            list_booleans: HashMap::new(),
        }
    }
}
fn can_parse_as_bool(value: &[String]) -> bool {
    value.iter().all(|v| v.parse::<bool>().is_ok())
}

fn can_parse_as_int(value: &[String]) -> bool {
    value.iter().all(|v| v.parse::<i64>().is_ok())
}

fn can_parse_as_float(value: &[String]) -> bool {
    value.iter().all(|v| v.parse::<f64>().is_ok())
}
fn parse_list_float(value: &[String]) -> Vec<f64> {
    value.iter()
        .map(|v| v.parse::<f64>())
        .collect::<Result<Vec<f64>, _>>()
        .unwrap_or_else(|e| panic!("Failed to parse float values: {:?}", e))
}

fn parse_list_int(value: &[String]) -> Vec<i64> {
    value.iter()
        .map(|v| v.parse::<i64>())
        .collect::<Result<Vec<i64>, _>>()
        .unwrap_or_else(|e| panic!("Failed to parse integer values: {:?}", e))
}

fn parse_list_bool(value: &[String]) -> Vec<bool> {
    value.iter()
        .map(|v| v.parse::<bool>())
        .collect::<Result<Vec<bool>, _>>()
        .unwrap_or_else(|e| panic!("Failed to parse boolean values: {:?}", e))
}
fn parse_list_string(value: &Vec<String>) -> Vec<String> {
    let mut string_vec: Vec<String> = Vec::new();
    for v in value {
        string_vec.push(v.to_owned());
    }
    string_vec.to_owned()
}

fn is_list_type(value: &Vec<String>) -> bool {
    let first = value.first().expect("List was empty!").trim();
    let last = value.last().expect("List was empty!").trim();
    first.starts_with("[") && last.ends_with("]")
}
fn remove_list_string_ends(value: &[String]) -> Vec<String> {
    value
        .join(" ")
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                Some(
                    trimmed
                        .trim_start_matches('[')
                        .trim_end_matches(']')
                        .to_owned(),
                )
            } else {
                None
            }
        })
        .collect()
}

fn parse(value: &Vec<String>) -> ArgType {
    let clean_values: Vec<String> = value
        .clone()
        .into_iter()
        .map(|s| s.trim().to_owned())
        .filter(|f| f.len() > 0)
        .collect();
    if is_list_type(&clean_values.clone()) {
        let string_removed_values = remove_list_string_ends(&clean_values);
        if can_parse_as_int(&string_removed_values.clone()) {
            return ArgType::BracedListInteger;
        } else if can_parse_as_float(&string_removed_values.clone()) {
            return ArgType::BracedListFloat;
        } else {
            return ArgType::BracedListString;
        }
    } else {
        if can_parse_as_bool(&clean_values.clone()) {
            if clean_values.len() == 1 {
                ArgType::Boolean
            } else {
                ArgType::ListBoolean
            }
        } else if can_parse_as_int(&clean_values.clone()) {
            if clean_values.len() == 1 {
                ArgType::Integer
            } else {
                ArgType::ListInteger
            }
        } else if can_parse_as_float(&clean_values.clone()) {
            if clean_values.len() == 1 {
                ArgType::Float
            } else {
                ArgType::ListFloat
            }
        } else {
            ArgType::String
        }
    }
}

fn parse_custom_command(
    command: &String,
    boolean_flags: HashSet<String>,
) -> Option<ArgTypesCollection> {
    let tokens: Vec<String> = command
        .split_whitespace()
        .to_owned()
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let mut arg_types = ArgTypesCollection::new();
    let mut current_values: Vec<String> = Vec::new();
    let mut has_current_key = false;
    let mut current_key = String::new();
    let mut args_string_collection: HashMap<String, Vec<String>> = HashMap::new();
    for token in tokens.to_owned().into_iter() {
        if token.starts_with("--") {
            if has_current_key {
                let clen = (&current_values.len()).clone();
                if clen >= 1 {
                    let mmap: Vec<String> = current_values
                        .clone()
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect();
                    args_string_collection.insert(current_key.to_owned(), mmap);
                    current_values.clear();
                }
            }
            current_key = token.clone()[2..].to_owned();

            if boolean_flags.contains(&current_key) {
                args_string_collection.insert(current_key.to_owned(), vec!["true".to_owned()]);
                has_current_key = false;
            } else {
                has_current_key = true;
            }
        } else {
            current_values.push(token.clone());
        }
    }
    if has_current_key {
        let clen = (&current_values.len()).clone();
        if clen >= 1 {
            let mmap: Vec<String> = current_values
                .into_iter()
                .map(|s| s.to_owned())
                .collect();
            args_string_collection.insert(current_key.to_owned(), mmap.clone());
        }
    }
    for (key, value) in args_string_collection.to_owned() {
        let arg_type = parse(&value);
        match arg_type {
            ArgType::Boolean => {
                let vals = parse_list_bool(&value);

                arg_types.booleans.insert(key, vals[0]);
            }
            ArgType::String => {
                let vals = value.join(" ").trim().to_owned();

                arg_types.strings.insert(key, vals);
            }
            ArgType::Integer => {
                let vals = parse_list_int(&value);

                arg_types
                    .integers
                    .insert(key, vals.first().unwrap().to_owned());
            }
            ArgType::Float => {
                let vals = parse_list_float(&value);

                arg_types
                    .floats
                    .insert(key, vals.first().unwrap().to_owned());
            }
            ArgType::ListBoolean => {
                let vals = parse_list_bool(&value);

                arg_types.list_booleans.insert(key, vals);
            }
            ArgType::ListInteger => {
                arg_types.list_integers.insert(key, parse_list_int(&value));
            }
            ArgType::ListFloat => {
                arg_types.list_floats.insert(key, parse_list_float(&value));
            }
            ArgType::BracedListString => {
                let unbraced = remove_list_string_ends(&value);
                arg_types.list_strings.insert(key, parse_list_string(&unbraced));
            }
            ArgType::BracedListInteger => {
                let unbraced = remove_list_string_ends(&value);
                arg_types.list_integers.insert(key, parse_list_int(&unbraced));
            }
            ArgType::BracedListFloat => {
                let unbraced = remove_list_string_ends(&value);
                arg_types.list_floats.insert(key, parse_list_float(&unbraced));
            }
        }
    }
    Some(arg_types)
}


#[pyfunction]
fn parse_command(
    _py: Python,
    command: String,
    boolean_flags: HashSet<String>,
) -> PyResult<&PyDict> {
    let arg_types_option = parse_custom_command(&command, boolean_flags);
    match arg_types_option {
        Some(arg_types) => {
            let dict = PyDict::new(_py);
            for (key, value) in arg_types.booleans {
                dict.set_item(key, value)?;
            }
            for (key, value) in arg_types.strings {
                dict.set_item(key, value)?;
            }
            for (key, value) in arg_types.integers {
                dict.set_item(key, value)?;
            }
            for (key, value) in arg_types.floats {
                dict.set_item(key, value)?;
            }
            for (key, value) in arg_types.list_strings {
                dict.set_item(key, value)?;
            }
            for (key, value) in arg_types.list_integers {
                dict.set_item(key, value)?;
            }
            for (key, value) in arg_types.list_floats {
                dict.set_item(key, value)?;
            }
            Ok(dict)
        }
        None => {
            panic!("Failed to parse command!");
        }
    }
}

#[pymodule]
fn rust_arg_parser(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_command, m)?)?;
    Ok(())
}
