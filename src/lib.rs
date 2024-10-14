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

fn get_match_for_query_substring(
    candidate_substrings: &Vec<&str>,
    query_substring: &str,
    candidate: &str,
    query: &str,
) -> LevenshteinMatch {
    let mut levenshtein_matches: Vec<LevenshteinMatch> = candidate_substrings
        .iter()
        .map(|candidate_substring| {
            let levenshtein_distance =
                levenshtein::get_distance(query_substring, &candidate_substring, 5, 2, 1);

            let shorter_string_length = if query_substring.len() <= candidate_substring.len() {
                query.len()
            } else {
                candidate_substring.len()
            };

            let score =
                levenshtein::normalize_similarity(shorter_string_length, levenshtein_distance);
            let start_index = candidate.find(candidate_substring).unwrap_or(0);
            let end_index = start_index + candidate_substring.len();

            LevenshteinMatch::new(
                score,
                candidate_substring.to_string(),
                start_index,
                end_index,
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
    query_substrings: &Vec<&str>,
) -> Option<Match> {
    let candidate_substrings: Vec<&str> = candidate
        .split(" ")
        .filter(|query_substring| query_substring.len() >= substring_min_length)
        .collect();

    let levenshtein_matches_per_query: Vec<LevenshteinMatch> = query_substrings
        .iter()
        .map(|query_substring| {
            get_match_for_query_substring(&candidate_substrings, query_substring, candidate, query)
        })
        .collect();

    let levenshtein_scores: Vec<f64> = levenshtein_matches_per_query
        .iter()
        .map(|levenshtein_match| levenshtein_match.score)
        .collect();

    let median = median::get_median(&levenshtein_scores).unwrap_or(0.0);

    let filtered_matches: Vec<LevenshteinMatch> = levenshtein_matches_per_query
        .iter()
        .filter(|levenshtein_match| levenshtein_match.score > 0.0)
        .cloned()
        .collect();

    (median >= threshold).then(|| Match::new(median, &*candidate, filtered_matches))
}

pub fn fuzzy_match(
    query: &str,
    candidates: &[&str],
    threshold: f64,
    substring_min_length: usize,
) -> Vec<Match> {
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
