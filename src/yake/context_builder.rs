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

use super::yake_tokenizer::Sentence;

pub struct Occurrence<'a> {
    shift_offset: usize,
    shift: usize,
    sentence_index: usize,
    word: &'a str,
}

impl<'a> Occurrence<'a> {
    pub fn new(word: &'a str, shift: usize, sentence_index: usize, word_index: usize) -> Self {
        Self {
            word,
            sentence_index,
            shift_offset: shift + word_index,
            shift,
        }
    }

    pub fn get_word(&self) -> &'a str {
        self.word
    }

    pub fn get_sentence_index(&self) -> usize {
        self.sentence_index
    }

    pub fn get_shift_offset(&self) -> usize {
        self.shift_offset
    }

    pub fn get_shift(&self) -> usize {
        self.shift
    }
}

pub type Occurrences<'a> = HashMap<String, Vec<Occurrence<'a>>>;
pub type LeftRightContext<'a> = HashMap<String, (Vec<&'a str>, Vec<&'a str>)>;

pub struct Context<'a> {
    occurrences: Occurrences<'a>,
    contexts: LeftRightContext<'a>,
}

fn is_punctuation(word: &str, punctuation: &HashSet<&str>) -> bool {
    word.is_empty() || ((word.graphemes(true).count() == 1) && punctuation.contains(word))
}

fn build_occurrences<'a>(
    sentences: &'a [Sentence<'a>],
    punctuation: &'a HashSet<&'a str>,
) -> Occurrences<'a> {
    sentences
        .iter()
        .enumerate()
        .fold(Occurrences::new(), |mut acc, (i, sentence)| {
            let shift = sentences
                .iter()
                .take(i)
                .map(|s| s.get_length())
                .sum::<usize>();

            sentence
                .get_words()
                .iter()
                .enumerate()
                .for_each(|(j, word)| {
                    if is_punctuation(word, punctuation) {
                        return ();
                    }

                    let entry = acc.entry(word.to_lowercase()).or_insert(Vec::new());
                    entry.push(Occurrence::new(word, shift, i, j));
                });
            acc
        })
}

fn build_contexts<'a>(sentences: &'a [Sentence<'a>], window_size: usize) -> LeftRightContext<'a> {
    sentences
        .iter()
        .fold(LeftRightContext::new(), |mut acc, sentence| {
            sentence
                .get_words()
                .iter()
                .fold(Vec::<&str>::new(), |mut buffer, w1| {
                    let w1_lower = w1.to_lowercase();

                    buffer.iter().for_each(|w2| {
                        let entry_1 = acc
                            .entry(w1_lower.to_string())
                            .or_insert((Vec::new(), Vec::new()));
                        entry_1.0.push(*w2);
                        let entry_2 = acc
                            .entry(w2.to_lowercase())
                            .or_insert((Vec::new(), Vec::new()));
                        entry_2.1.push(*w1);
                    });

                    buffer.push(*w1);
                    if buffer.len() > window_size {
                        buffer.remove(0);
                    }

                    buffer
                });
            acc
        })
}

impl<'a> Context<'a> {
    pub fn new(
        sentences: &'a [Sentence<'a>],
        punctuation: &'a HashSet<&'a str>,
        window_size: usize,
    ) -> Context<'a> {
        Self {
            occurrences: build_occurrences(sentences, punctuation),
            contexts: build_contexts(sentences, window_size),
        }
    }

    pub fn get_word_occurrences(&self, word: &str) -> Option<&[Occurrence<'a>]> {
        self.occurrences.get(word).map(|v| v.as_slice())
    }

    pub fn get_word_context(&self, word: &str) -> Option<(&[&'a str], &[&'a str])> {
        self.contexts
            .get(word)
            .map(|(v1, v2)| (v1.as_slice(), v2.as_slice()))
    }

    pub fn occurrences(&self) -> impl Iterator<Item = (&String, &Vec<Occurrence<'a>>)> {
        self.occurrences.iter()
    }
}
