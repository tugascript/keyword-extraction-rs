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

use crate::common::{Stopwords, Text};
use crate::tokenizer::Tokenizer;
use std::collections::HashMap;

impl RakeLogic {
    pub fn build_rake(
        text: Text,
        stopwords: Stopwords,
    ) -> (HashMap<String, f32>, HashMap<String, f32>) {
        let phrases = Self::split_into_phrases(text, stopwords);
        let word_scores = Self::calculate_word_scores(
            Self::generate_word_frequency(&phrases),
            Self::generate_word_degree(&phrases),
        );
        let phrase_scores = Self::calculate_phrase_scores(&phrases, &word_scores);
        (word_scores, phrase_scores)
    }

    fn split_into_phrases(text: &str, stopwords: &[String]) -> Vec<Vec<String>> {
        Tokenizer::new(text, stopwords, None)
            .split_into_phrases()
            .iter()
            .map(|sentence| {
                sentence
                    .split_whitespace()
                    .map(|w| w.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
    }

    fn generate_word_frequency(phrases: &[Vec<String>]) -> HashMap<&str, f32> {
        phrases
            .iter()
            .flat_map(|phrase| phrase.iter())
            .fold(HashMap::new(), |mut acc, word| {
                let count = acc.entry(word).or_insert(0.0);
                *count += 1.0;
                acc
            })
    }

    fn generate_word_degree(phrases: &[Vec<String>]) -> HashMap<&str, f32> {
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

    fn calculate_word_scores(
        word_frequency: HashMap<&str, f32>,
        word_degree: HashMap<&str, f32>,
    ) -> HashMap<String, f32> {
        word_frequency
            .iter()
            .map(|(word, frequency)| {
                let degree = word_degree.get(word).unwrap_or(&0.0);
                (word.to_string(), degree / frequency)
            })
            .collect::<HashMap<String, f32>>()
    }

    fn calculate_phrase_scores(
        phrases: &[Vec<String>],
        word_scores: &HashMap<String, f32>,
    ) -> HashMap<String, f32> {
        phrases
            .iter()
            .map(|phrase| {
                let score = phrase
                    .iter()
                    .map(|word| word_scores.get(word).unwrap_or(&0.0))
                    .sum::<f32>();
                (phrase.join(" "), score)
            })
            .collect::<HashMap<String, f32>>()
    }
}
