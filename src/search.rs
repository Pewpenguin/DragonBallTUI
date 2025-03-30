use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub struct FuzzySearch {
    matcher: SkimMatcherV2,
}

impl FuzzySearch {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn search<T, F>(&self, query: &str, items: &[T], extract_text: F) -> Vec<(usize, i64)>
    where
        F: Fn(&T) -> String,
    {
        let mut results = Vec::new();

        for (idx, item) in items.iter().enumerate() {
            let text = extract_text(item);
            if let Some(score) = self.matcher.fuzzy_match(&text, query) {
                results.push((idx, score));
            }
        }

        // Sort by score (highest first)
        results.sort_by(|a, b| b.1.cmp(&a.1));
        results
    }

    pub fn highlight_matches(&self, text: &str, query: &str) -> String {
        if let Some((_, indices)) = self.matcher.fuzzy_indices(text, query) {
            let mut result = String::new();
            let mut last_idx = 0;

            for &idx in &indices {
                // Add text before the match
                if idx > last_idx {
                    result.push_str(&text[last_idx..idx]);
                }
                
                // Add the matched character with highlighting
                if idx < text.len() {
                    let c = text.chars().nth(idx).unwrap();
                    result.push_str(&format!("[{}]", c));
                }
                
                last_idx = idx + 1;
            }
            
            // Add the remaining text after the last match
            if last_idx < text.len() {
                result.push_str(&text[last_idx..]);
            }
            
            result
        } else {
            text.to_string()
        }
    }
}