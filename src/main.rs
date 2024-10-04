mod utils;
use utils::levenshtein;
use utils::n_gram;

struct Match {
    score: f64,
    candidate: String,
}

fn fuzzy_match(query: &str, candidates: Vec<&str>, threshold: f64) -> Vec<Match> {
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

fn main() {
    let candidates: Vec<&str> = vec![
        "kitten", "smitten", "mitten", "smithing", "running", "banana",
    ];
    let query = "kitten";
    let matches = fuzzy_match(query, candidates, 10.0);
    println!("Querying: {}", query);
    for m in matches {
        println!("Match: candidate = {}, score = {}", m.candidate, m.score);
    }
}
