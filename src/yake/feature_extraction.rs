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

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use super::candidate_selection_and_context_builder::{LeftRightContext, Occurrences};

fn extract_feature<'a, 'b>(
    contexts: &'a LeftRightContext<'a>,
    word: &'b str,
    occurrence: Vec<(&'b str, usize)>,
    tf_mean: f32,
    tf_std: f32,
    tf_max: f32,
    sentences_len: f32,
) -> (&'b str, f32) {
    let tf = occurrence.len() as f32;

    let (tf_upper, tf_capitalized) =
        occurrence
            .iter()
            .fold((0.0_f32, 0.0_f32), |(tf_upper, tf_capitalized), (w, _)| {
                (
                    tf_upper
                        + if w.graphemes(true).count() > 1 && &w.to_uppercase().as_str() == w {
                            1.0
                        } else {
                            0.0
                        },
                    tf_capitalized
                        + if w.chars().next().unwrap_or(' ').is_uppercase()
                            && (w.graphemes(true).count() == 1
                                || w.chars().skip(1).any(|c| c.is_lowercase()))
                        {
                            1.0
                        } else {
                            0.0
                        },
                )
            });

    let casing = tf_upper.max(tf_capitalized) / (1.0 + tf.ln());
    let frequency = tf / (tf_mean + tf_std + f32::EPSILON);

    let occ_len = occurrence.len();
    let median = if occ_len == 0 {
        0.0
    } else if occ_len == 1 {
        occurrence[0].1 as f32
    } else if occ_len % 2 == 0 {
        occurrence[occ_len / 2].1 as f32
    } else {
        let mid = occ_len / 2;
        (occurrence[mid].1 + occurrence[mid - 1].1) as f32 / 2.0
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
        left_context_unique.len() as f32 / (left_right_context.0.len() as f32 + f32::EPSILON)
    } else {
        0.0_f32
    };

    let right_context_unique = left_right_context
        .1
        .iter()
        .copied()
        .collect::<HashSet<&str>>();
    let wr = if !right_context_unique.is_empty() {
        right_context_unique.len() as f32 / (left_right_context.1.len() as f32 + f32::EPSILON)
    } else {
        0.0_f32
    };

    let relatedness = 1.0 + ((wl + wr) * (tf / tf_max));

    let unique_sentences = occurrence
        .iter()
        .map(|occ| occ.1)
        .collect::<HashSet<usize>>();
    let different = unique_sentences.len() as f32 / (sentences_len + f32::EPSILON);

    (
        word,
        (relatedness * position) / (casing + (frequency / relatedness) + (different / relatedness)),
    )
}

pub struct FeatureExtractor;

impl<'a> FeatureExtractor {
    pub fn score_words(
        occurrences: Occurrences<'a>,
        contexts: LeftRightContext<'a>,
        sentences_len: f32,
    ) -> HashMap<&'a str, f32> {
        let (tf_total, tf_max_u) = occurrences.values().fold((0, 0), |(tf_total, tf_max), v| {
            let length = v.len();
            (tf_total + length, tf_max.max(length))
        });
        let tf_max = tf_max_u as f32;
        let tf_mean = tf_total as f32 / (occurrences.len() as f32 + f32::EPSILON);
        let tf_std = occurrences
            .values()
            .fold(0.0_f32, |a, v| a + (v.len() as f32 - tf_mean).powi(2))
            .sqrt();

        #[cfg(feature = "parallel")]
        {
            occurrences
                .into_par_iter()
                .map(|(word, occurrence)| {
                    extract_feature(
                        &contexts,
                        word,
                        occurrence,
                        tf_mean,
                        tf_std,
                        tf_max,
                        sentences_len,
                    )
                })
                .collect()
        }

        #[cfg(not(feature = "parallel"))]
        {
            occurrences
                .into_iter()
                .map(|(word, occurrence)| {
                    extract_feature(
                        &contexts,
                        word,
                        occurrence,
                        tf_mean,
                        tf_std,
                        tf_max,
                        sentences_len,
                    )
                })
                .collect()
        }
    }
}
