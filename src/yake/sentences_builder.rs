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

use std::borrow::Cow;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::common::get_special_char_regex;

pub struct Sentence<'a> {
    pub words: Vec<Cow<'a, str>>,
    pub stemmed: Vec<String>,
    pub length: usize,
}

impl<'a> Sentence<'a> {
    pub fn new(s: &'a str, special_char_regex: &Option<Regex>) -> Self {
        let words = s
            .split_word_bounds()
            .filter_map(|w| {
                let trimmed = w.trim();

                if trimmed.is_empty() || trimmed == " " {
                    return None;
                }

                if let Some(regex) = special_char_regex {
                    let value = regex.replace_all(trimmed, "");

                    if value.is_empty() || value == " " {
                        return None;
                    }

                    Some(value)
                } else {
                    Some(trimmed.into())
                }
            })
            .collect::<Vec<Cow<'a, str>>>();
        Self {
            stemmed: words
                .iter()
                .map(|w| w.to_lowercase())
                .collect::<Vec<String>>(),
            length: words.len(),
            words,
        }
    }
}

pub struct SentencesBuilder;

impl<'a> SentencesBuilder {
    pub fn build_sentences(pre_processed_text: &'a str) -> Vec<Sentence<'a>> {
        let special_char_regex = get_special_char_regex();
        pre_processed_text
            .unicode_sentences()
            .map(|s| Sentence::new(s.trim(), &special_char_regex))
            .collect()
    }
}
