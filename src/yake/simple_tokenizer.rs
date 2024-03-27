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

use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

use crate::common::{Text, PUNCTUATION};

pub struct SimpleTokenizer<'a> {
    text: &'a str,
    stopwords: HashSet<&'a str>,
    punctuation: HashSet<&'a str>,
}

fn process_word<'a>(
    word: &'a str,
    stopwords: &'a HashSet<&'a str>,
    punctuation: &'a HashSet<&'a str>,
) -> Option<&'a str> {
    if word.is_empty()
        || (word.graphemes(true).count() == 1 && punctuation.contains(word))
        || stopwords.contains(word)
    {
        return None;
    }
    Some(word)
}

impl<'a> SimpleTokenizer<'a> {
    /// Create a new Tokenizer instance.
    pub fn new(
        text: Text<'a>,
        stopwords: &'a [&'a str],
        punctuation: Option<&'a [&'a str]>,
    ) -> Self {
        Self {
            text,
            stopwords: stopwords.into_iter().copied().collect::<HashSet<&str>>(),
            punctuation: punctuation
                .unwrap_or(&PUNCTUATION)
                .into_iter()
                .copied()
                .collect::<HashSet<&str>>(),
        }
    }

    pub fn split_into_words(&'a self) -> Vec<&'a str> {
        self.text
            .unicode_words()
            .filter_map(|w| process_word(w, &self.stopwords, &self.punctuation))
            .collect()
    }

    pub fn split_into_sentences(&'a self) -> Vec<Vec<&'a str>> {
        self.text
            .unicode_sentences()
            .map(|s| {
                s.trim()
                    .unicode_words()
                    .filter_map(|w| process_word(w, &self.stopwords, &self.punctuation))
                    .collect()
            })
            .collect()
    }
}
