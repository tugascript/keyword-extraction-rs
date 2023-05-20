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

use super::tokenizer::Tokenizer;
use std::{cmp::Ordering, collections::HashMap};

mod calculate_weights;
mod candidate_selection;
mod levenshtein;
pub mod yake_params;

use calculate_weights::calculate_weights;
use candidate_selection::CandidateSelection;
pub use yake_params::{WeightParams, YakeParams};

pub struct SimplifedYake(HashMap<String, f32>);

impl SimplifedYake {
    pub fn new(params: YakeParams) -> Self {
        let (text, stopwords, ngrams, window_size, threshold, weights) = params.get_values();
        let sentences = Tokenizer::new(text, stopwords, None).split_into_sentences();
        let weighted_candidates = calculate_weights(
            &sentences,
            CandidateSelection::new(&sentences, ngrams, threshold).get_candidates(),
            window_size,
        );
        Self(
            weighted_candidates
                .iter()
                .map(|weighted_candidate| {
                    (
                        weighted_candidate.term(),
                        weighted_candidate
                            .calculate_score(weights.0, weights.1, weights.2, weights.3, weights.4),
                    )
                })
                .collect::<HashMap<String, f32>>(),
        )
    }

    pub fn get_score(&self, keyword: &str) -> f32 {
        *self.0.get(keyword).unwrap_or(&0.0)
    }

    pub fn get_ranked_words(&self, n: usize) -> Vec<String> {
        let mut sorted_yake = self.0.iter().collect::<Vec<(&String, &f32)>>();
        sorted_yake.sort_by(|a, b| {
            let order = a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal);

            if order == Ordering::Equal {
                return b.0.cmp(a.0);
            }

            order
        });
        sorted_yake
            .iter()
            .take(n)
            .map(|(word, _)| word.to_string())
            .collect::<Vec<String>>()
    }
}
