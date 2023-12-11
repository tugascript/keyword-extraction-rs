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

mod text_rank_logic;
pub mod text_rank_params;
use text_rank_logic::TextRankLogic;
pub use text_rank_params::TextRankParams;

use crate::{
    common::{get_ranked_scores, get_ranked_strings},
    tokenizer::Tokenizer,
};

pub struct TextRank {
    word_rank: HashMap<String, f32>,
    phrase_rank: HashMap<String, f32>,
}

impl TextRank {
    /// Create a new TextRank instance.
    pub fn new(params: TextRankParams) -> Self {
        let (text, stop_words, punctuation, window_size, damping, tol, phrase_length) =
            params.get_params();
        let tokenizer = Tokenizer::new(text, stop_words, punctuation);
        let (word_rank, phrase_rank) = TextRankLogic::build_text_rank(
            tokenizer.sync_split_into_words(),
            tokenizer.sync_split_into_phrases(phrase_length),
            window_size,
            damping,
            tol,
        );

        Self {
            word_rank,
            phrase_rank,
        }
    }

    /// Gets the score of a word.
    pub fn get_word_score(&self, word: &str) -> f32 {
        *self.word_rank.get(word).unwrap_or(&0.0)
    }

    /// Gets the score of a phrase.
    pub fn get_phrase_score(&self, phrase: &str) -> f32 {
        *self.phrase_rank.get(phrase).unwrap_or(&0.0)
    }

    /// Gets the top n words with the highest score.
    pub fn get_ranked_words(&self, n: usize) -> Vec<String> {
        get_ranked_strings(&self.word_rank, n)
    }

    /// Get the top n words with the highest score and their score.
    pub fn get_ranked_word_scores(&self, n: usize) -> Vec<(String, f32)> {
        get_ranked_scores(&self.word_rank, n)
    }

    /// Gets the top n phrases with the highest score.
    pub fn get_ranked_phrases(&self, n: usize) -> Vec<String> {
        get_ranked_strings(&self.phrase_rank, n)
    }

    /// Get the top n phrases with the highest score and their score.
    pub fn get_ranked_phrase_scores(&self, n: usize) -> Vec<(String, f32)> {
        get_ranked_scores(&self.phrase_rank, n)
    }

    /// Gets the word scores map.
    pub fn get_word_scores_map(&self) -> &HashMap<String, f32> {
        &self.word_rank
    }

    /// Gets the phrase scores map.
    pub fn get_phrase_scores_map(&self) -> &HashMap<String, f32> {
        &self.phrase_rank
    }
}
