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

mod rake_logic;
use rake_logic::RakeLogic;

pub struct Rake {
    word_scores: HashMap<String, f32>,
    phrase_scores: HashMap<String, f32>,
}

impl Rake {
    /// Create a new Rake instance.
    pub fn new(text: &str, stopwords: &[String]) -> Self {
        let (word_scores, phrase_scores) = RakeLogic::build_rake(text, stopwords);

        Self {
            phrase_scores,
            word_scores,
        }
    }

    /// Gets the top n words with the highest score.
    pub fn get_ranked_keyword(&self, n: usize) -> Vec<String> {
        let mut keywords = self.word_scores.iter().collect::<Vec<(&String, &f32)>>();
        keywords.sort_by(|a, b| {
            let order = b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal);

            if order == Ordering::Equal {
                return a.0.cmp(b.0);
            }

            order
        });
        keywords
            .iter()
            .take(n)
            .map(|(word, _)| word.to_string())
            .collect::<Vec<String>>()
    }

    /// Gets the top n phrases with the highest score.
    pub fn get_ranked_phrases(&self, n: usize) -> Vec<String> {
        let mut phrases = self.phrase_scores.iter().collect::<Vec<(&String, &f32)>>();
        phrases.sort_by(|a, b| {
            let order = b.1.partial_cmp(a.1).unwrap_or(Ordering::Equal);

            if order == Ordering::Equal {
                return a.0.len().cmp(&b.0.len());
            }

            order
        });
        phrases
            .iter()
            .take(n)
            .map(|(phrase, _)| phrase.to_string())
            .collect::<Vec<String>>()
    }

    pub fn get_keyword_score(&self, word: &str) -> f32 {
        *self.word_scores.get(word).unwrap_or(&0.0)
    }

    pub fn get_phrase_score(&self, phrase: &str) -> f32 {
        *self.phrase_scores.get(phrase).unwrap_or(&0.0)
    }
}
