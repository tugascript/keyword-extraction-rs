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

use super::yake_tokenizer::Sentence;

type PreCandidate<'a> = HashMap<String, (Vec<&'a str>, Vec<Vec<&'a str>>)>;

fn build_pre_candidates<'a>(sentences: &'a [Sentence], ngram: usize) -> PreCandidate<'a> {
    sentences
        .iter()
        .fold(PreCandidate::new(), |mut acc, sentence| {
            let sentence_len = sentence.get_length();

            (0..sentence_len).for_each(|j| {
                (j + 1..=min(j + ngram, sentence_len)).for_each(|k| {
                    let stems = sentence.get_stemmed()[j..k]
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<&'a str>>();
                    let words = sentence.get_words()[j..k].to_vec();
                    let key = stems.join(" ");
                    let entry = acc.entry(key).or_insert((Vec::new(), Vec::new()));
                    if entry.0.is_empty() {
                        entry.0 = stems;
                    }
                    entry.1.push(words);
                });
            });
            acc
        })
}

fn filter_pre_candidates<'a>(
    mut pre_candidates: PreCandidate<'a>,
    stop_words: &'a HashSet<&'a str>,
) -> PreCandidate<'a> {
    pre_candidates.retain(|_, (s, _)| s.iter().all(|w| !stop_words.contains(w)));
    pre_candidates
}

pub struct Candidates<'a>(PreCandidate<'a>);

impl<'a> Candidates<'a> {
    pub fn new(sentences: &'a [Sentence], ngram: usize, stop_words: &'a HashSet<&'a str>) -> Self {
        Self(filter_pre_candidates(
            build_pre_candidates(sentences, ngram),
            stop_words,
        ))
    }

    pub fn candidates(
        &self,
    ) -> impl Iterator<Item = (&String, &(Vec<&'a str>, Vec<Vec<&'a str>>))> {
        self.0.iter()
    }
}
