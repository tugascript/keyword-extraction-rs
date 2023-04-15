// Copyright (C) 2023 Afonso Barracha
//
// This file is part of keyword-extraction.
//
// keyword-extraction is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// keyword-extraction is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with keyword-extraction.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashSet;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

pub struct Tokenizer {
    text: String,
    stopwords: HashSet<String>,
    punctuation: HashSet<String>,
}

fn get_special_char_regex() -> Regex {
    Regex::new(r"('s|,|\.)").unwrap()
}

fn get_sentence_space_regex() -> Regex {
    Regex::new(r"[\n\t\r]").unwrap()
}

fn get_newline_regex() -> Regex {
    Regex::new(r"(\r|\n|\r\n)").unwrap()
}

fn process_word(
    w: &str,
    special_char_regex: &Regex,
    stopwords: &HashSet<String>,
    punctuation: &HashSet<String>,
) -> Option<String> {
    let word = special_char_regex.replace_all(w.trim(), "").to_lowercase();

    if word.is_empty() {
        return None;
    }
    if (word.graphemes(true).count() == 1) && punctuation.contains(&word) {
        return None;
    }
    if stopwords.contains(&word) {
        return None;
    }

    Some(word)
}

impl Tokenizer {
    pub fn new(text: &str, stopwords: Vec<String>, punctuation: Option<Vec<String>>) -> Tokenizer {
        Tokenizer {
            text: text.to_owned(),
            stopwords: HashSet::from_iter(stopwords.iter().map(|s| s.to_owned())),
            punctuation: HashSet::from_iter(
                punctuation
                    .unwrap_or(
                        vec![
                            "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", ";", ".", "/",
                            ":", ",", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|",
                            "}", "~", "-",
                        ]
                            .iter()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                    )
                    .iter()
                    .map(|s| s.to_string()),
            ),
        }
    }

    pub fn split_into_words(&self) -> Vec<String> {
        self.text
            .split_word_bounds()
            .filter_map(|w| {
                process_word(
                    w,
                    &get_special_char_regex(),
                    &self.stopwords,
                    &self.punctuation,
                )
            })
            .collect::<Vec<String>>()
    }

    pub fn split_into_sentences(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();
        get_sentence_space_regex()
            .replace_all(&self.text, " ")
            .unicode_sentences()
            .map(|s| {
                s.split_word_bounds()
                    .filter_map(|w| {
                        process_word(w, &special_char_regex, &self.stopwords, &self.punctuation)
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
    }

    pub fn split_into_paragraphs(&self) -> Vec<String> {
        get_newline_regex()
            .split(&self.text)
            .map(|s| {
                s.unicode_sentences()
                    .map(|s| {
                        s.split_word_bounds()
                            .filter_map(|w| {
                                process_word(
                                    w,
                                    &get_special_char_regex(),
                                    &self.stopwords,
                                    &self.punctuation,
                                )
                            })
                            .collect::<Vec<String>>()
                            .join(" ")
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
    }
}
