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

use std::{cmp::Ordering, collections::HashMap, hash::RandomState};

mod candidate_selection;
mod context_builder;
mod feature_extraction;
mod levenshtein;
mod yake_logic;
pub mod yake_params;
mod yake_tokenizer;
pub use yake_params::YakeParams;

use crate::common::PUNCTUATION;

use self::yake_logic::YakeLogic;

fn basic_sort<'a>(map: &'a HashMap<String, f32, RandomState>) -> Vec<(&'a String, &'a f32)> {
    let mut map_values = map.iter().collect::<Vec<(&'a String, &'a f32)>>();
    map_values.sort_by(|a, b| {
        let order = a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal);

        if order == Ordering::Equal {
            return b.0.cmp(a.0);
        }

        order
    });
    map_values
}

pub struct Yake(HashMap<String, f32>);

impl Yake {
    pub fn new(params: YakeParams) -> Self {
        let (text, stop_words, puctuation, threshold, ngram, window_size) = params.get_params();
        Self(YakeLogic::build_yake(
            text,
            stop_words.iter().map(|s| s.as_str()).collect(),
            match puctuation {
                Some(p) => p.iter().map(|s| s.as_str()).collect(),
                None => PUNCTUATION.iter().copied().collect(),
            },
            threshold,
            ngram,
            window_size,
        ))
    }

    pub fn get_score(&self, keyword: &str) -> f32 {
        *self.0.get(keyword).unwrap_or(&0.0)
    }

    pub fn get_ranked_keywords(&self, n: usize) -> Vec<String> {
        basic_sort(&self.0)
            .iter()
            .take(n)
            .map(|(k, _)| k.to_string())
            .collect()
    }

    pub fn get_ranked_keyword_scores(&self, n: usize) -> Vec<(String, f32)> {
        basic_sort(&self.0)
            .iter()
            .take(n)
            .map(|(k, v)| (k.to_string(), **v))
            .collect()
    }

    pub fn get_keyword_scores_map(&self) -> &HashMap<String, f32> {
        &self.0
    }
}
