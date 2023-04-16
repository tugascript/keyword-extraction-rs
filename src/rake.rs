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

use std::{cmp::Ordering, collections::HashMap};

use crate::tokenizer::Tokenizer;

pub struct Rake {
    word_scores: HashMap<String, f32>,
    phrase_scores: HashMap<String, f32>,
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

fn generate_word_frequency(phrases: &[Vec<String>]) -> HashMap<String, f32> {
    phrases
        .iter()
        .flat_map(|phrase| phrase.iter().map(|word| word.to_string()))
        .fold(HashMap::new(), |mut acc, word| {
            let count = acc.entry(word).or_insert(0.0);
            *count += 1.0;
            acc
        })
}

fn generate_word_degree(phrases: &[Vec<String>]) -> HashMap<String, f32> {
    phrases
        .iter()
        .flat_map(|phrase| {
            phrase
                .iter()
                .map(|word| (phrase.len() as f32 - 1.0, word.to_string()))
        })
        .fold(HashMap::new(), |mut acc, (len, word)| {
            acc.entry(word)
                .and_modify(|count| *count += len)
                .or_insert(len);

            acc
        })
}

fn calculate_word_scores(
    word_frequency: HashMap<String, f32>,
    word_degree: HashMap<String, f32>,
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

impl Rake {
    pub fn new(text: &str, stopwords: &Vec<String>) -> Self {
        let phrases = split_into_phrases(text, stopwords);
        let word_scores = calculate_word_scores(
            generate_word_frequency(&phrases),
            generate_word_degree(&phrases),
        );

        Self {
            phrase_scores: calculate_phrase_scores(&phrases, &word_scores),
            word_scores,
        }
    }

    pub fn get_ranked_keyword(&self, n: usize) -> Vec<String> {
        let mut keywords = self.word_scores.iter().collect::<Vec<(&String, &f32)>>();
        keywords.sort_by(|a, b| {
            let order = b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal);

            if order == Ordering::Equal {
                return a.0.cmp(&b.0);
            }

            order
        });
        keywords
            .iter()
            .take(n)
            .map(|(word, _)| word.to_string())
            .collect::<Vec<String>>()
    }

    pub fn get_ranked_phrases(&self, n: usize) -> Vec<String> {
        let mut phrases = self.phrase_scores.iter().collect::<Vec<(&String, &f32)>>();
        phrases.sort_by(|a, b| {
            let order = b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal);

            if order == Ordering::Equal {
                return a.0.len().cmp(&b.0.len());
            }

            order
        });
        phrases
            .iter()
            .take(n)
            .map(|(phrase, _)| phrase.to_string())
            .collect::<Vec<String>>()
    }

    pub fn get_keyword_score(&self, word: &str) -> f32 {
        *self.word_scores.get(word).unwrap_or(&0.0)
    }

    pub fn get_phrase_score(&self, phrase: &str) -> f32 {
        *self.phrase_scores.get(phrase).unwrap_or(&0.0)
    }
}
