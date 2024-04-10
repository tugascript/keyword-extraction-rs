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

mod context_builder;
use context_builder::ContextBuilder;
mod feature_extraction;
mod levenshtein;
mod yake_logic;
use yake_logic::YakeLogic;
pub mod yake_params;
pub use yake_params::YakeParams;

use crate::common::{get_ranked_scores, get_ranked_strings, PUNCTUATION};

pub struct Yake(HashMap<String, f32>);

impl Yake {
    pub fn new(params: YakeParams) -> Self {
        let (text, stopwords, punctuation, threshold, ngrams, window_size) = params.get_params();
        Self(YakeLogic::build_yake(
            ContextBuilder::new(
                text,
                stopwords.iter().map(|s| s.as_str()).collect(),
                match punctuation {
                    None => PUNCTUATION.into_iter().collect(),
                    Some(p) => p.iter().map(|s| s.as_str()).collect(),
                },
                window_size,
                ngrams,
            ),
            threshold,
        ))
    }

    pub fn get_score(&self, keyword: &str) -> f32 {
        *self.0.get(keyword).unwrap_or(&0.0)
    }

    pub fn get_ranked_keywords(&self, n: usize) -> Vec<String> {
        get_ranked_strings(&self.0, n)
    }

    pub fn get_ranked_keyword_scores(&self, n: usize) -> Vec<(String, f32)> {
        get_ranked_scores(&self.0, n)
    }

    pub fn get_keyword_scores_map(&self) -> &HashMap<String, f32> {
        &self.0
    }
}
