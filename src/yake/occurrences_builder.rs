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

use std::collections::{HashMap, HashSet};

use unicode_segmentation::UnicodeSegmentation;

use super::sentences_builder::Sentence;

pub struct Occurrence<'a> {
    pub sentence_index: usize,
    pub word: &'a str,
}

impl<'a> Occurrence<'a> {
    pub fn new(word: &'a str, sentence_index: usize) -> Self {
        Self {
            word,
            sentence_index,
        }
    }
}

pub type Occurrences<'a> = HashMap<&'a str, Vec<Occurrence<'a>>>;

pub struct OccurrencesBuilder;

fn is_punctuation(word: &str, punctuation: &HashSet<&str>) -> bool {
    word.is_empty() || ((word.graphemes(true).count() == 1) && punctuation.contains(word))
}

impl OccurrencesBuilder {
    pub fn build_occurrences<'a>(
        sentences: &'a [Sentence<'a>],
        punctuation: &'a HashSet<&'a str>,
        stop_words: &'a HashSet<&'a str>,
    ) -> Occurrences<'a> {
        sentences
            .iter()
            .enumerate()
            .fold(Occurrences::new(), |mut acc, (i, sentence)| {
                sentence.words.iter().enumerate().for_each(|(j, word)| {
                    if is_punctuation(word, punctuation) || stop_words.contains(word.as_ref()) {
                        return;
                    }

                    let entry = acc.entry(sentence.stemmed[j].as_str()).or_default();
                    entry.push(Occurrence::new(word, i));
                });
                acc
            })
    }
}
