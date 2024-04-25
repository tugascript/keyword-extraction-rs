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

use std::{
    cmp::{min, Ordering},
    collections::HashMap,
    hash::RandomState,
};

mod candidate_selection;
mod context_builder;
mod feature_extraction;
mod levenshtein;
mod sentences_builder;
mod yake_logic;
pub mod yake_params;
pub use yake_params::YakeParams;

use crate::common::PUNCTUATION;

use levenshtein::Levenshtein;
use yake_logic::YakeLogic;

fn basic_sort<'a>(map: &'a HashMap<String, f32, RandomState>) -> Vec<(String, f32)> {
    let mut map_values = map
        .iter()
        .map(|(k, v)| (k.to_owned(), *v))
        .collect::<Vec<(String, f32)>>();
    map_values.sort_by(|a, b| {
        let order = b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal);

        if order == Ordering::Equal {
            return a.0.cmp(&b.0);
        }

        order
    });
    map_values
}

fn build_ranked_keywords(vec: &mut Vec<String>, word: &str, threshold: f32) -> () {
    if vec
        .iter()
        .any(|w| Levenshtein::new(w, word).ratio() >= threshold)
    {
        return;
    }
    vec.push(word.to_string());
}

fn build_ranked_scores(vec: &mut Vec<(String, f32)>, word: &str, score: f32, threshold: f32) -> () {
    if vec
        .iter()
        .any(|(w, s)| Levenshtein::new(w, word).ratio() >= threshold && *s >= score)
    {
        return;
    }
    vec.push((word.to_string(), score));
}

pub struct Yake {
    scores: HashMap<String, f32>,
    sorted_scores: Vec<(String, f32)>,
    size: usize,
    threshold: f32,
}

impl Yake {
    pub fn new(params: YakeParams) -> Self {
        let (text, stop_words, puctuation, threshold, ngram, window_size) = params.get_params();
        let scores = YakeLogic::build_yake(
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
            size: scores.len(),
            sorted_scores: basic_sort(&scores),
            scores,
            threshold,
        }
    }

    pub fn get_score(&self, keyword: &str) -> f32 {
        *self.scores.get(keyword).unwrap_or(&0.0)
    }

    pub fn get_ranked_keywords(&self, n: usize) -> Vec<String> {
        let capacity = min(self.size, n);
        let mut vec = Vec::with_capacity(capacity);

        for (word, _) in &self.sorted_scores {
            if vec.len() == n {
                break;
            }

            build_ranked_keywords(&mut vec, word, self.threshold);
        }

        vec
    }

    pub fn get_ranked_keyword_scores(&self, n: usize) -> Vec<(String, f32)> {
        let capacity = min(self.size, n);
        let mut vec = Vec::with_capacity(capacity);

        for (word, score) in &self.sorted_scores {
            if vec.len() == capacity {
                break;
            }

            build_ranked_scores(&mut vec, word, *score, self.threshold);
        }

        vec
    }

    pub fn get_keyword_scores_map(&self) -> &HashMap<String, f32> {
        &self.scores
    }
}
