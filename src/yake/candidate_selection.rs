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

use super::{levenshtein::Levenshtein, sentences_builder::Sentence};

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

fn build_pre_candidates<'a>(sentences: &'a [Sentence<'a>], ngram: usize) -> PreCandidates<'a> {
    sentences
        .iter()
        .fold(PreCandidates::new(), |mut acc, sentence| {
            let sentence_len = sentence.length;
            let skip = min(ngram, sentence_len);

            (0..sentence_len).for_each(|i| {
                (i + 1..=min(i + skip, sentence_len)).for_each(|j: usize| {
                    let stems = sentence.stemmed[i..j]
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<&'a str>>();
                    let words = sentence.words[i..j]
                        .iter()
                        .map(|s| s.as_ref())
                        .collect::<Vec<&'a str>>();
                    let key = stems.join(" ");
                    let entry = acc.entry(key).or_insert(PreCandidate::new(stems));
                    entry.add(words);
                });
            });
            acc
        })
}

fn filter_pre_candidates<'a>(
    pre_candidates: PreCandidates<'a>,
    stop_words: &'a HashSet<&'a str>,
    punctuation: &'a HashSet<&'a str>,
    threshold: f32,
) -> HashMap<String, PreCandidate<'a>> {
    let first_iter = pre_candidates
        .into_iter()
        .filter_map(|(v, pc)| {
            if pc.surface_forms.len() == 0 {
                return None;
            }
            if pc.surface_forms[0].len() == 0 {
                return None;
            }
            if pc.lexical_form.iter().any(|w| {
                stop_words.contains(w) || punctuation.contains(w) || w.parse::<f32>().is_ok()
            }) {
                return None;
            }

            Some((v, pc))
        })
        .collect::<Vec<(String, PreCandidate<'a>)>>();
    first_iter
        .iter()
        .enumerate()
        .filter_map(|(i, (k1, v))| {
            for (k2, _) in &first_iter[i + 1..] {
                let lev = Levenshtein::new(&k1, k2);
                if lev.ratio() >= threshold {
                    return None;
                }
            }
            Some((k1.to_string(), v.to_owned()))
        })
        .collect()
}

pub struct CandidateSelection;

impl<'a> CandidateSelection {
    pub fn select_candidates(
        sentences: &'a [Sentence<'a>],
        ngram: usize,
        stop_words: &'a HashSet<&'a str>,
        punctuation: &'a HashSet<&'a str>,
        threshold: f32,
    ) -> PreCandidates<'a> {
        filter_pre_candidates(
            build_pre_candidates(sentences, ngram),
            stop_words,
            punctuation,
            threshold,
        )
    }
}
