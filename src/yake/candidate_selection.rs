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

use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use super::sentences_builder::Sentence;

#[derive(Clone)]
pub struct PreCandidate<'a> {
    pub lexical_form: Vec<&'a str>,
    pub surface_forms: Vec<Vec<&'a str>>,
}

impl<'a> PreCandidate<'a> {
    pub fn new(stems: Vec<&'a str>) -> Self {
        Self {
            lexical_form: stems,
            surface_forms: Vec::new(),
        }
    }

    fn add(&mut self, words: Vec<&'a str>) {
        self.surface_forms.push(words);
    }
}

type PreCandidates<'a> = HashMap<String, PreCandidate<'a>>;

pub struct CandidateSelection;

impl<'a> CandidateSelection {
    pub fn select_candidates(
        sentences: &'a [Sentence<'a>],
        ngram: usize,
        stop_words: &'a HashSet<&'a str>,
        punctuation: &'a HashSet<&'a str>,
    ) -> PreCandidates<'a> {
        sentences
            .iter()
            .fold(PreCandidates::new(), |mut acc, sentence| {
                let sentence_len = sentence.length;

                (0..sentence_len).for_each(|i| {
                    (i + 1..=min(i + ngram, sentence_len)).for_each(|j: usize| {
                        let stems = sentence.stemmed[i..j]
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&'a str>>();

                        if stems.iter().any(|w| {
                            stop_words.contains(w)
                                || punctuation.contains(w)
                                || w.parse::<f32>().is_ok()
                        }) {
                            return;
                        }

                        let words = sentence.words[i..j]
                            .iter()
                            .map(|s| s.as_ref())
                            .collect::<Vec<&'a str>>();
                        let entry = acc
                            .entry(stems.join(" "))
                            .or_insert(PreCandidate::new(stems));
                        entry.add(words);
                    });
                });
                acc
            })
    }
}
