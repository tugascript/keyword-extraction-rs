// Copyright (C) 2024 Afonso Barracha
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

use unicode_segmentation::UnicodeSegmentation;

use crate::common::{get_capitalized_regex, get_upper_case_regex};

use super::{context_builder::Context, yake_tokenizer::Sentence};

#[derive(Debug, Default)]
pub struct Features {
    tf: f32,
    tf_capitalized: f32,
    tf_all_upper: f32,
    casing: f32,
    position: f32,
    frequency: f32,
    wl: f32,
    wr: f32,
    different: f32,
    relatedness: f32,
    weight: f32,
}

impl Features {
    pub fn get_weight(&self) -> f32 {
        self.weight
    }
}

pub struct FeatureExtraction<'a>(HashMap<&'a str, Features>);

impl<'a> FeatureExtraction<'a> {
    pub fn new(
        context: &'a Context<'a>,
        sentences: &'a [Sentence<'a>],
        stop_words: &'a HashSet<&'a str>,
    ) -> Self {
        let tf_values = context
            .occurrences()
            .filter_map(|(w, o)| {
                if stop_words.contains(w.as_str()) {
                    return None;
                }
                Some(o.len())
            })
            .collect::<Vec<usize>>();
        let tf_total = tf_values.iter().sum::<usize>() as f32;
        let tf_mean = tf_total / (tf_values.len() as f32 + f32::EPSILON);
        let tf_std = tf_values
            .iter()
            .fold(0.0_f32, |a, v| a + (*v as f32 - tf_mean).powi(2))
            .sqrt();
        let tf_max = tf_values
            .iter()
            .max()
            .map(|x| *x as f32)
            .unwrap_or(f32::EPSILON);
        let sentence_len = sentences.len() as f32;

        Self(context.occurrences().fold(
            HashMap::<&str, Features>::new(),
            |mut acc, (word, occurrences)| {
                let word = word.as_str();
                let mut features = Features {
                    tf: occurrences.len() as f32,
                    ..Default::default()
                };

                let all_upper_check = |w: &str| {
                    let is_large = w.graphemes(true).count() > 1;
                    let upper_regex = get_upper_case_regex();
                    match upper_regex {
                        Some(r) => is_large && r.is_match(w),
                        None => is_large && w.to_uppercase().as_str() == w,
                    }
                };
                let capitalized_check = |w: &str| {
                    let capitalized_regex = get_capitalized_regex();
                    match capitalized_regex {
                        Some(r) => r.is_match(w),
                        None => {
                            w.chars().next().unwrap().is_uppercase()
                                && w.chars().skip(1).all(char::is_lowercase)
                        }
                    }
                };

                occurrences.iter().for_each(|occurrence| {
                    let w = occurrence.get_word();
                    features.tf_all_upper += if all_upper_check(w) { 1.0 } else { 0.0 };
                    features.tf_capitalized += if capitalized_check(w)
                        && occurrence.get_shift() != occurrence.get_shift_offset()
                    {
                        1.0
                    } else {
                        0.0
                    };
                });

                features.casing = features.tf_all_upper.max(features.tf_capitalized)
                    / (1.0 + features.tf.ln_1p());
                features.frequency = features.tf / (tf_mean + tf_std + f32::EPSILON);

                let occ_len = occurrences.len();
                let median = if occ_len == 0 {
                    0.0
                } else if occ_len == 1 {
                    occurrences[0].get_sentence_index() as f32
                } else if occ_len % 2 == 0 {
                    occurrences[occ_len / 2].get_sentence_index() as f32
                } else {
                    let mid = occ_len / 2;
                    (occurrences[mid].get_sentence_index()
                        + occurrences[mid - 1].get_sentence_index()) as f32
                        / 2.0
                };

                features.position = (3.0 + median).ln().ln();

                let left_right_context = context.get_word_context(word).unwrap_or((&[], &[]));
                let left_context_unique = left_right_context
                    .0
                    .iter()
                    .copied()
                    .collect::<HashSet<&str>>();

                if !left_context_unique.is_empty() {
                    features.wl = left_context_unique.len() as f32
                        / (left_right_context.0.len() as f32 + f32::EPSILON);
                }

                let right_context_unique = left_right_context
                    .1
                    .iter()
                    .copied()
                    .collect::<HashSet<&str>>();
                if !right_context_unique.is_empty() {
                    features.wr = right_context_unique.len() as f32
                        / (left_right_context.1.len() as f32 + f32::EPSILON);
                }

                features.relatedness = 1.0 + ((features.wl + features.wr) * (features.tf / tf_max));

                let unique_sentences = occurrences
                    .iter()
                    .map(|occ| occ.get_sentence_index())
                    .collect::<HashSet<usize>>();
                features.different = unique_sentences.len() as f32 / (sentence_len + f32::EPSILON);

                features.weight = (features.relatedness * features.position)
                    / (features.casing
                        + (features.frequency / features.relatedness)
                        + (features.different / features.relatedness));

                acc.insert(word, features);
                acc
            },
        ))
    }

    pub fn get_word_features(&self, word: &str) -> Option<&Features> {
        self.0.get(word)
    }
}
