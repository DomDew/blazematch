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
    pub fn new(score: impl Into<f64>, substring: impl Into<String>, start_index: impl Into<usize>, end_index: impl Into<usize>) -> Self {
        Self {
            score: score.into(),
            substring: substring.into(),
            start_index: start_index.into(),
            end_index: end_index.into()
        }
    }
}

impl Match {
    pub fn new(median: impl Into<f64>, candidate: impl Into<String>, matches: impl Into<Vec<LevenshteinMatch>>) -> Self {
        Self {
            median: median.into(),
            candidate: candidate.into(),
            matches: matches.into(),
        }
    }
}

pub fn fuzzy_match(query: &str, candidates: &Vec<&str>, threshold: f64) -> Vec<Match> {
    let query_substrings: Vec<&str>  = query.split(" ").collect();

    let mut matches = candidates
        .iter()
        .filter_map(|candidate| {
            let candidate_substrings: Vec<&str> = candidate.split(" ").collect();       

            let levenshtein_matches_per_query: Vec<LevenshteinMatch> = query_substrings.iter().map(|query_substring| {
                
                let mut levenshtein_matches: Vec<LevenshteinMatch> = candidate_substrings.iter().map(|candidate_substring| {
                    let levenshtein_distance = levenshtein::get_distance(query_substring, &candidate_substring, 5, 2, 1);
                    
                    let shorter_string_length = if query_substring.len() <= candidate_substring.len() {
                        query.len()
                    } else {
                        candidate_substring.len()
                    };
    
                    let score = levenshtein::normalize_similarity(shorter_string_length, levenshtein_distance);
                    let start_index = candidate.find(candidate_substring).unwrap_or(0);
                    let end_index = start_index + candidate_substring.len();

                    
                    LevenshteinMatch::new(score, candidate_substring.to_string(), start_index, end_index)
                }).collect();
    
                // Reverse sort the matches, as 1.0 is the 'best' score
                levenshtein_matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
                levenshtein_matches[0].clone()
            }).collect();

            let levenshtein_scores: Vec<f64> = levenshtein_matches_per_query.iter().map(|levenshtein_match| {
                levenshtein_match.score
            }).collect();
            
            let median = median::get_median(&levenshtein_scores).unwrap_or(0.0);

            let filtered_matches: Vec<LevenshteinMatch> = levenshtein_matches_per_query.iter().filter(|levenshtein_match| { levenshtein_match.score > 0.0 }).cloned().collect();

            if median <= threshold {
                Some(Match::new(median, *candidate, filtered_matches))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    matches.sort_by(|a, b| {
        b.median
            .partial_cmp(&a.median)
            .expect("medians should be comparable")
    });

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let candidates = vec![
            "I desperately need a kitten",
            "I desperately need a mitten",
            "I am a butterfly and need food",
            "You need food",
            "This is a banana"
        ];
        let query = "I need kitten";


        assert_eq!(
            fuzzy_match(query, &candidates, 10.),
            vec![      
                Match {
                    median: 1.0,
                    candidate: "I desperately need a kitten".into(),
                    matches: vec![
                        LevenshteinMatch::new(1.0, "I", 0_usize, 1_usize),
                        LevenshteinMatch::new(1.0, "need", 14_usize, 18_usize),
                        LevenshteinMatch::new(1.0, "kitten", 21_usize, 27_usize),
                        ]
                },
                Match { median: 1.0, 
                    candidate: "I desperately need a mitten".into(),
                    matches: 
                    vec![
                        LevenshteinMatch::new(1.0, "I", 0_usize, 1_usize), 
                        LevenshteinMatch::new(1.0, "need", 14_usize, 18_usize), 
                        LevenshteinMatch::new(0.9200444146293232, "mitten", 21_usize, 27_usize), 
                        ]
                },
                Match { median: 1.0, 
                    candidate: "I am a butterfly and need food".into(),
                    matches: 
                    vec![
                        LevenshteinMatch::new(1.0, "I", 0_usize, 1_usize), 
                        LevenshteinMatch::new(1.0, "need", 21_usize, 25_usize), 
                        LevenshteinMatch::new(0.10539922456186433, "butterfly", 7_usize, 16_usize), 
                        ]
                },
                Match { median: 0.6411803884299546, 
                        candidate: "This is a banana".into(),
                        matches: 
                        vec![
                            LevenshteinMatch::new(0.9200444146293232, "a", 8_usize, 9_usize), 
                            LevenshteinMatch::new(0.6411803884299546, "This", 0_usize, 4_usize), 
                            LevenshteinMatch::new(0.42437284567695, "banana", 10_usize, 16_usize)
                            ]
                },
                Match { median: 0.5352614285189903, 
                    candidate: "You need food".into(),
                    matches: 
                    vec![
                        LevenshteinMatch::new(0.5352614285189903, "You", 0_usize, 3_usize), 
                        LevenshteinMatch::new(1.0, "need", 4_usize, 8_usize), 
                        ]
                }
            ]        
        );
    }
}
