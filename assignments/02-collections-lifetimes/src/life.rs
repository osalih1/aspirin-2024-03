fn split_string<'a>(string: &'a str, delimiter: &str) -> Vec<&'a str> {
    string.split(delimiter).filter(|s| !s.is_empty()).collect()
}

#[derive(PartialEq, Debug)]
struct Differences<'a> {
    only_in_first: Vec<&'a str>,
    only_in_second: Vec<&'a str>,
}

fn contains_substring(word: &str, other_string: &str) -> bool {
    other_string
        .split_whitespace()
        .any(|w| w.find(word).is_some())
}

fn find_differences<'a>(first_string: &'a str, second_string: &'a str) -> Differences<'a> {
    let first_words: Vec<&str> = split_string(first_string, " ");
    let second_words: Vec<&str> = split_string(second_string, " ");

    let only_in_first: Vec<&str> = first_words
        .into_iter()
        .filter(|&word| !word.is_empty() && !contains_substring(word, second_string))
        .collect();

    let only_in_second: Vec<&str> = second_words
        .into_iter()
        .filter(|&word| !word.is_empty() && !contains_substring(word, first_string))
        .collect();

    Differences {
        only_in_first,
        only_in_second,
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}
fn merge_names(first_name: &str, second_name: &str) -> String {
    let mut result = String::new();
    let mut first_iter = first_name.chars();
    let mut second_iter = second_name.chars();

    let mut use_first = true;

    loop {
        let current_iter = if use_first {
            &mut first_iter
        } else {
            &mut second_iter
        };

        let mut found_vowel = false;
        let mut first_char = true;

        while let Some(c) = current_iter.next() {
            if is_vowel(c) && !first_char {
                found_vowel = true;
                use_first = !use_first;
                break;
            }
            result.push(c);
            first_char = false;
        }

        if !found_vowel && current_iter.as_str().is_empty() {
            break;
        }

        // If the next part starts with a vowel, continue with the same logic
        if found_vowel && current_iter.as_str().starts_with(|c: char| is_vowel(c)) {
            use_first = !use_first;
        }
    }

    result.extend(first_iter);
    result.extend(second_iter);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        // First, make sure the lifetimes were correctly marked
        let matches;
        let string_to_split = String::from("Hello, World!");

        {
            let delimeter = String::from(", ");
            matches = split_string(&string_to_split, &delimeter);
        }
        println!("Matches can be printed! See: {:?}", matches);

        // Now check the split logic
        assert_eq!(split_string(&"", &""), Vec::<&str>::new());
        assert_eq!(
            split_string(&"Hello, World!", &", "),
            vec!["Hello", "World!"]
        );
        assert_eq!(
            split_string(
                &"I this think this that this sentence this is this very this confusing this ",
                &" this "
            ),
            vec!["I", "think", "that", "sentence", "is", "very", "confusing"]
        );
        assert_eq!(
            split_string(&"apple🍎banana🍎orange", &"🍎"),
            vec!["apple", "banana", "orange"]
        );
        assert_eq!(
            split_string(
                &"Ayush;put|a,lot~of`random;delimeters|in|this,sentence",
                &";"
            ),
            vec![
                "Ayush",
                "put|a,lot~of`random",
                "delimeters|in|this,sentence"
            ]
        );
    }

    #[test]
    fn test_find_differences() {
        assert_eq!(
            find_differences(&"", &""),
            Differences {
                only_in_first: Vec::new(),
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(&"pineapple pen", &"apple"),
            Differences {
                only_in_first: vec!["pineapple", "pen"],
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(
                &"Sally sold seashells at the seashore",
                &"Seashells seashells at the seashore"
            ),
            Differences {
                only_in_first: vec!["Sally", "sold"],
                only_in_second: vec!["Seashells"]
            }
        );
        assert_eq!(
            find_differences(
                "How much wood could a wood chuck chuck",
                "If a wood chuck could chuck wood"
            ),
            Differences {
                only_in_first: vec!["How", "much"],
                only_in_second: vec!["If"]
            }
        );
        assert_eq!(
            find_differences(
                &"How much ground would a groundhog hog",
                &"If a groundhog could hog ground"
            ),
            Differences {
                only_in_first: vec!["How", "much", "would"],
                only_in_second: vec!["If", "could"]
            }
        );
    }

    #[test]
    fn test_merge_names() {
        assert_eq!(merge_names(&"alex", &"jake"), "aljexake");
        assert_eq!(merge_names(&"steven", &"stephen"), "ststevephenen");
        assert_eq!(merge_names(&"gym", &"rhythm"), "gymrhythm");
        assert_eq!(merge_names(&"walter", &"gibraltor"), "wgaltibreraltor");
        assert_eq!(merge_names(&"baker", &"quaker"), "bqakueraker");
        assert_eq!(merge_names(&"", &""), "");
        assert_eq!(merge_names(&"samesies", &"samesies"), "ssamamesesiieses");
        assert_eq!(merge_names(&"heather", &"meagan"), "hmeeathageran");
        assert_eq!(merge_names(&"panda", &"turtle"), "ptandurtlae");
        assert_eq!(merge_names(&"hot", &"sauce"), "hsotauce");
        assert_eq!(merge_names(&"", &"second"), "second");
        assert_eq!(merge_names(&"first", &""), "first");
    }
}
