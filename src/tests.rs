// Copyright (C) 2023 Afonso Barracha
//
// Rust Keyword Extraction is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rust Keyword Extraction is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Rust Keyword Extraction. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashSet;

use stop_words::{get, LANGUAGE};

use crate::*;

const TEXT: &str = r#"
Title: Junior Rust Developer

Job Description:

We are seeking a talented and motivated Junior Rust Developer to join our growing team. The ideal candidate will have a passion for programming, a strong foundation in Rust, and a desire to learn and grow in a dynamic work environment.

Responsibilities:

Assist in the development and maintenance of our core Rust applications
Write clean, efficient, and well-documented code
Collaborate with the development team to design and implement new features
Actively participate in code reviews and provide constructive feedback
Continuously learn and stay up-to-date with the latest Rust ecosystem trends and technologies

Requirements:

Bachelor's degree in Computer Science or related field, or equivalent experience
Proficiency in Rust programming language
Familiarity with version control systems, preferably Git
Strong problem-solving and debugging skills
Excellent written and verbal communication skills
Ability to work well in a team-oriented environment

Nice-to-Haves:

Experience with other programming languages, such as Python, JavaScript, or C++
Knowledge of database systems, like PostgreSQL or MongoDB
Familiarity with web development frameworks, such as Actix or Rocket

What We Offer:

Competitive salary and benefits package
Opportunity for growth and career advancement
Supportive and collaborative work environment
Chance to work on cutting-edge projects using Rust

If you are passionate about Rust development and looking to kickstart your career in a supportive and dynamic environment, we encourage you to apply!
"#;

fn get_cs_hashset() -> HashSet<String> {
    HashSet::from_iter(vec!["c", "computer"].iter().map(|s| s.to_string()))
}

fn get_stop_words() -> Vec<String> {
    let cs_hashset = get_cs_hashset();
    get(LANGUAGE::English)
        .iter()
        .filter_map(|w| {
            let word = w.replace("\"", "");
            if !cs_hashset.contains(&word) {
                Some(word)
            } else {
                None
            }
        })
        .collect()
}

fn is_percent_in_hashset(vector: &[String], hashset: &HashSet<String>, percent: f64) -> bool {
    let mut count = 0;

    for item in vector {
        if hashset.contains(item) {
            count += 1;
        }
    }

    let percentage = (count as f64 / vector.len() as f64) * 100.0;
    percentage >= percent
}

