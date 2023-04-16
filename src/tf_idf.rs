// Copyright (C) 2023 Afonso Barracha
//
// This file is part of keyword-extraction.
//
// keyword-extraction is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// keyword-extraction is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with keyword-extraction.  If not, see <http://www.gnu.org/licenses/>.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub struct TfIdf(HashMap<String, f32>);

fn generate_word_hashmap(documents: &[String]) -> HashMap<String, f32> {
    documents
        .iter()
        .flat_map(|document| document.split_whitespace().map(|word| word.to_string()))
        .fold(HashMap::new(), |mut acc, word| {
            let count = acc.entry(word).or_insert(0.0);
            *count += 1.0;
            acc
        })
}

fn generate_unique_word_hashmap(documents: &[String]) -> HashMap<String, f32> {
    documents
        .iter()
        .map(|document| {
            document
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>()
        })
        .flat_map(|unique_words| unique_words.into_iter())
        .fold(HashMap::new(), |mut acc, word| {
            let count = acc.entry(word).or_insert(0.0);
            *count += 1.0;
            acc
        })
}

fn calculate_tf(tf: HashMap<String, f32>) -> HashMap<String, f32> {
    let total_words = tf.values().sum::<f32>();

    tf.iter()
        .map(|(word, count)| (word.to_string(), count / total_words))
        .collect::<HashMap<String, f32>>()
}

fn calculate_idf(docs_len: f32, word_hashmap: HashMap<String, f32>) -> HashMap<String, f32> {
    let one = 1.0_f32;

    word_hashmap
        .iter()
        .map(|(word, count)| {
            let documents_with_term = (docs_len + one) / (count + one);
            (word.to_string(), documents_with_term.ln() + one)
        })
        .collect::<HashMap<String, f32>>()
}

fn calculate_tf_idf(tf: HashMap<String, f32>, idf: HashMap<String, f32>) -> HashMap<String, f32> {
    tf.iter()
        .map(|(word, count)| (word.to_string(), count * idf.get(word).unwrap_or(&0.0_f32)))
        .collect::<HashMap<String, f32>>()
}

fn l2_normalize(tf_id: HashMap<String, f32>) -> HashMap<String, f32> {
    let l2_norm = tf_id
        .values()
        .map(|value| value * value)
        .sum::<f32>()
        .sqrt();

    tf_id
        .iter()
        .map(|(key, value)| (key.clone(), value / l2_norm))
        .collect::<HashMap<String, f32>>()
}

impl TfIdf {
    pub fn new(documents: &[String]) -> Self {
        Self(l2_normalize(calculate_tf_idf(
            calculate_tf(generate_word_hashmap(documents)),
            calculate_idf(
                documents.len() as f32,
                generate_unique_word_hashmap(documents),
            ),
        )))
    }

    pub fn get_score(&self, word: &str) -> f32 {
        *self.0.get(word).unwrap_or(&0.0)
    }

    pub fn get_n_best(&self, n: usize) -> Vec<(String, f32)> {
        let mut sorted_tfidf = self.0.iter().collect::<Vec<(&String, &f32)>>();
        sorted_tfidf.sort_by(|a, b| {
            let order = b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal);

            if order == Ordering::Equal {
                return a.0.cmp(&b.0);
            }

            order
        });
        sorted_tfidf
            .iter()
            .take(n)
            .map(|(word, score)| (word.to_string(), **score))
            .collect::<Vec<(String, f32)>>()
    }
}
