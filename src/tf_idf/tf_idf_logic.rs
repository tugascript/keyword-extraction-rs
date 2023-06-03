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

#[cfg(feature = "parallel")]
use rayon::prelude::*;

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
        #[cfg(feature = "parallel")]
        return Self::parallel_word_hashmap(documents);

        #[cfg(not(feature = "parallel"))]
        documents
            .iter()
            .flat_map(|document| document.split_whitespace())
            .fold(HashMap::new(), |mut acc, word| {
                let count = acc.entry(word).or_insert(0.0);
                *count += 1.0;
                acc
            })
    }

    #[cfg(feature = "parallel")]
    fn parallel_word_hashmap(documents: &[String]) -> HashMap<&str, f32> {
        documents
            .par_iter()
            .fold(
                || HashMap::new(),
                |mut acc, document| {
                    document
                        .split_whitespace()
                        .for_each(|word| *acc.entry(word).or_insert(0.0) += 1.0);
                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut acc, hmap| {
                    for (word, count) in hmap {
                        *acc.entry(word).or_insert(0.0) += count;
                    }
                    acc
                },
            )
    }

    fn generate_unique_word_hashmap(documents: &[String]) -> HashMap<&str, f32> {
        #[cfg(feature = "parallel")]
        return Self::parallel_unique_word_hashmap(documents);

        #[cfg(not(feature = "parallel"))]
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

    #[cfg(feature = "parallel")]
    fn parallel_unique_word_hashmap(documents: &[String]) -> HashMap<&str, f32> {
        documents
            .par_iter()
            .map(|document| document.split_whitespace().collect::<HashSet<&str>>())
            .fold(
                || HashMap::new(),
                |mut acc, unique_words| {
                    unique_words
                        .into_iter()
                        .for_each(|word| *acc.entry(word).or_insert(0.0) += 1.0);
                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut acc, hmap| {
                    for (word, count) in hmap {
                        *acc.entry(word).or_insert(0.0) += count;
                    }
                    acc
                },
            )
    }

    fn calculate_tf<'a>(tf: HashMap<&'a str, f32>) -> HashMap<&'a str, f32> {
        #[cfg(feature = "parallel")]
        return Self::parallel_tf(tf);

        #[cfg(not(feature = "parallel"))]
        let total_words = tf.values().sum::<f32>();

        #[cfg(not(feature = "parallel"))]
        tf.iter()
            .map(|(word, count)| (*word, count / total_words))
            .collect::<HashMap<&'a str, f32>>()
    }

    #[cfg(feature = "parallel")]
    fn parallel_tf<'a>(tf: HashMap<&'a str, f32>) -> HashMap<&'a str, f32> {
        let total_words = tf.values().sum::<f32>();

        tf.par_iter()
            .map(|(word, count)| (*word, count / total_words))
            .collect::<HashMap<&'a str, f32>>()
    }

    fn calculate_idf<'a>(
        docs_len: f32,
        word_hashmap: HashMap<&'a str, f32>,
    ) -> HashMap<&'a str, f32> {
        #[cfg(feature = "parallel")]
        return Self::parallel_idf(docs_len, word_hashmap);

        #[cfg(not(feature = "parallel"))]
        word_hashmap
            .iter()
            .map(|(word, count)| {
                let documents_with_term = (docs_len + 1.0_f32) / (count + 1.0_f32);
                (*word, documents_with_term.ln() + 1.0_f32)
            })
            .collect::<HashMap<&'a str, f32>>()
    }

    #[cfg(feature = "parallel")]
    fn parallel_idf<'a>(
        docs_len: f32,
        word_hashmap: HashMap<&'a str, f32>,
    ) -> HashMap<&'a str, f32> {
        word_hashmap
            .par_iter()
            .map(|(word, count)| {
                let documents_with_term = (docs_len + 1.0_f32) / (count + 1.0_f32);
                (*word, documents_with_term.ln() + 1.0_f32)
            })
            .collect::<HashMap<&'a str, f32>>()
    }

    fn calculate_tf_idf<'a>(
        tf: HashMap<&'a str, f32>,
        idf: HashMap<&'a str, f32>,
    ) -> HashMap<&'a str, f32> {
        #[cfg(feature = "parallel")]
        return Self::parallel_tf_idf(tf, idf);

        #[cfg(not(feature = "parallel"))]
        tf.iter()
            .map(|(word, count)| (*word, count * idf.get(word).unwrap_or(&0.0_f32)))
            .collect::<HashMap<&'a str, f32>>()
    }

    #[cfg(feature = "parallel")]
    fn parallel_tf_idf<'a>(
        tf: HashMap<&'a str, f32>,
        idf: HashMap<&'a str, f32>,
    ) -> HashMap<&'a str, f32> {
        tf.par_iter()
            .map(|(word, count)| (*word, count * idf.get(word).unwrap_or(&0.0_f32)))
            .collect::<HashMap<&'a str, f32>>()
    }

    fn l2_normalize(tf_id: HashMap<&str, f32>) -> HashMap<String, f32> {
        #[cfg(feature = "parallel")]
        return Self::parallel_l2_normalize(tf_id);

        #[cfg(not(feature = "parallel"))]
        let l2_norm = tf_id
            .values()
            .map(|value| value * value)
            .sum::<f32>()
            .sqrt();

        #[cfg(not(feature = "parallel"))]
        tf_id
            .iter()
            .map(|(key, value)| (key.to_string(), value / l2_norm))
            .collect::<HashMap<String, f32>>()
    }

    #[cfg(feature = "parallel")]
    fn parallel_l2_normalize(tf_id: HashMap<&str, f32>) -> HashMap<String, f32> {
        let l2_norm = tf_id
            .par_iter()
            .map(|(_, value)| value * value)
            .sum::<f32>()
            .sqrt();

        tf_id
            .par_iter()
            .map(|(key, value)| (key.to_string(), value / l2_norm))
            .collect::<HashMap<String, f32>>()
    }
}
