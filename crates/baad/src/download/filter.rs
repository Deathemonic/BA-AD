use crate::helpers::error::FilterError;

use clap::ValueEnum;
use glob::Pattern as GlobPattern;
use lazy_regex::Regex;
use nucleo::{Config, Matcher, Utf32Str};

#[derive(Debug, Clone, ValueEnum)]
pub enum FilterMethod {
    Exact,
    Contains,
    Regex,
    Fuzzy,
    Glob,
    #[value(name = "contains-ignore-case")]
    ContainsIgnoreCase,
    #[value(name = "starts-with")]
    StartsWith,
    #[value(name = "ends-with")]
    EndsWith,
}

pub struct ResourceFilter {
    pattern: String,
    method: FilterMethod,
    compiled_regex: Option<Regex>,
    fuzzy_matcher: Option<Matcher>,
    lowercase_pattern: Option<String>,
}

impl ResourceFilter {
    pub fn new(pattern: &str, method: FilterMethod) -> Result<Self, FilterError> {
        let mut compiled_regex = None;
        let mut fuzzy_matcher = None;
        let mut lowercase_pattern = None;

        match method {
            FilterMethod::Regex => {
                compiled_regex =
                    Some(Regex::new(pattern).map_err(|_| FilterError::InvalidRegex {
                        pattern: pattern.into(),
                    })?);
            }
            FilterMethod::Fuzzy => {
                fuzzy_matcher = Some(Matcher::new(Config::DEFAULT));
            }
            FilterMethod::Glob => {
                GlobPattern::new(pattern).map_err(|_| FilterError::InvalidGlob {
                    pattern: pattern.into(),
                })?;
            }
            FilterMethod::ContainsIgnoreCase => {
                lowercase_pattern = Some(pattern.to_lowercase());
            }
            _ => {}
        }

        Ok(Self {
            pattern: pattern.into(),
            method,
            compiled_regex,
            fuzzy_matcher,
            lowercase_pattern
        })
    }

    fn match_exact(&self, path: &str) -> bool {
        path == self.pattern
    }

    fn match_contains(&self, path: &str) -> bool {
        path.contains(&self.pattern)
    }

    fn match_contains_ignore_case(&self, path: &str) -> bool {
        if let Some(ref lower_pattern) = self.lowercase_pattern {
            path.to_lowercase().contains(lower_pattern)
        } else {
            false
        }
    }

    fn match_starts_with(&self, path: &str) -> bool {
        path.starts_with(&self.pattern)
    }

    fn match_ends_with(&self, path: &str) -> bool {
        path.ends_with(&self.pattern)
    }

    fn match_regex(&self, path: &str) -> bool {
        self.compiled_regex
            .as_ref()
            .map(|r| r.is_match(path))
            .unwrap_or(false)
    }

    fn match_fuzzy(&self, path: &str) -> bool {
        if let Some(matcher) = &self.fuzzy_matcher {
            let mut matcher = matcher.clone();
            let mut pattern_buf = Vec::new();
            let mut haystack_buf = Vec::new();

            let pattern = Utf32Str::new(&self.pattern, &mut pattern_buf);
            let haystack = Utf32Str::new(path, &mut haystack_buf);

            matcher.fuzzy_match(haystack, pattern).is_some()
        } else {
            path.contains(&self.pattern)
        }
    }

    fn match_glob(&self, path: &str) -> bool {
        GlobPattern::new(&self.pattern)
            .map(|pattern| pattern.matches(path))
            .unwrap_or(false)
    }

    pub fn matches(&self, path: &str) -> bool {
        match self.method {
            FilterMethod::Exact => self.match_exact(path),
            FilterMethod::Contains => self.match_contains(path),
            FilterMethod::ContainsIgnoreCase => self.match_contains_ignore_case(path),
            FilterMethod::StartsWith => self.match_starts_with(path),
            FilterMethod::EndsWith => self.match_ends_with(path),
            FilterMethod::Regex => self.match_regex(path),
            FilterMethod::Fuzzy => self.match_fuzzy(path),
            FilterMethod::Glob => self.match_glob(path),
        }
    }
}

impl ResourceFilter {
    pub fn glob(pattern: &str) -> Result<Self, FilterError> {
        Self::new(pattern, FilterMethod::Glob)
    }

    pub fn regex(pattern: &str) -> Result<Self, FilterError> {
        Self::new(pattern, FilterMethod::Regex)
    }

    pub fn contains(pattern: &str) -> Result<Self, FilterError> {
        Self::new(pattern, FilterMethod::Contains)
    }

    pub fn exact(pattern: &str) -> Result<Self, FilterError> {
        Self::new(pattern, FilterMethod::Exact)
    }

    pub fn starts_with(pattern: &str) -> Result<Self, FilterError> {
        Self::new(pattern, FilterMethod::StartsWith)
    }

    pub fn ends_with(pattern: &str) -> Result<Self, FilterError> {
        Self::new(pattern, FilterMethod::EndsWith)
    }

    pub fn contains_ignore_case(pattern: &str) -> Result<Self, FilterError> {
        Self::new(pattern, FilterMethod::ContainsIgnoreCase)
    }

    pub fn fuzzy(pattern: &str) -> Result<Self, FilterError> {
        Self::new(pattern, FilterMethod::Fuzzy)
    }
}
