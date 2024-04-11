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

use std::collections::HashMap;

use super::{
    context_builder::ContextBuilder, feature_extraction::FeatureExtraction,
    levenshtein::Levenshtein,
};

pub struct YakeLogic;

impl YakeLogic {
    pub fn build_yake(context_builder: ContextBuilder<'_>, threshold: f32) -> HashMap<String, f32> {
        let words = context_builder.build_words();
        let sentences = context_builder.build_sentences();
        let right_left_context = context_builder.build_right_left_context();
        let pre_candidates = context_builder.build_pre_candidates(&words);
        let max_tf = context_builder.build_max_tf();

        Self::score_candidates(
            Self::filter_candidates(&pre_candidates, threshold),
            FeatureExtraction::new(&sentences, &words, right_left_context, max_tf),
        )
    }

    // Filter Pre Candidates into Candidates
    // Note: this reverses the order, but order is not important for the final calculation
    fn filter_candidates<'a>(
        pre_candidates: &'a [Vec<&'a str>],
        threshold: f32,
    ) -> Vec<Vec<&'a str>> {
        pre_candidates
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, candidate)| {
                let current = candidate.join(" ").to_lowercase();

                for pre_candidate in pre_candidates.iter().take(i) {
                    let other = pre_candidate.join(" ").to_lowercase();
                    let lev = Levenshtein::new(&current, &other);
                    if lev.ratio() >= threshold {
                        return None;
                    }
                }
                Some(candidate.to_vec())
            })
            .collect::<Vec<Vec<&'a str>>>()
    }

    /**
     * Formula
     * S(kw) = Π(H) / TF(kw)(1 + Σ(H))
     **/
    fn score_candidates(
        candidates: Vec<Vec<&str>>,
        feature_extraction: FeatureExtraction,
    ) -> HashMap<String, f32> {
        candidates
            .iter()
            .fold(HashMap::new(), |mut acc, candidate| {
                let (product, sum, tf) = candidate.iter().fold((1.0, 0.0, 0.0), |acc, word| {
                    let word = word.to_lowercase();
                    let value = feature_extraction
                        .get_feature_score(&word)
                        .unwrap_or((f32::EPSILON, f32::EPSILON));
                    (acc.0 * value.1, acc.1 + value.1, acc.2 + value.0)
                });
                let tf_kw = tf / candidate.len() as f32;
                let score = product / (tf_kw * (1.0 + sum));
                acc.insert(candidate.join(" ").to_lowercase(), score);
                acc
            })
    }
}
