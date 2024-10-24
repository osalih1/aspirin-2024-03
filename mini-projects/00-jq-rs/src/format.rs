use colored::*;
use regex::Regex;
use serde_json::{Map, Value};
use std::env;

pub fn sort_keys(json: &Value) -> Value {
    match json {
        Value::Object(map) => {
            let mut sorted_map = Map::new();
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            for key in keys {
                sorted_map.insert(key.clone(), sort_keys(&map[key]));
            }
            Value::Object(sorted_map)
        }
        Value::Array(arr) => {
            let sorted_arr: Vec<Value> = arr.iter().map(sort_keys).collect();
            Value::Array(sorted_arr)
        }
        _ => json.clone(),
    }
}

pub fn pretty_print(json: &Value, indent: u8, color_output: bool) -> String {
    let mut json_str = serde_json::to_string_pretty(json).unwrap();
    if color_output {
        json_str = apply_colors(&json_str);
    }
    json_str.replace("\n", &format!("\n{}", " ".repeat(indent as usize)))
}

pub fn compact_output(json: &Value) -> String {
    serde_json::to_string(json).unwrap()
}

fn apply_colors(json_str: &str) -> String {
    let jq_colors = env::var("JQ_COLORS")
        .unwrap_or_else(|_| "0;90:0;37:0;37:0;37:0;32:1;37:1;37:1;34".to_string());
    let colors: Vec<&str> = jq_colors.split(':').collect();
    let re = Regex::new(r"\d+").unwrap();

    let json_str = json_str
        .replace("null", &"null".color(colors[0]).to_string())
        .replace("false", &"false".color(colors[1]).to_string())
        .replace("true", &"true".color(colors[2]).to_string())
        .replace("\"", &"\"".color(colors[4]).to_string())
        .replace("[", &"[".color(colors[5]).to_string())
        .replace("]", &"]".color(colors[5]).to_string())
        .replace("{", &"{".color(colors[6]).to_string())
        .replace("}", &"}".color(colors[6]).to_string())
        .replace(":", &":".color(colors[7]).to_string());

    re.replace_all(&json_str, |caps: &regex::Captures| {
        caps[0].color(colors[3]).to_string()
    })
    .to_string()
}

pub fn monochrome_print(json: &Value) -> String {
    serde_json::to_string_pretty(json).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sort_keys() {
        let input = json!({"b": 1, "a": 2});
        let output = sort_keys(&input);
        assert_eq!(output, json!({"a": 2, "b": 1}));
    }

    #[test]
    fn test_pretty_print() {
        let input = json!({"key": "value"});
        let output = pretty_print(&input, 2, false);
        assert_eq!(output, "{\n  \"key\": \"value\"\n}");
    }

    #[test]
    fn test_pretty_print_with_indent() {
        let input = json!({"key": "value"});
        let output = pretty_print(&input, 4, false);
        assert_eq!(output, "{\n    \"key\": \"value\"\n}");
    }

    #[test]
    fn test_pretty_print_with_color() {
        env::set_var("JQ_COLORS", "0;90:0;37:0;37:0;37:0;32:1;37:1;37:1;34");
        let input = json!({"key": "value"});
        let output = pretty_print(&input, 2, true);
        let expected_output = "{\n  \"key\": \"\u{1b}[0;32mvalue\u{1b}[0m\"\n}";
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_compact_output() {
        let input = json!({"key": "value"});
        let output = compact_output(&input);
        assert_eq!(output, "{\"key\":\"value\"}");
    }

    #[test]
    fn test_monochrome_print() {
        let input = json!({"key": "value"});
        let output = monochrome_print(&input);
        assert_eq!(output, "{\n  \"key\": \"value\"\n}");
    }

    #[test]
    fn test_sort_keys_nested() {
        let input = json!({"b": {"d": 4, "c": 3}, "a": 2});
        let output = sort_keys(&input);
        assert_eq!(output, json!({"a": 2, "b": {"c": 3, "d": 4}}));
    }

    #[test]
    fn test_pretty_print_complex() {
        let input = json!({"key": "value", "array": [1, 2, 3], "nested": {"a": 1, "b": 2}});
        let output = pretty_print(&input, 2, false);
        let expected_output = "{\n  \"key\": \"value\",\n  \"array\": [\n    1,\n    2,\n    3\n  ],\n  \"nested\": {\n    \"a\": 1,\n    \"b\": 2\n  }\n}";
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_compact_output_complex() {
        let input = json!({"key": "value", "array": [1, 2, 3], "nested": {"a": 1, "b": 2}});
        let output = compact_output(&input);
        assert_eq!(
            output,
            "{\"key\":\"value\",\"array\":[1,2,3],\"nested\":{\"a\":1,\"b\":2}}"
        );
    }
}
