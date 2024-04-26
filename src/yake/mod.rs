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

use std::{cmp::min, collections::HashMap};

mod candidate_selection_and_context_builder;
mod feature_extraction;
mod levenshtein;
mod sentences_builder;
mod text_pre_processor;
mod yake_logic;
pub mod yake_params;
pub use yake_params::YakeParams;

use crate::common::{get_ranked_scores, get_ranked_strings, sort_ranked_map, PUNCTUATION};

use levenshtein::Levenshtein;
use yake_logic::YakeLogic;

fn build_ranked_keywords(vec: &mut Vec<String>, word: &str, threshold: f32) {
    if vec
        .iter()
        .any(|w| Levenshtein::new(w, word).ratio() >= threshold)
    {
        return;
    }
    vec.push(word.to_string());
}

fn build_ranked_scores(vec: &mut Vec<(String, f32)>, word: &str, score: f32, threshold: f32) {
    if vec
        .iter()
        .any(|(w, _)| Levenshtein::new(w, word).ratio() >= threshold)
    {
        return;
    }
    vec.push((word.to_string(), score));
}

pub struct Yake {
    keyword_rank: HashMap<String, f32>,
    term_rank: HashMap<String, f32>,
    size: usize,
    threshold: f32,
}

impl Yake {
    /// Create a new YAKE instance.
    pub fn new(params: YakeParams) -> Self {
        let (text, stop_words, puctuation, threshold, ngram, window_size) = params.get_params();
        let (keyword_rank, term_rank) = YakeLogic::build_yake(
            text,
            stop_words.iter().map(|s| s.as_str()).collect(),
            match puctuation {
                Some(p) => p.iter().map(|s| s.as_str()).collect(),
                None => PUNCTUATION.iter().copied().collect(),
            },
            ngram,
            window_size,
        );
        Self {
            size: keyword_rank.len(),
            keyword_rank,
            term_rank,
            threshold,
        }
    }

    /// Gets the score of a (n-gram terms) keyword.
    pub fn get_keyword_score(&self, keyword: &str) -> f32 {
        *self.keyword_rank.get(keyword).unwrap_or(&0.0)
    }

    /// Gets the score of a single term.
    pub fn get_word_score(&self, word: &str) -> f32 {
        *self.term_rank.get(word).unwrap_or(&0.0)
    }

    /// Get the top n (n-gram terms) keywords with the highest score.
    pub fn get_ranked_keywords(&self, n: usize) -> Vec<String> {
        let capacity = min(self.size, n);
        let result = sort_ranked_map(&self.keyword_rank).into_iter().try_fold(
            Vec::<String>::with_capacity(capacity),
            |mut acc, (word, _)| {
                if acc.len() == capacity {
                    return Err(acc);
                }
                build_ranked_keywords(&mut acc, word, self.threshold);
                Ok(acc)
            },
        );

        match result {
            Ok(v) => v,
            Err(v) => v,
        }
    }

    /// Gets the top n (n-gram terms) keywords with the highest score and their scores.
    pub fn get_ranked_keyword_scores(&self, n: usize) -> Vec<(String, f32)> {
        let capacity = min(self.size, n);
        let result = sort_ranked_map(&self.keyword_rank).into_iter().try_fold(
            Vec::<(String, f32)>::with_capacity(capacity),
            |mut acc, (word, score)| {
                if acc.len() == capacity {
                    return Err(acc);
                }
                build_ranked_scores(&mut acc, word, *score, self.threshold);
                Ok(acc)
            },
        );

        match result {
            Ok(v) => v,
            Err(v) => v,
        }
    }

    /// Gets the top n terms with the highest score.
    pub fn get_ranked_terms(&self, n: usize) -> Vec<String> {
        get_ranked_strings(&self.term_rank, n)
    }

    /// Gets the top n terms with the highest score and their scores.
    pub fn get_ranked_term_scores(&self, n: usize) -> Vec<(String, f32)> {
        get_ranked_scores(&self.term_rank, n)
    }

    /// Gets the (n-gram terms) keyword scores map
    pub fn get_keyword_scores_map(&self) -> &HashMap<String, f32> {
        &self.keyword_rank
    }

    /// Gets the term scores map
    pub fn get_term_scores_map(&self) -> &HashMap<String, f32> {
        &self.term_rank
    }
}
