fn longest_equal_sequence_prescriptive<T: PartialEq>(sequence: &[T]) -> i32 {
    if sequence.is_empty() {
        return 0;
    }

    let mut current_length: i32 = 1; // Start with 1 since we always have the first element
    let mut longest_length: i32 = 1;

    for i in 1..sequence.len() {
        if sequence[i - 1] == sequence[i] {
            current_length += 1;
        } else {
            if current_length > longest_length {
                longest_length = current_length;
            }
            current_length = 1; // Reset current sequence length
        }
    }

    // Final check for the longest sequence after the loop
    if current_length > longest_length {
        longest_length = current_length;
    }

    longest_length
}

fn longest_equal_sequence_functional<T: PartialEq>(sequence: &[T]) -> i32 {
    if sequence.is_empty() {
        return 0;
    }

    sequence
        .windows(2)
        .fold((1, 1), |(longest, current_len), window| {
            if window[0] == window[1] {
                (longest.max(current_len + 1), current_len + 1)
            } else {
                (longest.max(current_len), 1)
            }
        })
        .0
}

fn is_valid_parentheses(parentheses: &str) -> bool {
    if parentheses.is_empty() {
        return true;
    }
    let mut stack: Vec<char> = Vec::new();

    for item in parentheses.chars() {
        match item {
            '(' | '{' | '[' => {
                // Push the opening brackets onto the stack
                stack.push(item);
            }
            ')' => {
                // If it's a closing parenthesis, check if it matches the last opening parenthesis
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => {
                // For any other characters, we do nothing or handle differently
                // This ensures that all characters are covered
            }
        }
    }
    stack.is_empty() // if empty, it is a valid parenthesis string
}

fn longest_common_substring<'a>(first_str: &'a str, second_str: &'a str) -> &'a str {
    let mut longest_substr = "";
    // Iterate over all possible starting points in `first_str`
    for i in 0..first_str.len() {
        // Iterate over all possible substrings starting from index `i`
        for j in i + 1..=first_str.len() {
            let current_substr = &first_str[i..j];
            // Check if `current_substr` exists in `second_str`
            if second_str.contains(current_substr) {
                // Update `longest_substr` if this one is longer
                if current_substr.len() > longest_substr.len() {
                    longest_substr = current_substr;
                }
            }
        }
    }
    longest_substr
}

fn longest_common_substring_multiple<'a>(strings: &'a [&'a str]) -> &'a str {
    if strings.is_empty() {
        return ""; // If no strings are given, return an empty string.
    }

    // Start with the first string as the "current longest common substring"
    let mut common_substr = strings[0];

    // Iterate over the remaining strings and progressively find the longest common substring
    for i in 1..strings.len() {
        common_substr = longest_common_substring(common_substr, strings[i]);

        // If at any point the common substring becomes empty, return early
        if common_substr.is_empty() {
            return "";
        }
    }
    common_substr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_equal_sequence_prescriptive() {
        assert_eq!(longest_equal_sequence_prescriptive(&vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&vec![-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_prescriptive(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec!['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_prescriptive(&vec), 3);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 4, 4]),
            3
        );
        assert_eq!(longest_equal_sequence_prescriptive(&vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_prescriptive(&vec![1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }
    #[test]
    fn test_longest_equal_sequence_functional() {
        assert_eq!(longest_equal_sequence_functional(&vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1.0, 2.0, 2.0, 2.0, 3.0, 4.0, 4.0]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&vec![-100]), 1);
        let empty_vec: Vec<char> = Vec::new();
        assert_eq!(longest_equal_sequence_functional(&empty_vec), 0);
        assert_eq!(
            longest_equal_sequence_functional(&vec![
                1000, 1000, 2000, 2000, 2000, 3000, 3000, 3000, 3000
            ]),
            4
        );
        assert_eq!(
            longest_equal_sequence_functional(&vec!['a', 'b', 'a', 'b', 'a', 'b']),
            1
        );
        let vec: Vec<u8> = vec![5, 5, 5, 1, 2, 3];
        assert_eq!(longest_equal_sequence_functional(&vec), 3);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1, 2, 3, 4, 4, 4]),
            3
        );
        assert_eq!(longest_equal_sequence_functional(&vec![1, 2, 3, 4, 5]), 1);
        assert_eq!(
            longest_equal_sequence_functional(&vec![1, 1, 2, 2, 2, 3, 1, 1, 1, 1, 1]),
            5
        );
    }

    #[test]
    fn test_is_valid_parentheses() {
        assert_eq!(is_valid_parentheses(&String::from("{}")), true);
        assert_eq!(is_valid_parentheses(&String::from("()")), true);
        assert_eq!(is_valid_parentheses(&String::from("()[]{}")), true);
        assert_eq!(is_valid_parentheses(&String::from("({[]})")), true);
        assert_eq!(is_valid_parentheses(&String::from("([]){}{}([]){}")), true);
        assert_eq!(is_valid_parentheses(&String::from("()(")), false);
        assert_eq!(is_valid_parentheses(&String::from("(()")), false);
        assert_eq!(is_valid_parentheses(&String::from("([)]{[})")), false);
        assert_eq!(
            is_valid_parentheses(&String::from("({[()]}){[([)]}")),
            false
        );
        assert_eq!(
            is_valid_parentheses(&String::from("()[]{}(([])){[()]}(")),
            false
        );
    }

    #[test]
    fn test_common_substring() {
        assert_eq!(longest_common_substring(&"abcdefg", &"bcdef"), "bcdef");
        assert_eq!(longest_common_substring(&"apple", &"pineapple"), "apple");
        assert_eq!(longest_common_substring(&"dog", &"cat"), "");
        assert_eq!(longest_common_substring(&"racecar", &"racecar"), "racecar");
        assert_eq!(longest_common_substring(&"ababc", &"babca"), "babc");
        assert_eq!(longest_common_substring(&"xyzabcxyz", &"abc"), "abc");
        assert_eq!(longest_common_substring(&"", &"abc"), "");
        assert_eq!(longest_common_substring(&"abcdefgh", &"defghijk"), "defgh");
        assert_eq!(longest_common_substring(&"xyabcz", &"abcxy"), "abc");
        assert_eq!(longest_common_substring(&"ABCDEFG", &"abcdefg"), "");
        assert_eq!(
            longest_common_substring(
                &"thisisaverylongstringwithacommonsubstring",
                &"anotherlongstringwithacommonsubstring"
            ),
            "longstringwithacommonsubstring"
        );
        assert_eq!(longest_common_substring("a", "a"), "a");
    }

    #[test]
    fn test_common_substring_multiple() {
        assert_eq!(
            longest_common_substring_multiple(&vec!["abcdefg", "cdef"]),
            "cdef"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["apple", "pineapple", "maple", "snapple"]),
            "ple"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["dog", "cat", "fish"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["racecar", "car", "scar"]),
            "car"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["ababc", "babca", "abcab"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["xyzabcxyz", "abc", "zabcy", "abc"]),
            "abc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["", "abc", "def"]),
            ""
        );
        assert_eq!(
            longest_common_substring_multiple(&vec![
                "abcdefgh",
                "bcd",
                "bcdtravels",
                "abcs",
                "webcam"
            ]),
            "bc"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["identical", "identical", "identical"]),
            "identical"
        );
        assert_eq!(
            longest_common_substring_multiple(&vec!["xyabcz", "abcxy", "zabc"]),
            "abc"
        );
        assert_eq!(longest_common_substring_multiple(&vec!["a", "a", "a"]), "a");
        assert_eq!(
            longest_common_substring_multiple(&vec![
                "thisisaverylongstringwiththecommonsubstring",
                "anotherlongstringwithacommonsubstring",
                "yetanotherstringthatcontainsacommonsubstring",
            ]),
            "commonsubstring",
        );
    }
}
