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
use unicode_segmentation::{UnicodeSegmentation, UnicodeWords};

use crate::common::get_special_char_regex;

pub struct Sentence<'a> {
    words: Vec<&'a str>,
    stemmed: Vec<String>,
    length: usize,
}

impl<'a> Sentence<'a> {
    pub fn new(words: UnicodeWords<'a>, special_char_regex: &Option<Regex>) -> Self {
        let words = words.collect::<Vec<&str>>();
        Self {
            stemmed: words
                .iter()
                .map(|w| process_word(w, special_char_regex))
                .collect::<Vec<String>>(),
            length: words.len(),
            words,
        }
    }

    pub fn get_words(&self) -> &[&'a str] {
        &self.words
    }

    pub fn get_stemmed(&self) -> &[String] {
        &self.stemmed
    }

    pub fn get_length(&self) -> usize {
        self.length
    }
}

fn process_word(w: &str, special_char_regex: &Option<Regex>) -> String {
    if let Some(regex) = special_char_regex {
        regex.replace_all(w.trim(), "").to_lowercase()
    } else {
        w.trim().to_lowercase()
    }
}

pub struct YakeTokenizer<'a>(Vec<Sentence<'a>>);

impl<'a> YakeTokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        let special_char_regex = get_special_char_regex();
        Self(
            text.unicode_sentences()
                .map(|s| Sentence::new(s.trim().unicode_words(), &special_char_regex))
                .collect::<Vec<Sentence<'a>>>(),
        )
    }

    pub fn get_sentences(&self) -> &[Sentence<'a>] {
        &self.0
    }
}
