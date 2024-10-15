mod utils;
use utils::levenshtein;
use utils::median;

#[derive(Debug, PartialEq, Clone)]
pub struct LevenshteinMatch {
    score: f64,
    substring: String,
    start_index: usize,
    end_index: usize,
}

#[derive(Debug, PartialEq)]
pub struct Match {
    median: f64,
    matches: Vec<LevenshteinMatch>,
    candidate: String,
}

impl LevenshteinMatch {
    pub fn new(
        score: impl Into<f64>,
        substring: impl Into<String>,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        Self {
            score: score.into(),
            substring: substring.into(),
            start_index,
            end_index,
        }
    }
}

impl Match {
    pub fn new(
        median: f64,
        candidate: impl Into<String>,
        matches: impl Into<Vec<LevenshteinMatch>>,
    ) -> Self {
        Self {
            median,
            candidate: candidate.into(),
            matches: matches.into(),
        }
    }
}

fn build_levenshtein_match(
    query_substring: &str,
    candidate_substring: &str,
    query: &str,
    candidate: &str,
    deletion_cost: usize,
    insertion_cost: usize,
    substition_cost: usize,
) -> LevenshteinMatch {
    let levenshtein_distance = levenshtein::get_distance(
        query_substring,
        &candidate_substring,
        deletion_cost,   // 5
        insertion_cost,  // 2
        substition_cost, // 1
    );

    let shorter_string_length = if query_substring.len() <= candidate_substring.len() {
        query.len()
    } else {
        candidate_substring.len()
    };

    let score = levenshtein::normalize_similarity(shorter_string_length, levenshtein_distance);
    let start_index = candidate.find(candidate_substring).unwrap_or(0);
    let end_index = start_index + candidate_substring.len();

    LevenshteinMatch::new(
        score,
        candidate_substring.to_string(),
        start_index,
        end_index,
    )
}

fn get_best_levenshtein_match_for_query_substring(
    candidate_substrings: &[&str],
    query_substring: &str,
    candidate: &str,
    query: &str,
    deletion_cost: usize,
    insertion_cost: usize,
    substition_cost: usize,
) -> LevenshteinMatch {
    let mut levenshtein_matches: Vec<LevenshteinMatch> = candidate_substrings
        .iter()
        .map(|candidate_substring| {
            build_levenshtein_match(
                query_substring,
                candidate_substring,
                query,
                candidate,
                deletion_cost,
                insertion_cost,
                substition_cost,
            )
        })
        .collect();

    // Reverse sort the matches, as 1.0 is the 'best' score
    levenshtein_matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    levenshtein_matches[0].clone()
}

fn get_match(
    query: &str,
    candidate: &str,
    substring_min_length: usize,
    threshold: f64,
    query_substrings: &[&str],
    deletion_cost: usize,
    insertion_cost: usize,
    substition_cost: usize,
) -> Option<Match> {
    let candidate_substrings: Vec<&str> = candidate
        .split(" ")
        .filter(|query_substring| query_substring.len() >= substring_min_length)
        .collect();

    let levenshtein_matches_per_query_substring: Vec<LevenshteinMatch> = query_substrings
        .iter()
        .map(|query_substring| {
            get_best_levenshtein_match_for_query_substring(
                &candidate_substrings,
                query_substring,
                candidate,
                query,
                deletion_cost,
                insertion_cost,
                substition_cost,
            )
        })
        .collect();

    let levenshtein_scores: Vec<f64> = levenshtein_matches_per_query_substring
        .iter()
        .map(|levenshtein_match| levenshtein_match.score)
        .collect();

    let median = median::get_median(&levenshtein_scores).unwrap_or(0.0);

    let filtered_matches: Vec<LevenshteinMatch> = levenshtein_matches_per_query_substring
        .iter()
        .filter(|levenshtein_match| levenshtein_match.score > 0.0)
        .cloned()
        .collect();

    (median >= threshold).then(|| Match::new(median, &*candidate, filtered_matches))
}

#[derive(Debug)]
pub struct FuzzyMatchOptions {
    pub threshold: f64,
    pub substring_min_length: usize,
    pub deletion_cost: usize,
    pub insertion_cost: usize,
    pub substition_cost: usize,
}

impl Default for FuzzyMatchOptions {
    fn default() -> Self {
        Self {
            threshold: 0.6,
            substring_min_length: 3,
            deletion_cost: 1,
            insertion_cost: 1,
            substition_cost: 1,
        }
    }
}

pub fn fuzzy_match(query: &str, candidates: &[&str], options: FuzzyMatchOptions) -> Vec<Match> {
    let FuzzyMatchOptions {
        threshold,
        substring_min_length,
        deletion_cost,
        insertion_cost,
        substition_cost,
    } = options;

    let query_substrings: Vec<&str> = query
        .split(" ")
        .filter(|query_substring| query_substring.len() >= substring_min_length)
        .collect();

    let mut matches = candidates
        .iter()
        .filter_map(|candidate| {
            get_match(
                query,
                candidate,
                substring_min_length,
                threshold,
                &query_substrings,
                deletion_cost,
                insertion_cost,
                substition_cost,
            )
        })
        .collect::<Vec<_>>();

    matches.sort_by(|a, b| {
        b.median
            .partial_cmp(&a.median)
            .expect("Medians should be comparable")
    });

    matches
}

pub trait FuzzyMatch<Q: ToString> {
    fn fuzzy_match(&self, query: Q, options: FuzzyMatchOptions) -> Vec<Match>;
}

impl<T: ToString, Q: ToString> FuzzyMatch<Q> for Vec<T> {
    fn fuzzy_match(&self, query: Q, options: FuzzyMatchOptions) -> Vec<Match> {
        let candidate_strings: Vec<String> = self.iter().map(|s| s.to_string()).collect();
        let candidates: Vec<&str> = candidate_strings.iter().map(|s| s.as_str()).collect();
        fuzzy_match(&query.to_string(), &candidates, options)
    }
}

impl<T: ToString, Q: ToString, const N: usize> FuzzyMatch<Q> for [T; N] {
    fn fuzzy_match(&self, query: Q, options: FuzzyMatchOptions) -> Vec<Match> {
        let candidate_strings: Vec<String> = self.iter().map(|s| s.to_string()).collect();
        let candidates: Vec<&str> = candidate_strings.iter().map(|s| s.as_str()).collect();
        fuzzy_match(&query.to_string(), &candidates, options)
    }
}
