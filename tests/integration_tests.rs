use blazematch::{fuzzy_match, LevenshteinMatch, Match};

#[test]
fn it_finds_a_short_query_in_longer_candidates() {
    let candidates = vec![
        "I desperately need a kitten",
        "I desperately need a mitten",
        "I am a butterfly and need food",
        "You need food",
        "This is a banana",
    ];

    let query = "kitten";

    let actual = fuzzy_match(query, &candidates, 0.1, 0);

    let expected = vec![
        Match::new(
            1.0,
            "I desperately need a kitten",
            vec![LevenshteinMatch::new(1.0, "kitten", 21_usize, 27_usize)],
        ),
        Match::new(
            0.8187307530779818,
            "I desperately need a mitten",
            vec![LevenshteinMatch::new(
                0.8187307530779818,
                "mitten",
                21_usize,
                27_usize,
            )],
        ),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn it_finds_5_matches_with_lax_settings() {
    let candidates = vec![
        "I desperately need a kitten",
        "I desperately need a mitten",
        "I am a butterfly and need food",
        "You need food",
        "This is a banana",
    ];

    let query = "I need kitten";

    let actual = fuzzy_match(query, &candidates, 0.0, 0);

    let expected = vec![
        Match::new(
            1.0,
            "I desperately need a kitten",
            vec![
                LevenshteinMatch::new(1.0, "I", 0_usize, 1_usize),
                LevenshteinMatch::new(1.0, "need", 14_usize, 18_usize),
                LevenshteinMatch::new(1.0, "kitten", 21_usize, 27_usize),
            ],
        ),
        Match::new(
            1.0,
            "I desperately need a mitten",
            vec![
                LevenshteinMatch::new(1.0, "I", 0_usize, 1_usize),
                LevenshteinMatch::new(1.0, "need", 14_usize, 18_usize),
                LevenshteinMatch::new(0.9200444146293232, "mitten", 21_usize, 27_usize),
            ],
        ),
        Match::new(
            1.0,
            "I am a butterfly and need food",
            vec![
                LevenshteinMatch::new(1.0, "I", 0_usize, 1_usize),
                LevenshteinMatch::new(1.0, "need", 21_usize, 25_usize),
                LevenshteinMatch::new(0.10539922456186433, "butterfly", 7_usize, 16_usize),
            ],
        ),
        Match::new(
            0.6411803884299546,
            "This is a banana",
            vec![
                LevenshteinMatch::new(0.9200444146293232, "a", 8_usize, 9_usize),
                LevenshteinMatch::new(0.6411803884299546, "This", 0_usize, 4_usize),
                LevenshteinMatch::new(0.42437284567695, "banana", 10_usize, 16_usize),
            ],
        ),
        Match::new(
            0.5352614285189903,
            "You need food",
            vec![
                LevenshteinMatch::new(0.5352614285189903, "You", 0_usize, 3_usize),
                LevenshteinMatch::new(1.0, "need", 4_usize, 8_usize),
            ],
        ),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn it_finds_2_matches_with_strict_settings() {
    let candidates = vec![
        "I desperately need a kitten",
        "I desperately need a mitten",
        "I am a butterfly and need food",
        "You need food",
        "This is a banana",
    ];

    let query = "I need kitten";

    let actual = fuzzy_match(query, &candidates, 0.8, 4);

    let expected = vec![
        Match::new(
            1.0,
            "I desperately need a kitten",
            vec![
                LevenshteinMatch::new(1.0, "need", 14_usize, 18_usize),
                LevenshteinMatch::new(1.0, "kitten", 21_usize, 27_usize),
            ],
        ),
        Match::new(
            0.9600222073146616,
            "I desperately need a mitten",
            vec![
                LevenshteinMatch::new(1.0, "need", 14_usize, 18_usize),
                LevenshteinMatch::new(0.9200444146293232, "mitten", 21_usize, 27_usize),
            ],
        ),
    ];

    assert_eq!(actual, expected);
}
