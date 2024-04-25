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

pub type LeftRightContext<'a> = HashMap<&'a str, (Vec<&'a str>, Vec<&'a str>)>;
pub type Occurrences<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

fn is_punctuation(word: &str, punctuation: &HashSet<&str>) -> bool {
    word.is_empty() || ((word.graphemes(true).count() == 1) && punctuation.contains(word))
}

pub struct ContextBuilder;

impl<'a> ContextBuilder {
    pub fn build_context(
        sentences: &'a [Sentence],
        window_size: usize,
        punctuation: &'a HashSet<&'a str>,
        stop_words: &'a HashSet<&'a str>,
    ) -> (Occurrences<'a>, LeftRightContext<'a>) {
        sentences.iter().enumerate().fold(
            (Occurrences::new(), LeftRightContext::new()),
            |(mut occurences, mut lr_contexts), (i, sentence)| {
                sentence.words.iter().enumerate().fold(
                    Vec::<(&str, usize)>::new(),
                    |mut buffer, (j, w1)| {
                        let key1 = sentence.stemmed[j].as_str();
                        let w1_str = w1.as_str();

                        if !(is_punctuation(key1, punctuation) || stop_words.contains(key1)) {
                            let entry = occurences.entry(key1).or_default();
                            entry.push((w1_str, i));
                        }

                        buffer.iter().for_each(|(w2, k)| {
                            let entry_1 =
                                lr_contexts.entry(key1).or_insert((Vec::new(), Vec::new()));
                            entry_1.0.push(*w2);
                            let entry_2 = lr_contexts
                                .entry(sentence.stemmed[*k].as_str())
                                .or_insert((Vec::new(), Vec::new()));
                            entry_2.1.push(w1_str);
                        });

                        buffer.push((w1_str, j));

                        if buffer.len() > window_size {
                            buffer.remove(0);
                        }

                        buffer
                    },
                );
                (occurences, lr_contexts)
            },
        )
    }
}
