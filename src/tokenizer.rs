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

use std::collections::HashSet;

use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::common::{
    get_special_char_regex, is_punctuation, process_word, PhraseLength, Punctuation, Stopwords,
    Text, PUNCTUATION,
};

pub struct Tokenizer {
    text: String,
    stopwords: HashSet<String>,
    punctuation: HashSet<String>,
}

#[cfg(feature = "parallel")]
fn get_sentence_space_regex() -> Regex {
    Regex::new(r"^([\.!?])[\n\t\r]").unwrap()
}

fn create_phrase(
    mut phrases: Vec<String>,
    mut phrase: String,
    base_word: &str,
    special_char_regex: &Regex,
    punctuation: &HashSet<String>,
    stopwords: &HashSet<String>,
    length: Option<usize>,
) -> (Vec<String>, String) {
    let word = special_char_regex
        .replace_all(base_word.trim(), "")
        .to_lowercase();

    if !is_punctuation(&word, punctuation) {
        if stopwords.contains(&word) {
            if !phrase.is_empty() {
                phrases.push(phrase);
                phrase = String::new();
            }
        } else {
            if !phrase.is_empty() {
                phrase.push(' ');
            }

            phrase.push_str(&word);
        }
    }
    if let Some(length) = length {
        if phrase.split_whitespace().count() >= length {
            phrases.push(phrase);
            phrase = String::new();
        }
    }

    (phrases, phrase)
}

fn process_sentences(
    sentence: &str,
    special_char_regex: &Regex,
    punctuation: &HashSet<String>,
    stopwords: &HashSet<String>,
) -> String {
    sentence
        .split_word_bounds()
        .filter_map(|w| process_word(w, special_char_regex, stopwords, punctuation))
        .collect::<Vec<String>>()
        .join(" ")
}

fn process_paragraphs(
    paragraph: &str,
    special_char_regex: &Regex,
    punctuation: &HashSet<String>,
    stopwords: &HashSet<String>,
) -> Option<String> {
    if paragraph.trim().is_empty() {
        return None;
    }

    Some(
        paragraph
            .unicode_sentences()
            .map(|s| {
                s.split_word_bounds()
                    .filter_map(|w| process_word(w, special_char_regex, stopwords, punctuation))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join(" "),
    )
}

impl Tokenizer {
    /// Create a new Tokenizer instance.
    pub fn new(text: Text, stopwords: Stopwords, punctuation: Punctuation) -> Self {
        Self {
            text: text.to_owned(),
            stopwords: stopwords
                .iter()
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>(),
            punctuation: punctuation
                .unwrap_or(
                    &PUNCTUATION
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                )
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        }
    }

    /// Split text into words by splitting on word bounds.
    pub fn split_into_words(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();

        #[cfg(feature = "parallel")]
        {
            self.text
                .split_word_bounds()
                .par_bridge()
                .filter_map(|w| {
                    process_word(w, &special_char_regex, &self.stopwords, &self.punctuation)
                })
                .collect::<Vec<String>>()
        }

        #[cfg(not(feature = "parallel"))]
        {
            self.text
                .split_word_bounds()
                .filter_map(|w| {
                    process_word(w, &special_char_regex, &self.stopwords, &self.punctuation)
                })
                .collect::<Vec<String>>()
        }
    }

    /// Split text into words by splitting on word bounds (always synchronous even with parallel flag).
    pub fn sync_split_into_words(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();
        self.text
            .split_word_bounds()
            .filter_map(|w| {
                process_word(w, &special_char_regex, &self.stopwords, &self.punctuation)
            })
            .collect::<Vec<String>>()
    }

    /// Split text into unicode sentences by splitting on punctuation.
    pub fn split_into_sentences(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();

        #[cfg(feature = "parallel")]
        {
            self.text
                .unicode_sentences()
                .par_bridge()
                .map(|s| {
                    process_sentences(s, &special_char_regex, &self.punctuation, &self.stopwords)
                })
                .collect::<Vec<String>>()
        }

        #[cfg(not(feature = "parallel"))]
        {
            self.text
                .unicode_sentences()
                .map(|s| {
                    process_sentences(s, &special_char_regex, &self.punctuation, &self.stopwords)
                })
                .collect::<Vec<String>>()
        }
    }

    /// Split text into unicode sentences (always synchronous even with parallel flag).
    pub fn sync_split_into_sentences(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();
        self.text
            .unicode_sentences()
            .map(|s| process_sentences(s, &special_char_regex, &self.punctuation, &self.stopwords))
            .collect::<Vec<String>>()
    }

    /// Split text into phrases by splitting on stopwords.
    pub fn split_into_phrases(&self, length: PhraseLength) -> Vec<String> {
        let special_char_regex = get_special_char_regex();

        #[cfg(feature = "parallel")]
        {
            self.parallel_phrase_split(&special_char_regex, length)
        }

        #[cfg(not(feature = "parallel"))]
        {
            self.basic_phrase_split(&special_char_regex, length)
        }
    }

    /// Split text into words by splitting on word bounds (always synchronous even with parallel flag).
    pub fn sync_split_into_phrases(&self, length: Option<usize>) -> Vec<String> {
        let special_char_regex = get_special_char_regex();

        self.basic_phrase_split(&special_char_regex, length)
    }

    fn basic_phrase_split(&self, special_char_regex: &Regex, length: Option<usize>) -> Vec<String> {
        let (mut phrases, last_phrase) = self.text.split_word_bounds().fold(
            (Vec::<String>::new(), String::new()),
            |(phrases, acc), w| {
                create_phrase(
                    phrases,
                    acc,
                    w,
                    special_char_regex,
                    &self.punctuation,
                    &self.stopwords,
                    length,
                )
            },
        );

        if !last_phrase.is_empty() {
            phrases.push(last_phrase);
        }

        phrases
    }

    #[cfg(feature = "parallel")]
    fn parallel_phrase_split(
        &self,
        special_char_regex: &Regex,
        length: Option<usize>,
    ) -> Vec<String> {
        get_sentence_space_regex()
            .replace_all(&self.text, "¶")
            .par_split('¶')
            .map(|s| {
                let (mut phrases, last_phrase) = s.split_word_bounds().fold(
                    (Vec::<String>::new(), String::new()),
                    |(phrases, acc), w| {
                        create_phrase(
                            phrases,
                            acc,
                            w,
                            special_char_regex,
                            &self.punctuation,
                            &self.stopwords,
                            length,
                        )
                    },
                );

                if !last_phrase.is_empty() {
                    phrases.push(last_phrase);
                }

                phrases
            })
            .flatten()
            .collect::<Vec<String>>()
    }

    /// Split text into paragraphs by splitting on newlines.
    pub fn split_into_paragraphs(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();

        #[cfg(feature = "parallel")]
        {
            self.text
                .par_lines()
                .filter_map(|s| {
                    process_paragraphs(s, &special_char_regex, &self.punctuation, &self.stopwords)
                })
                .collect::<Vec<String>>()
        }

        #[cfg(not(feature = "parallel"))]
        {
            self.text
                .lines()
                .filter_map(|s| {
                    process_paragraphs(s, &special_char_regex, &self.punctuation, &self.stopwords)
                })
                .collect()
        }
    }

    /// Split text into paragraphs (always synchronous even with parallel flag).
    pub fn sync_split_into_paragraphs(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();
        self.text
            .lines()
            .filter_map(|s| {
                process_paragraphs(s, &special_char_regex, &self.punctuation, &self.stopwords)
            })
            .collect()
    }
}