#[test]
fn test_tokenize() {
    let tokenizer = tokenizer::Tokenizer::new(TEXT, &get_stop_words(), None);
    let sentence_tokens = tokenizer.split_into_sentences();
    let expected_sentences = vec![
        "title junior rust developer",
        "job description",
        "seeking talented motivated junior rust developer growing team",
        "ideal candidate passion programming strong foundation rust desire learn grow dynamic environment",
        "responsibilities",
        "assist development maintenance core rust applications",
        "write clean efficient documented code",
        "collaborate development team design implement features",
        "actively participate code reviews provide constructive feedback",
        "continuously learn stay rust ecosystem trends technologies",
        "requirements",
        "bachelor degree computer science field equivalent experience",
        "proficiency rust programming language",
        "familiarity version control systems preferably git",
        "strong solving debugging skills",
        "excellent written verbal communication skills",
        "ability team oriented environment",
        "nice haves",
        "experience programming languages python javascript c",
        "knowledge database systems postgresql mongodb",
        "familiarity development frameworks actix rocket",
        "offer",
        "competitive salary benefits package",
        "opportunity growth career advancement",
        "supportive collaborative environment",
        "chance cutting edge projects rust",
        "passionate rust development kickstart career supportive dynamic environment encourage apply",
    ].iter().map(|s| s.to_string()).collect::<HashSet<String>>();

    let word_tokens = tokenizer.split_into_words();
    let expected_words = vec![
        "title",
        "junior",
        "rust",
        "developer",
        "job",
        "description",
        "seeking",
        "talented",
        "motivated",
        "junior",
        "rust",
        "developer",
        "growing",
        "team",
        "ideal",
        "candidate",
        "passion",
        "programming",
        "strong",
        "foundation",
        "rust",
        "desire",
        "learn",
        "grow",
        "dynamic",
        "environment",
        "responsibilities",
        "assist",
        "development",
        "maintenance",
        "core",
        "rust",
        "applications",
        "write",
        "clean",
        "efficient",
        "documented",
        "code",
        "collaborate",
        "development",
        "team",
        "design",
        "implement",
        "features",
        "actively",
        "participate",
        "code",
        "reviews",
        "provide",
        "constructive",
        "feedback",
        "continuously",
        "learn",
        "stay",
        "rust",
        "ecosystem",
        "trends",
        "technologies",
        "requirements",
        "bachelor",
        "degree",
        "computer",
        "science",
        "field",
        "equivalent",
        "experience",
        "proficiency",
        "rust",
        "programming",
        "language",
        "familiarity",
        "version",
        "control",
        "systems",
        "preferably",
        "git",
        "strong",
        "solving",
        "debugging",
        "skills",
        "excellent",
        "written",
        "verbal",
        "communication",
        "skills",
        "ability",
        "team",
        "oriented",
        "environment",
        "nice",
        "haves",
        "experience",
        "programming",
        "languages",
        "python",
        "javascript",
        "c",
        "knowledge",
        "database",
        "systems",
        "postgresql",
        "mongodb",
        "familiarity",
        "development",
        "frameworks",
        "actix",
        "rocket",
        "offer",
        "competitive",
        "salary",
        "benefits",
        "package",
        "opportunity",
        "growth",
        "career",
        "advancement",
        "supportive",
        "collaborative",
        "environment",
        "chance",
        "cutting",
        "edge",
        "projects",
        "rust",
        "passionate",
        "rust",
        "development",
        "kickstart",
        "career",
        "supportive",
        "dynamic",
        "environment",
        "encourage",
        "apply",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect::<HashSet<String>>();

    let paragraph_tokens = tokenizer.split_into_paragraphs();
    let expected_paragraphs = vec![
        "title junior rust developer",
        "job description",
        "seeking talented motivated junior rust developer growing team ideal candidate passion programming strong foundation rust desire learn grow dynamic environment",
        "responsibilities",
        "assist development maintenance core rust applications",
        "write clean efficient documented code",
        "collaborate development team design implement features",
        "actively participate code reviews provide constructive feedback",
        "continuously learn stay rust ecosystem trends technologies",
        "requirements",
        "bachelor degree computer science field equivalent experience",
        "proficiency rust programming language",
        "familiarity version control systems preferably git",
        "strong solving debugging skills",
        "excellent written verbal communication skills",
        "ability team oriented environment",
        "nice haves",
        "experience programming languages python javascript c",
        "knowledge database systems postgresql mongodb",
        "familiarity development frameworks actix rocket",
        "offer", "competitive salary benefits package",
        "opportunity growth career advancement",
        "supportive collaborative environment",
        "chance cutting edge projects rust",
        "passionate rust development kickstart career supportive dynamic environment encourage apply",
    ].iter().map(|s| s.to_string()).collect::<HashSet<String>>();

    assert!(is_percent_in_hashset(
        &sentence_tokens,
        &expected_sentences,
        90.0
    ));
    assert!(is_percent_in_hashset(&word_tokens, &expected_words, 95.0));
    assert!(is_percent_in_hashset(
        &paragraph_tokens,
        &expected_paragraphs,
        95.0
    ));
}

#[test]
fn test_tf_idf() {
    let tf_idf = tf_idf::TfIdf::new(tf_idf::TfIdfParams::TextBlock(
        TEXT,
        &get_stop_words(),
        None,
        tf_idf::TextSplit::Paragraphs,
    ));
    let words_result = tf_idf.get_ranked_words(100);
    let expected_words = vec![
        "rust",
        "development",
        "environment",
        "work",
        "programming",
        "team",
        "career",
        "code",
        "developer",
        "dynamic",
        "experience",
        "familiarity",
        "junior",
        "learn",
        "skills",
        "strong",
        "supportive",
        "systems",
        "to",
        "well",
        "ability",
        "actively",
        "actix",
        "advancement",
        "applications",
        "apply",
        "assist",
        "bachelor",
        "benefits",
        "candidate",
        "chance",
        "clean",
        "collaborate",
        "collaborative",
        "communication",
        "competitive",
        "computer",
        "constructive",
        "continuously",
        "control",
        "core",
        "cutting",
        "database",
        "date",
        "debugging",
        "degree",
        "description",
        "design",
        "desire",
        "documented",
        "ecosystem",
        "edge",
        "efficient",
        "encourage",
        "equivalent",
        "excellent",
        "features",
        "feedback",
        "field",
        "foundation",
        "frameworks",
        "git",
        "grow",
        "growing",
        "growth",
        "haves",
        "ideal",
        "implement",
        "javascript",
        "job",
        "join",
        "kickstart",
        "knowledge",
        "language",
        "languages",
        "latest",
        "like",
        "looking",
        "maintenance",
        "mongodb",
        "motivated",
        "new",
        "nice",
        "offer",
        "opportunity",
        "oriented",
        "package",
        "participate",
        "passion",
        "passionate",
        "postgresql",
        "preferably",
        "problem",
        "proficiency",
        "projects",
        "provide",
        "python",
        "related",
        "requirements",
        "responsibilities",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<HashSet<String>>();
    assert!(is_percent_in_hashset(&words_result, &expected_words, 85.0));
}

#[cfg(feature = "co_occurrence")]
#[test]
fn test_co_occurrence() {
    let documents =
        tokenizer::Tokenizer::new(TEXT, &get_stop_words(), None).split_into_paragraphs();
    let word_vec = vec![
        "rust",
        "development",
        "environment",
        "work",
        "programming",
        "team",
        "career",
        "code",
        "developer",
        "dynamic",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>();
    let co_occurrence = co_occurrence::CoOccurrence::new(&documents, &word_vec, 10);
    assert_eq!(
        co_occurrence.get_matrix_row("rust").unwrap(),
        [0.6666667, 0.6666667, 0.6666667, 0.0, 1.0, 0.6666667, 0.33333334, 0.0, 1.0, 0.6666667]
    );
    assert_eq!(
        co_occurrence.get_matrix_row("development").unwrap(),
        [0.6666667, 0.0, 0.33333334, 0.0, 0.0, 0.33333334, 0.33333334, 0.0, 0.0, 0.33333334]
    );
    assert_eq!(
        co_occurrence.get_matrix_row("developer").unwrap(),
        [1.0, 0.0, 0.0, 0.0, 0.33333334, 0.33333334, 0.0, 0.0, 0.0, 0.0]
    );
    assert_eq!(
        co_occurrence.get_matrix_row("dynamic").unwrap(),
        [0.6666667, 0.33333334, 0.6666667, 0.0, 0.33333334, 0.0, 0.33333334, 0.0, 0.0, 0.0]
    );
}

#[test]
fn test_rake() {
    let rake_result = [
        "core rust applications write clean efficient",
        "version control systems preferably git strong",
        "title junior rust developer job description",
        "provide constructive feedback continuously learn",
        "motivated junior rust developer",
        "debugging skills excellent written",
        "technologies requirements bachelor degree",
        "verbal communication skills ability",
        "team oriented environment nice",
        "rust programming language familiarity",
    ];
    let rake_struct = rake::Rake::new(rake::RakeParams::WithDefaults(TEXT, &get_stop_words()));
    assert!(is_percent_in_hashset(
        &rake_struct.get_ranked_phrases(10),
        &rake_result
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>(),
        90.0
    ));

    let limited_rake_struct = rake::Rake::new(rake::RakeParams::WithDefaultsAndPhraseLength(
        TEXT,
        &get_stop_words(),
        Some(3),
    ));
    for phrase in limited_rake_struct.get_ranked_phrases(10) {
        assert!(phrase.split_whitespace().count() <= 3);
    }
}

#[test]
fn test_text_rank() {
    let expected_words = [
        "rust",
        "environment",
        "development",
        "team",
        "programming",
        "code",
        "systems",
        "skills",
        "experience",
        "familiarity",
    ];
    let text_rank = text_rank::TextRank::new(text_rank::TextRankParams::WithDefaults(
        TEXT,
        &get_stop_words(),
    ));
    assert!(is_percent_in_hashset(
        &text_rank.get_ranked_words(10),
        &expected_words
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>(),
        90.0
    ));
    assert!(is_percent_in_hashset(
        &text_rank
            .get_ranked_phrases(5)
            .iter()
            .flat_map(|phrases| phrases.split_whitespace().map(|w| w.to_string()))
            .collect::<Vec<String>>(),
        &text_rank
            .get_ranked_words(10)
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>(),
        90.0
    ));

    let limited_text_rank = text_rank::TextRank::new(
        text_rank::TextRankParams::WithDefaultsAndPhraseLength(TEXT, &get_stop_words(), Some(3)),
    );
    for phrase in limited_text_rank.get_ranked_phrases(10) {
        assert!(phrase.split_whitespace().count() <= 3);
    }
}

#[cfg(feature = "yake")]
#[test]
fn test_yake() {
    let yake = yake::Yake::new(yake::YakeParams::WithDefaults(TEXT, &get_stop_words()));
    let yake_result = [
        "motivated junior rust",
        "motivated junior",
        "job description",
        "developer job description",
        "rust developer",
        "developer job",
        "junior rust developer",
        "rust developer job",
        "junior rust",
        "rust programming language",
    ];
    println!("{:?}", yake.get_ranked_keyword_scores(20));
    assert!(is_percent_in_hashset(
        &yake.get_ranked_keywords(10),
        &yake_result
            .iter()
            .map(|x| x.to_string())
            .collect::<HashSet<String>>(),
        90.0
    ));
}
