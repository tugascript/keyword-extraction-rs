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

use std::collections::HashMap;

mod document_processor;
mod tf_idf_logic;
pub mod tf_idf_params;
use tf_idf_logic::TfIdfLogic;
pub use tf_idf_params::{TextSplit, TfIdfParams};

use crate::common::{get_ranked_scores, get_ranked_strings};

pub struct TfIdf(HashMap<String, f32>);

impl TfIdf {
    /// Creates a new TfIdf struct with the given parameters.
    pub fn new(params: TfIdfParams) -> Self {
        let documents = params.get_documents();
        Self(TfIdfLogic::build_tfidf(&documents))
    }

    /// Gets the score of a given word.
    pub fn get_score(&self, word: &str) -> f32 {
        *self.0.get(word).unwrap_or(&0.0)
    }

    /// Gets the top n words with the highest score.
    pub fn get_ranked_words(&self, n: usize) -> Vec<String> {
        get_ranked_strings(&self.0, n)
    }

    /// Gets the top n words with the highest score.
    pub fn get_ranked_word_scores(&self, n: usize) -> Vec<(String, f32)> {
        get_ranked_scores(&self.0, n)
    }

    /// Gets the word scores map.
    pub fn get_word_scores_map(&self) -> &HashMap<String, f32> {
        &self.0
    }
}
