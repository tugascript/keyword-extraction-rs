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

use std::{cmp::Ordering, collections::HashMap};

mod text_rank_logic;
pub mod text_rank_params;
use text_rank_logic::TextRankLogic;
pub use text_rank_params::TextRankParams;

use crate::tokenizer::Tokenizer;

pub struct TextRank {
    word_rank: HashMap<String, f32>,
    phrase_rank: HashMap<String, f32>,
}

impl TextRank {
    pub fn new(params: TextRankParams) -> Self {
        let (text, stop_words, window_size, damping, tol) = params.get_params();
        let tokenizer = Tokenizer::new(text, stop_words, None);
        let (word_rank, phrase_rank) = TextRankLogic::build_text_rank(
            tokenizer.split_into_words(),
            tokenizer.split_into_phrases(),
            window_size,
            damping,
            tol,
        );

        Self {
            word_rank,
            phrase_rank,
        }
    }

    pub fn get_word_score(&self, word: &str) -> f32 {
        *self.word_rank.get(word).unwrap_or(&0.0)
    }

    pub fn get_phrase_score(&self, phrase: &str) -> f32 {
        *self.phrase_rank.get(phrase).unwrap_or(&0.0)
    }

    pub fn get_ranked_words(&self, n: usize) -> Vec<String> {
        let mut sorted_words = self.word_rank.iter().collect::<Vec<(&String, &f32)>>();
        sorted_words.sort_by(|a, b| {
            let order = b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal);

            if order == Ordering::Equal {
                return a.0.cmp(b.0);
            }

            order
        });
        sorted_words
            .iter()
            .take(n)
            .map(|(word, _)| word.to_string())
            .collect::<Vec<String>>()
    }

    pub fn get_ranked_phrases(&self, n: usize) -> Vec<String> {
        let mut sorted_phrases = self.phrase_rank.iter().collect::<Vec<(&String, &f32)>>();
        sorted_phrases.sort_by(|a, b| {
            let order = b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal);

            if order == Ordering::Equal {
                return a.0.cmp(b.0);
            }

            order
        });
        sorted_phrases
            .iter()
            .take(n)
            .map(|(phrase, _)| phrase.to_string())
            .collect::<Vec<String>>()
    }
}
