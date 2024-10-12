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

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use super::{
    candidate_selection_and_context_builder::{Candidate, CandidateSelectionAndContextBuilder},
    feature_extraction::FeatureExtractor,
    sentences_builder::SentencesBuilder,
    text_pre_processor::TextPreProcessor,
};

pub struct YakeLogic;

fn score_candidate<'a>(
    dedup_hashmap: &'a HashMap<&'a str, f32>,
    word_scores: &HashMap<&'a str, f32>,
    mut acc: Vec<(String, f32)>,
    max: f32,
    k: String,
    pc: Candidate<'a>,
) -> (Vec<(String, f32)>, f32) {
    let (prod, sum) = pc.lexical_form.iter().fold(
        (*dedup_hashmap.get(k.as_str()).unwrap_or(&1.0), 0.0),
        |acc, w| {
            let weight = *word_scores.get(*w).unwrap_or(&0.0);

            (acc.0 * weight, acc.1 + weight)
        },
    );
    let tf = pc.surface_forms.len() as f32;
    let sum = if sum == -1.0 { 1.0 - f32::EPSILON } else { sum };
    let score = prod / (tf * (1.0 + sum));
    let inverse_score = 1.0 / score;
    acc.push((k, inverse_score));
    (acc, max.max(inverse_score))
}

impl YakeLogic {
    pub fn build_yake(
        text: &str,
        stop_words: HashSet<&str>,
        punctuation: HashSet<&str>,
        ngram: usize,
        window_size: usize,
    ) -> (HashMap<String, f32>, HashMap<String, f32>) {
        let text = TextPreProcessor::process_text(text);
        let sentences = SentencesBuilder::build_sentences(&text);
        let (candidates, dedup_hashmap, occurrences, lr_contexts) =
            CandidateSelectionAndContextBuilder::select_candidates_and_build_context(
                &sentences,
                ngram,
                window_size,
                stop_words,
                punctuation,
            );
        let word_scores =
            FeatureExtractor::score_words(occurrences, lr_contexts, sentences.len() as f32);
        (
            Self::score_candidates(candidates, dedup_hashmap, &word_scores),
            Self::score_terms(word_scores),
        )
    }

    fn score_candidates<'a>(
        candidates: HashMap<String, Candidate<'a>>,
        dedup_hashmap: HashMap<&'a str, f32>,
        word_scores: &HashMap<&'a str, f32>,
    ) -> HashMap<String, f32> {
        let candidates_len = candidates.len();

        #[cfg(feature = "parallel")]
        {
            let (vec_scores, max) = candidates
                .into_par_iter()
                .fold(
                    || (Vec::<(String, f32)>::new(), f32::EPSILON),
                    |(acc, max), (k, pc)| {
                        score_candidate(&dedup_hashmap, word_scores, acc, max, k, pc)
                    },
                )
                .reduce(
                    || (Vec::with_capacity(candidates_len), f32::EPSILON),
                    |(mut acc1, max1), (mut acc2, max2)| {
                        acc1.append(&mut acc2);
                        (acc1, max1.max(max2))
                    },
                );

            vec_scores
                .into_par_iter()
                .map(|(k, v)| (k, v / max))
                .collect::<HashMap<String, f32>>()
        }

        #[cfg(not(feature = "parallel"))]
        {
            let (vec_scores, max) = candidates.into_iter().fold(
                (
                    Vec::<(String, f32)>::with_capacity(candidates_len),
                    f32::EPSILON,
                ),
                |(acc, max), (k, pc)| score_candidate(&dedup_hashmap, word_scores, acc, max, k, pc),
            );

            vec_scores
                .into_iter()
                .map(|(k, v)| (k, v / max))
                .collect::<HashMap<String, f32>>()
        }
    }

    fn score_terms(word_scores: HashMap<&str, f32>) -> HashMap<String, f32> {
        let word_scores_len = word_scores.len();

        #[cfg(feature = "parallel")]
        {
            let (vec_scores, max) = word_scores
                .into_par_iter()
                .fold(
                    || (Vec::<(String, f32)>::new(), f32::EPSILON),
                    |(mut acc, max), (k, score)| {
                        let inverse_score = 1.0 / score;
                        acc.push((k.to_string(), inverse_score));
                        (acc, max.max(inverse_score))
                    },
                )
                .reduce(
                    || (Vec::with_capacity(word_scores_len), f32::EPSILON),
                    |(mut acc1, max1), (mut acc2, max2)| {
                        acc1.append(&mut acc2);
                        (acc1, max1.max(max2))
                    },
                );

            vec_scores
                .into_par_iter()
                .map(|(k, v)| (k, v / max))
                .collect::<HashMap<String, f32>>()
        }

        #[cfg(not(feature = "parallel"))]
        {
            let (vec_scores, max) = word_scores.into_iter().fold(
                (
                    Vec::<(String, f32)>::with_capacity(word_scores_len),
                    f32::EPSILON,
                ),
                |(mut acc, max), (k, score)| {
                    let inverse_score = 1.0 / score;
                    acc.push((k.to_string(), inverse_score));
                    (acc, max.max(inverse_score))
                },
            );

            vec_scores
                .into_iter()
                .map(|(k, v)| (k, v / max))
                .collect::<HashMap<String, f32>>()
        }
    }
}
