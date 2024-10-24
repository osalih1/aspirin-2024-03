use serde_json::Value;

pub fn identity_filter(json: &Value) -> Value {
    json.clone()
}

pub fn object_identifier_index(json: &Value, key: &str) -> Value {
    json.get(key).cloned().unwrap_or(Value::Null)
}

pub fn array_index(json: &Value, index: usize) -> Value {
    json.get(index).cloned().unwrap_or(Value::Null)
}

pub fn array_slice(json: &Value, start: usize, end: usize) -> Value {
    if let Some(array) = json.as_array() {
        Value::Array(array[start..end].to_vec())
    } else {
        Value::Null
    }
}

pub fn pipe(json: &Value, key: &str, index: usize) -> Value {
    let intermediate = object_identifier_index(json, key);
    array_index(&intermediate, index)
}

pub fn array_iterator(json: &Value, key: &str) -> Value {
    if let Some(array) = json.as_array() {
        let iterated: Vec<Value> = array
            .iter()
            .map(|item| object_identifier_index(item, key))
            .collect();
        Value::Array(iterated)
    } else {
        Value::Null
    }
}

pub fn add(json: &Value) -> Value {
    if let Some(array) = json.as_array() {
        let sum = array
            .iter()
            .fold(Value::Null, |acc, item| match (acc, item) {
                (Value::String(mut acc_str), Value::String(item_str)) => {
                    acc_str.push_str(item_str);
                    Value::String(acc_str)
                }
                (Value::Number(acc_num), Value::Number(item_num)) => {
                    let acc_f64 = acc_num.as_f64().unwrap_or(0.0);
                    let item_f64 = item_num.as_f64().unwrap_or(0.0);
                    Value::Number(serde_json::Number::from_f64(acc_f64 + item_f64).unwrap())
                }
                _ => Value::Null,
            });
        sum
    } else {
        Value::Null
    }
}

pub fn length(json: &Value) -> Value {
    match json {
        Value::Array(arr) => Value::Number(arr.len().into()),
        Value::Object(obj) => Value::Number(obj.len().into()),
        Value::String(s) => Value::Number(s.len().into()),
        _ => Value::Null,
    }
}

pub fn del(json: &mut Value, key_or_index: &str) -> Value {
    match json {
        Value::Object(ref mut obj) => {
            obj.remove(key_or_index);
            Value::Object(obj.clone())
        }
        Value::Array(ref mut arr) => {
            if let Ok(index) = key_or_index.parse::<usize>() {
                if index < arr.len() {
                    arr.remove(index);
                }
            }
            Value::Array(arr.clone())
        }
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_identity_filter() {
        let input = json!({"key": "value"});
        let output = identity_filter(&input);
        assert_eq!(output, input);
    }

    #[test]
    fn test_object_identifier_index() {
        let input = json!({"key": "value"});
        let output = object_identifier_index(&input, "key");
        assert_eq!(output, json!("value"));
    }

    #[test]
    fn test_array_index() {
        let input = json!(["value1", "value2"]);
        let output = array_index(&input, 0);
        assert_eq!(output, json!("value1"));
    }

    #[test]
    fn test_array_slice() {
        let input = json!(["value1", "value2", "value3"]);
        let output = array_slice(&input, 0, 2);
        assert_eq!(output, json!(["value1", "value2"]));
    }

    #[test]
    fn test_pipe() {
        let input = json!({"key": ["value1", "value2"]});
        let output = pipe(&input, "key", 1);
        assert_eq!(output, json!("value2"));
    }

    #[test]
    fn test_array_iterator() {
        let input = json!([{"name": "value1"}, {"name": "value2"}]);
        let output = array_iterator(&input, "name");
        assert_eq!(output, json!(["value1", "value2"]));
    }

    #[test]
    fn test_add() {
        let input = json!(["one", "two", "three"]);
        let output = add(&input);
        assert_eq!(output, json!("onetwothree"));
    }

    #[test]
    fn test_length() {
        let input = json!(["one", "two", "three"]);
        let output = length(&input);
        assert_eq!(output, json!(3));
    }

    #[test]
    fn test_del() {
        let mut input = json!({"key": "value", "key2": "value2"});
        let output = del(&mut input, "key");
        assert_eq!(output, json!({"key2": "value2"}));
    }
}
