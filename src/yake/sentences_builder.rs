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

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

use crate::common::{get_space_regex, get_special_char_regex};

fn process_text(text: &str) -> String {
    let space_regex = get_space_regex();
    let trimmed_text = text.trim();

    if let Some(regex) = space_regex {
        regex.replace_all(trimmed_text, " ").to_string()
    } else {
        trimmed_text.to_string()
    }
}

pub struct Sentence {
    pub words: Vec<String>,
    pub stemmed: Vec<String>,
    pub length: usize,
}

impl Sentence {
    pub fn new(s: &str, special_char_regex: &Option<Regex>) -> Self {
        let words = s
            .split_word_bounds()
            .filter_map(|w| {
                let trimmed = w.trim();

                if trimmed.is_empty() {
                    return None;
                }

                if let Some(regex) = special_char_regex {
                    let value = regex.replace_all(trimmed, "");

                    if value.is_empty() {
                        return None;
                    }

                    Some(value.to_string())
                } else {
                    Some(trimmed.to_string())
                }
            })
            .collect::<Vec<String>>();
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

impl SentencesBuilder {
    pub fn build_sentences(text: &str) -> Vec<Sentence> {
        let special_char_regex = get_special_char_regex();
        let pre_processed_text = process_text(text);
        pre_processed_text
            .unicode_sentences()
            .map(|s| Sentence::new(s.trim(), &special_char_regex))
            .collect()
    }
}
