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
    candidate_selection::Candidates,
    context_builder::Context,
    feature_extraction::{FeatureExtraction, Features},
    levenshtein::Levenshtein,
    yake_tokenizer::YakeTokenizer,
};

pub struct YakeLogic;

impl YakeLogic {
    pub fn build_yake(
        text: &str,
        stop_words: HashSet<&str>,
        punctuation: HashSet<&str>,
        threshold: f32,
        ngram: usize,
        window_size: usize,
    ) -> HashMap<String, f32> {
        let tokenizer = YakeTokenizer::new(text);
        let sentences = tokenizer.get_sentences();
        let context = Context::new(sentences, &punctuation, window_size);
        let feature_extraction = FeatureExtraction::new(&context, sentences, &stop_words);
        let candidates = Candidates::new(sentences, ngram, &stop_words);
        Self::filter_candidates(
            &candidates,
            Self::score_candidates(
                &feature_extraction,
                &candidates,
                Self::build_de_duplicate_hashset(&candidates),
            ),
            threshold,
        )
    }

    // Filter Pre Candidates into Candidates
    // Note: this reverses the order, but order is not important for the final calculation
    fn filter_candidates(
        candidates: &Candidates,
        score: HashMap<&str, f32>,
        threshold: f32,
    ) -> HashMap<String, f32> {
        candidates
            .candidates()
            .enumerate()
            .filter_map(|(i, (k1, _))| {
                for (k2, _) in candidates.candidates().skip(i + 1) {
                    let lev = Levenshtein::new(k1, k2);
                    if lev.ratio() >= threshold {
                        return None;
                    }
                }
                Some((k1.to_string(), *score.get(k1.as_str()).unwrap_or(&0.0)))
            })
            .collect()
    }

    fn build_de_duplicate_hashset<'a>(candidates: &'a Candidates) -> HashSet<&'a str> {
        candidates
            .candidates()
            .fold(HashSet::new(), |mut acc, (_, (s, _))| {
                if s.len() > 1 {
                    s.iter().for_each(|w| {
                        acc.insert(*w);
                    })
                }

                acc
            })
    }

    /**
     * Formula
     * S(kw) = Π(H) / TF(kw)(1 + Σ(H))
     **/
    fn score_candidates<'a>(
        feature_extraction: &'a FeatureExtraction,
        candidates: &'a Candidates,
        dedup_hashset: HashSet<&'a str>,
    ) -> HashMap<&'a str, f32> {
        candidates
            .candidates()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                let (prod, sum) = v.0.iter().fold((1.0, 0.0), |acc, w| {
                    let weight = feature_extraction
                        .get_word_features(w)
                        .unwrap_or(&Features::default())
                        .get_weight();
                    (acc.0 * weight, acc.1 + weight)
                });
                let tf = v.1.len() as f32;
                let key = k.as_str();
                let prod = if dedup_hashset.contains(key) {
                    prod + 5.0
                } else {
                    prod
                };
                let sum = if sum == -1.0 { 1.0 - f32::EPSILON } else { sum };

                acc.insert(key, prod / (tf * (1.0 + sum)));
                acc
            })
    }
}
