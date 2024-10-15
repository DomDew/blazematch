use blazematch::{fuzzy_match, FuzzyMatchOptions, LevenshteinMatch, Match};

#[test]
fn test_with_default_options() {
    let candidates = vec!["kitten", "bitten", "smitten"];

    let query = "kitten";

    let actual = fuzzy_match(query, &candidates, FuzzyMatchOptions::default());

    let expected = vec![
        Match::new(
            1.0,
            "kitten",
            vec![LevenshteinMatch::new(1.0, "kitten", 0, 6)],
        ),
        Match::new(
            0.8187307530779818,
            "bitten",
            vec![LevenshteinMatch::new(0.8187307530779818, "bitten", 0, 6)],
        ),
        Match::new(
            0.6065306597126334,
            "smitten",
            vec![LevenshteinMatch::new(0.6065306597126334, "smitten", 0, 7)],
        ),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_a_short_query_against_longer_candidates() {
    let candidates = vec![
        "I desperately need a kitten",
        "I desperately need a mitten",
        "I am a butterfly and need food",
        "You need food",
        "This is a banana",
    ];

    let query = "kitten";

    let options = FuzzyMatchOptions {
        threshold: 0.1,
        substring_min_length: 1,
        deletion_cost: 5,
        insertion_cost: 2,
        substition_cost: 1,
    };

    let actual = fuzzy_match(query, &candidates, options);

    let expected = vec![
        Match::new(
            1.0,
            "I desperately need a kitten",
            vec![LevenshteinMatch::new(1.0, "kitten", 21, 27)],
        ),
        Match::new(
            0.8187307530779818,
            "I desperately need a mitten",
            vec![LevenshteinMatch::new(0.8187307530779818, "mitten", 21, 27)],
        ),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_with_lax_settings() {
    let candidates = vec![
        "I desperately need a kitten",
        "I desperately need a mitten",
        "I am a butterfly and need food",
        "You need food",
        "This is a banana",
    ];

    let query = "I need kitten";

    let options = FuzzyMatchOptions {
        threshold: 0.0,
        substring_min_length: 1,
        deletion_cost: 5,
        insertion_cost: 2,
        substition_cost: 1,
    };

    let actual = fuzzy_match(query, &candidates, options);

    let expected = vec![
        Match::new(
            1.0,
            "I desperately need a kitten",
            vec![
                LevenshteinMatch::new(1.0, "I", 0, 1),
                LevenshteinMatch::new(1.0, "need", 14, 18),
                LevenshteinMatch::new(1.0, "kitten", 21, 27),
            ],
        ),
        Match::new(
            1.0,
            "I desperately need a mitten",
            vec![
                LevenshteinMatch::new(1.0, "I", 0, 1),
                LevenshteinMatch::new(1.0, "need", 14, 18),
                LevenshteinMatch::new(0.9200444146293232, "mitten", 21, 27),
            ],
        ),
        Match::new(
            1.0,
            "I am a butterfly and need food",
            vec![
                LevenshteinMatch::new(1.0, "I", 0, 1),
                LevenshteinMatch::new(1.0, "need", 21, 25),
                LevenshteinMatch::new(0.10539922456186433, "butterfly", 7, 16),
            ],
        ),
        Match::new(
            0.6411803884299546,
            "This is a banana",
            vec![
                LevenshteinMatch::new(0.9200444146293232, "a", 8, 9),
                LevenshteinMatch::new(0.6411803884299546, "This", 0, 4),
                LevenshteinMatch::new(0.42437284567695, "banana", 10, 16),
            ],
        ),
        Match::new(
            0.5352614285189903,
            "You need food",
            vec![
                LevenshteinMatch::new(0.5352614285189903, "You", 0, 3),
                LevenshteinMatch::new(1.0, "need", 4, 8),
            ],
        ),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_with_strict_settings() {
    let candidates = vec![
        "I desperately need a kitten",
        "I desperately need a mitten",
        "I am a butterfly and need food",
        "You need food",
        "This is a banana",
    ];

    let query = "I need kitten";

    let options = FuzzyMatchOptions {
        threshold: 0.8,
        substring_min_length: 4,
        deletion_cost: 5,
        insertion_cost: 2,
        substition_cost: 1,
    };

    let actual = fuzzy_match(query, &candidates, options);

    let expected = vec![
        Match::new(
            1.0,
            "I desperately need a kitten",
            vec![
                LevenshteinMatch::new(1.0, "need", 14, 18),
                LevenshteinMatch::new(1.0, "kitten", 21, 27),
            ],
        ),
        Match::new(
            0.9600222073146616,
            "I desperately need a mitten",
            vec![
                LevenshteinMatch::new(1.0, "need", 14, 18),
                LevenshteinMatch::new(0.9200444146293232, "mitten", 21, 27),
            ],
        ),
    ];

    assert_eq!(actual, expected);
}
