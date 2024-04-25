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

use super::{
    candidate_selection::{Candidate, CandidateSelection},
    context_builder::ContextBuilder,
    feature_extraction::FeatureExtractor,
    sentences_builder::SentencesBuilder,
};

pub struct YakeLogic;

impl YakeLogic {
    pub fn build_yake(
        text: &str,
        stop_words: HashSet<&str>,
        punctuation: HashSet<&str>,
        ngram: usize,
        window_size: usize,
    ) -> HashMap<String, f32> {
        let sentences = SentencesBuilder::build_sentences(text);
        let sentences_len = sentences.len() as f32;
        let (candidates, dedup_hashmap) =
            CandidateSelection::select_candidates(&sentences, ngram, &stop_words, &punctuation);
        let (occurrences, lr_contexts) =
            ContextBuilder::build_context(&sentences, window_size, &punctuation, &stop_words);
        Self::score_candidates(
            FeatureExtractor::extract_features(occurrences, lr_contexts, sentences_len),
            candidates,
            dedup_hashmap,
        )
    }

    fn score_candidates<'a>(
        single_scores: HashMap<&'a str, f32>,
        candidates: HashMap<String, Candidate<'a>>,
        dedup_hashmap: HashMap<&'a str, f32>,
    ) -> HashMap<String, f32> {
        candidates
            .into_iter()
            .map(|(k, pc)| {
                let (prod, sum) = pc.lexical_form.iter().fold(
                    (*dedup_hashmap.get(k.as_str()).unwrap_or(&1.0), 0.0),
                    |acc, w| {
                        let weight = *single_scores.get(*w).unwrap_or(&0.0);

                        (acc.0 * weight, acc.1 + weight)
                    },
                );
                let tf = pc.surface_forms.len() as f32;
                let sum = if sum == -1.0 { 1.0 - f32::EPSILON } else { sum };
                let value = prod / (tf * (1.0 + sum));
                (k, 1.0 - value)
            })
            .collect()
    }
}
