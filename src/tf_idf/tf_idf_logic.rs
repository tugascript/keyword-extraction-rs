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

use std::collections::{HashMap, HashSet};

pub struct TfIdfLogic;

impl TfIdfLogic {
    pub fn build_tfidf(documents: &[String]) -> HashMap<String, f32> {
        Self::l2_normalize(Self::calculate_tf_idf(
            Self::calculate_tf(Self::generate_word_hashmap(documents)),
            Self::calculate_idf(
                documents.len() as f32,
                Self::generate_unique_word_hashmap(documents),
            ),
        ))
    }

    fn generate_word_hashmap(documents: &[String]) -> HashMap<&str, f32> {
        documents
            .iter()
            .flat_map(|document| document.split_whitespace())
            .fold(HashMap::new(), |mut acc, word| {
                let count = acc.entry(word).or_insert(0.0);
                *count += 1.0;
                acc
            })
    }

    fn generate_unique_word_hashmap(documents: &[String]) -> HashMap<&str, f32> {
        documents
            .iter()
            .map(|document| document.split_whitespace().collect::<HashSet<&str>>())
            .flat_map(|unique_words| unique_words.into_iter())
            .fold(HashMap::new(), |mut acc, word| {
                let count = acc.entry(word).or_insert(0.0);
                *count += 1.0;
                acc
            })
    }

    fn calculate_tf<'a>(tf: HashMap<&'a str, f32>) -> HashMap<&'a str, f32> {
        let total_words = tf.values().sum::<f32>();

        tf.iter()
            .map(|(word, count)| (*word, count / total_words))
            .collect::<HashMap<&'a str, f32>>()
    }

    fn calculate_idf<'a>(
        docs_len: f32,
        word_hashmap: HashMap<&'a str, f32>,
    ) -> HashMap<&'a str, f32> {
        let one = 1.0_f32;

        word_hashmap
            .iter()
            .map(|(word, count)| {
                let documents_with_term = (docs_len + one) / (count + one);
                (*word, documents_with_term.ln() + one)
            })
            .collect::<HashMap<&'a str, f32>>()
    }

    fn calculate_tf_idf<'a>(
        tf: HashMap<&'a str, f32>,
        idf: HashMap<&'a str, f32>,
    ) -> HashMap<&'a str, f32> {
        tf.iter()
            .map(|(word, count)| (*word, count * idf.get(word).unwrap_or(&0.0_f32)))
            .collect::<HashMap<&'a str, f32>>()
    }

    fn l2_normalize(tf_id: HashMap<&str, f32>) -> HashMap<String, f32> {
        let l2_norm = tf_id
            .values()
            .map(|value| value * value)
            .sum::<f32>()
            .sqrt();

        tf_id
            .iter()
            .map(|(key, value)| (key.to_string(), value / l2_norm))
            .collect::<HashMap<String, f32>>()
    }
}
