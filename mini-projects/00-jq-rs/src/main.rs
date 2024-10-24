mod filters;
mod format;

use clap::{App, Arg};
use serde_json::Value;
use std::fs;
use std::process;

fn main() {
    let matches = App::new("JQ Command Line Interface")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Processes JSON with various filters and formatting options")
        .arg(
            Arg::new("filter-string")
                .help("Filter string for JQ")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("path-to-json")
                .help("Path to the JSON file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("color-output")
                .long("color-output")
                .help("Enable color output")
                .takes_value(false),
        )
        .arg(
            Arg::new("monochrome-output")
                .long("monochrome-output")
                .help("Enable monochrome output")
                .takes_value(false),
        )
        .arg(
            Arg::new("sort-keys")
                .long("sort-keys")
                .help("Sort keys in the output")
                .takes_value(false),
        )
        .arg(
            Arg::new("indent")
                .long("indent")
                .help("Indentation level for pretty printing (default: 2, range: 0-7)")
                .takes_value(true)
                .default_value("2")
                .validator(|v| match v.parse::<u8>() {
                    Ok(num) if num <= 7 => Ok(()),
                    _ => Err(String::from("Indent must be a number between 0 and 7")),
                }),
        )
        .arg(
            Arg::new("compact-output")
                .long("compact-output")
                .help("Enable compact output")
                .takes_value(false),
        )
        .get_matches();

    let color_output = matches.is_present("color-output");
    let monochrome_output = matches.is_present("monochrome-output");
    let compact_output = matches.is_present("compact-output");
    let _sort_keys = matches.is_present("sort-keys");
    let indent = matches.value_of("indent").unwrap().parse::<u8>().unwrap();

    // Check for conflicting arguments
    if color_output && monochrome_output {
        eprintln!("Conflicting arguments: --color-output and --monochrome-output cannot be used together.");
        process::exit(1);
    }

    let filter_string = matches.value_of("filter-string").unwrap();
    let path_to_json = matches.value_of("path-to-json").unwrap();

    let json_data = fs::read_to_string(path_to_json).expect("Unable to read file");
    let parsed_json: Value = serde_json::from_str(&json_data).expect("Unable to parse JSON");

    // Split the filter string by the pipe operator and apply each filter sequentially
    let filters: Vec<&str> = filter_string.split('|').map(|s| s.trim()).collect();
    let mut result = parsed_json;

    for filter in filters {
        result = match filter {
            "." => filters::identity_filter(&result),
            "add" => filters::add(&result),
            "length" => filters::length(&result),
            _ if filter.starts_with("del(") && filter.ends_with(")") => {
                let key_or_index = &filter[4..filter.len() - 1];
                filters::del(&mut result, key_or_index)
            }
            _ if filter.starts_with("index(") && filter.ends_with(")") => {
                let index: usize = filter[6..filter.len() - 1].parse().expect("Invalid index");
                filters::array_index(&result, index)
            }
            _ if filter.starts_with("slice(") && filter.ends_with(")") => {
                let params: Vec<&str> = filter[6..filter.len() - 1].split(',').collect();
                let start: usize = params[0].parse().expect("Invalid start index");
                let end: usize = params[1].parse().expect("Invalid end index");
                filters::array_slice(&result, start, end)
            }
            _ if filter.starts_with("pipe(") && filter.ends_with(")") => {
                let params: Vec<&str> = filter[5..filter.len() - 1].split(',').collect();
                let key = params[0];
                let index: usize = params[1].parse().expect("Invalid index");
                filters::pipe(&result, key, index)
            }
            _ if filter.starts_with("iterator(") && filter.ends_with(")") => {
                let key = &filter[9..filter.len() - 1];
                filters::array_iterator(&result, key)
            }
            _ => {
                eprintln!("Unsupported filter: {}", filter);
                process::exit(1);
            }
        };
    }

    // Apply formatting options
    let output = if compact_output {
        format::compact_output(&result)
    } else if monochrome_output {
        format::monochrome_print(&result)
    } else {
        format::pretty_print(&result, indent, color_output)
    };

    println!("{}", output);
}
