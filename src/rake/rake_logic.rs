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

pub struct RakeLogic;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::common::{PhraseLength, Punctuation, Stopwords, Text};
use crate::tokenizer::Tokenizer;
use std::collections::HashMap;

fn str_to_strig_vector(text: &str) -> Vec<String> {
    text.split_whitespace().map(|w| w.to_string()).collect()
}

fn calculate_word_score(
    word: &str,
    frequency: &f32,
    word_degree: &HashMap<&str, f32>,
) -> (String, f32) {
    let degree = word_degree.get(word).unwrap_or(&0.0);
    (word.to_string(), degree / frequency)
}

fn calculate_phrase_score(phrase: &[String], word_scores: &HashMap<String, f32>) -> (String, f32) {
    let score = phrase
        .iter()
        .map(|word| word_scores.get(word).unwrap_or(&0.0))
        .sum::<f32>();
    (phrase.join(" "), score / phrase.len() as f32)
}

impl RakeLogic {
    pub fn build_rake(
        text: Text,
        stopwords: Stopwords,
        punctuation: Punctuation,
        phrase_len: PhraseLength,
    ) -> (HashMap<String, f32>, HashMap<String, f32>) {
        let phrases = Self::split_into_phrases(text, stopwords, punctuation, phrase_len);
        let word_scores = Self::calculate_word_scores(
            Self::generate_word_frequency(&phrases),
            Self::generate_word_degree(&phrases),
        );
        let phrase_scores = Self::calculate_phrase_scores(&phrases, &word_scores);
        (word_scores, phrase_scores)
    }

    fn split_into_phrases(
        text: &str,
        stopwords: Stopwords,
        punctuation: Punctuation,
        length: PhraseLength,
    ) -> Vec<Vec<String>> {
        let phrases = Tokenizer::new(text, stopwords, punctuation).split_into_phrases(length);

        #[cfg(feature = "parallel")]
        {
            phrases
                .par_iter()
                .map(|sentence| str_to_strig_vector(sentence))
                .collect::<Vec<Vec<String>>>()
        }

        #[cfg(not(feature = "parallel"))]
        {
            phrases
                .iter()
                .map(|sentence| str_to_strig_vector(sentence))
                .collect::<Vec<Vec<String>>>()
        }
    }

    fn generate_word_frequency(phrases: &[Vec<String>]) -> HashMap<&str, f32> {
        #[cfg(feature = "parallel")]
        {
            Self::parallel_word_frequency(phrases)
        }

        #[cfg(not(feature = "parallel"))]
        {
            Self::basic_word_frequency(phrases)
        }
    }

    #[cfg(not(feature = "parallel"))]
    fn basic_word_frequency(phrases: &[Vec<String>]) -> HashMap<&str, f32> {
        phrases
            .iter()
            .flat_map(|phrase| phrase.iter())
            .fold(HashMap::new(), |mut acc, word| {
                *acc.entry(word).or_insert(0.0) += 1.0;
                acc
            })
    }

    #[cfg(feature = "parallel")]
    fn parallel_word_frequency(phrases: &[Vec<String>]) -> HashMap<&str, f32> {
        phrases
            .par_iter()
            .fold(
                HashMap::<&str, f32>::new,
                |mut acc, phrase| {
                    phrase.iter().for_each(|word| {
                        *acc.entry(word).or_insert(0.0) += 1.0;
                    });
                    acc
                },
            )
            .reduce(
                HashMap::<&str, f32>::new,
                |mut acc, hmap| {
                    hmap.iter().for_each(|(word, count)| {
                        *acc.entry(word).or_insert(0.0) += count;
                    });
                    acc
                },
            )
    }

    fn generate_word_degree(phrases: &[Vec<String>]) -> HashMap<&str, f32> {
        #[cfg(feature = "parallel")]
        {
            Self::parallel_word_degree(phrases)
        }

        #[cfg(not(feature = "parallel"))]
        {
            Self::basic_word_degree(phrases)
        }
    }

    #[cfg(not(feature = "parallel"))]
    fn basic_word_degree(phrases: &[Vec<String>]) -> HashMap<&str, f32> {
        phrases
            .iter()
            .flat_map(|phrase| phrase.iter().map(|word| (phrase.len() as f32 - 1.0, word)))
            .fold(HashMap::new(), |mut acc, (len, word)| {
                acc.entry(word)
                    .and_modify(|count| *count += len)
                    .or_insert(len);

                acc
            })
    }

    #[cfg(feature = "parallel")]
    fn parallel_word_degree(phrases: &[Vec<String>]) -> HashMap<&str, f32> {
        phrases
            .par_iter()
            .fold(
                HashMap::<&str, f32>::new,
                |mut acc, phrase| {
                    let len = phrase.len() as f32 - 1.0;
                    phrase.iter().for_each(|word| {
                        acc.entry(word)
                            .and_modify(|count| *count += len)
                            .or_insert(len);
                    });
                    acc
                },
            )
            .reduce(
                HashMap::<&str, f32>::new,
                |mut acc, hmap| {
                    hmap.iter().for_each(|(word, degree)| {
                        *acc.entry(word).or_insert(0.0) += degree;
                    });
                    acc
                },
            )
    }

    fn calculate_word_scores(
        word_frequency: HashMap<&str, f32>,
        word_degree: HashMap<&str, f32>,
    ) -> HashMap<String, f32> {
        #[cfg(feature = "parallel")]
        {
            word_frequency
                .par_iter()
                .map(|(word, frequency)| calculate_word_score(word, frequency, &word_degree))
                .collect::<HashMap<String, f32>>()
        }

        #[cfg(not(feature = "parallel"))]
        {
            word_frequency
                .iter()
                .map(|(word, frequency)| calculate_word_score(word, frequency, &word_degree))
                .collect::<HashMap<String, f32>>()
        }
    }

    fn calculate_phrase_scores(
        phrases: &[Vec<String>],
        word_scores: &HashMap<String, f32>,
    ) -> HashMap<String, f32> {
        #[cfg(feature = "parallel")]
        {
            phrases
                .par_iter()
                .map(|phrase| calculate_phrase_score(phrase, word_scores))
                .collect::<HashMap<String, f32>>()
        }

        #[cfg(not(feature = "parallel"))]
        {
            phrases
                .iter()
                .map(|phrase| calculate_phrase_score(phrase, word_scores))
                .collect::<HashMap<String, f32>>()
        }
    }
}
