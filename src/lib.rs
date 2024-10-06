mod utils;
use utils::levenshtein;
use utils::n_gram;

#[derive(Debug, PartialEq)]
pub struct Match {
    score: f64,
    candidate: String,
}

pub fn fuzzy_match(query: &str, candidates: Vec<&str>, threshold: f64) -> Vec<Match> {
    let mut matches: Vec<Match> = Vec::new();

    for candidate in &candidates {
        // TODO:
        // Rather run Levenshtein matching for every word in query string against every word in candidate
        // Reasoning: Be more sensitive towards exact matches
        // Split query
        // Split candidate
        // For each query substring, compare with each candidate substring

        let levenshtein_distance = levenshtein::get_distance(query, &candidate, 10, 3, 1);
        let n_gram_similarity = n_gram::get_similarity(query, &candidate, 3);
        // Include exact matches score

        let score =
            3.0 * (1 - levenshtein_distance / candidate.len()) as f64 + 7.0 * n_gram_similarity;

        if score <= threshold {
            matches.push(Match {
                score,
                candidate: candidate.to_string(),
            });
        }
    }

    matches.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let candidates = vec![
            "kitten", "smitten", "mitten", "smithing", "running", "banana",
        ];
        let query = "kitten";
        assert_eq!(
            fuzzy_match(query, candidates, 10.),
            vec![
                Match {
                    score: 0.0,
                    candidate: "smithing".into(),
                },
                Match {
                    score: 0.0,
                    candidate: "running".into(),
                },
                Match {
                    score: 0.0,
                    candidate: "banana".into(),
                },
                Match {
                    score: 6.5,
                    candidate: "smitten".into(),
                },
                Match {
                    score: 7.2,
                    candidate: "mitten".into(),
                },
                Match {
                    score: 10.0,
                    candidate: "kitten".into(),
                }
            ]
        );
    }
}
