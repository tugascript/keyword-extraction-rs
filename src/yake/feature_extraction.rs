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

use super::{
    context_builder::LeftRightContext, occurrences_builder::Occurrences,
    sentences_builder::Sentence,
};

pub struct FeatureExtraction<'a>(pub HashMap<&'a str, f32>);

impl<'a> FeatureExtraction<'a> {
    pub fn new(
        occurrences: Occurrences<'a>,
        contexts: LeftRightContext<'a>,
        sentences: &'a [Sentence<'a>],
    ) -> Self {
        let tf_values = occurrences
            .values()
            .map(|o| o.len())
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

        Self(
            occurrences
                .into_iter()
                .map(|(word, occurrences)| {
                    let tf = occurrences.len() as f32;

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

                    let mut tf_upper = 0.0_f32;
                    let mut tf_capitalized = 0.0_f32;
                    occurrences.iter().for_each(|occurrence| {
                        let w = occurrence.word;
                        tf_upper += if all_upper_check(w) { 1.0 } else { 0.0 };
                        tf_capitalized += if capitalized_check(w) { 1.0 } else { 0.0 };
                    });

                    let casing = tf_upper.max(tf_capitalized) / (1.0 + tf.ln());
                    let frequency = tf / (tf_mean + tf_std + f32::EPSILON);

                    let occ_len = occurrences.len();
                    let median = if occ_len == 0 {
                        0.0
                    } else if occ_len == 1 {
                        occurrences[0].sentence_index as f32
                    } else if occ_len % 2 == 0 {
                        occurrences[occ_len / 2].sentence_index as f32
                    } else {
                        let mid = occ_len / 2;
                        (occurrences[mid].sentence_index + occurrences[mid - 1].sentence_index)
                            as f32
                            / 2.0
                    };

                    let position = (3.0 + median).ln().ln();

                    let left_right_context = contexts
                        .get(word)
                        .map(|(v1, v2)| (v1.as_slice(), v2.as_slice()))
                        .unwrap_or((&[], &[]));
                    let left_context_unique = left_right_context
                        .0
                        .iter()
                        .copied()
                        .collect::<HashSet<&str>>();
                    let wl = if !left_context_unique.is_empty() {
                        left_context_unique.len() as f32
                            / (left_right_context.0.len() as f32 + f32::EPSILON)
                    } else {
                        0.0_f32
                    };

                    let right_context_unique = left_right_context
                        .1
                        .iter()
                        .copied()
                        .collect::<HashSet<&str>>();

                    let wr = if !right_context_unique.is_empty() {
                        right_context_unique.len() as f32
                            / (left_right_context.1.len() as f32 + f32::EPSILON)
                    } else {
                        0.0_f32
                    };

                    let relatedness = 1.0 + ((wl + wr) * (tf / tf_max));

                    let unique_sentences = occurrences
                        .iter()
                        .map(|occ| occ.sentence_index)
                        .collect::<HashSet<usize>>();
                    let different = unique_sentences.len() as f32 / (sentence_len + f32::EPSILON);

                    (
                        word,
                        (relatedness * position)
                            / (casing + (frequency / relatedness) + (different / relatedness)),
                    )
                })
                .collect(),
        )
    }
}
