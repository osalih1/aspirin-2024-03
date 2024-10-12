use anyhow::Result;
use regex::Regex;

pub trait Searcher {
    fn search(&self, line: &str) -> bool;
}

pub struct ExactSearcher {
    needle: String,
}

impl ExactSearcher {
    pub fn new(needle: &str) -> Self {
        Self {
            needle: needle.to_string(),
        }
    }
}

impl Searcher for ExactSearcher {
    fn search(&self, line: &str) -> bool {
        line.contains(&self.needle)
    }
}

pub struct CaseInsensitiveSearcher {
    needle: String,
}

impl CaseInsensitiveSearcher {
    pub fn new(needle: &str) -> Self {
        Self {
            needle: needle.to_lowercase(),
        }
    }
}

impl Searcher for CaseInsensitiveSearcher {
    fn search(&self, line: &str) -> bool {
        line.to_lowercase().contains(&self.needle)
    }
}

pub struct RegexSearcher {
    regex: Regex,
}

impl RegexSearcher {
    pub fn new(needle: &str, ignore_case: bool) -> Result<Self> {
        let regex_pattern = if ignore_case {
            format!("(?i){}", needle) // case-insensitive regex matching
        } else {
            needle.to_string()
        };

        let regex = Regex::new(&regex_pattern)
            .map_err(|e| anyhow::anyhow!("Invalid regex pattern: {}", e))?;

        Ok(Self { regex })
    }
}

impl Searcher for RegexSearcher {
    fn search(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }
}

// Unit Tests for Searcher
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_search() {
        let searcher = ExactSearcher::new("foo");
        assert!(searcher.search("foo bar"));
        assert!(!searcher.search("bar baz"));
    }

    #[test]
    fn test_case_insensitive_search() {
        let searcher = CaseInsensitiveSearcher::new("foo");
        assert!(searcher.search("Foo bar"));
        assert!(!searcher.search("bar baz"));
    }

    #[test]
    fn test_regex_search() {
        let searcher = RegexSearcher::new(r"fo+", false).unwrap();
        assert!(searcher.search("foo bar"));
        assert!(!searcher.search("bar baz"));

        let searcher_case_insensitive = RegexSearcher::new(r"fo+", true).unwrap();
        assert!(searcher_case_insensitive.search("Foo bar"));
    }
}
