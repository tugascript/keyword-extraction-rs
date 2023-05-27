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

mod rake_logic;
use rake_logic::RakeLogic;

use crate::common::{get_ranked_scores, get_ranked_strings, Stopwords, Text};

pub struct Rake {
    word_scores: HashMap<String, f32>,
    phrase_scores: HashMap<String, f32>,
}

impl Rake {
    /// Create a new Rake instance.
    pub fn new(text: Text, stopwords: Stopwords) -> Self {
        let (word_scores, phrase_scores) = RakeLogic::build_rake(text, stopwords);

        Self {
            phrase_scores,
            word_scores,
        }
    }

    /// Gets the top n words with the highest score.
    pub fn get_ranked_keyword(&self, n: usize) -> Vec<String> {
        get_ranked_strings(&self.word_scores, n)
    }

    /// Gets the top n words with the highest score and their score.
    pub fn get_ranked_keyword_scores(&self, n: usize) -> Vec<(String, f32)> {
        get_ranked_scores(&self.word_scores, n)
    }

    /// Gets the top n phrases with the highest score.
    pub fn get_ranked_phrases(&self, n: usize) -> Vec<String> {
        get_ranked_strings(&self.phrase_scores, n)
    }

    /// Gets the top n phrases with the highest score and their score.
    pub fn get_ranked_phares_scores(&self, n: usize) -> Vec<(String, f32)> {
        get_ranked_scores(&self.phrase_scores, n)
    }

    /// Gets the score of a word.
    pub fn get_keyword_score(&self, word: &str) -> f32 {
        *self.word_scores.get(word).unwrap_or(&0.0)
    }

    /// Gets the score of a phrase.
    pub fn get_phrase_score(&self, phrase: &str) -> f32 {
        *self.phrase_scores.get(phrase).unwrap_or(&0.0)
    }

    /// Gets the base hashmap of words and their score.
    pub fn get_word_scores_map(&self) -> &HashMap<String, f32> {
        &self.word_scores
    }

    /// Gets the base hashmap of phrases and their score.
    pub fn get_phrase_scores_map(&self) -> &HashMap<String, f32> {
        &self.phrase_scores
    }
}
