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

use unicode_segmentation::UnicodeSegmentation;

use super::sentences_builder::Sentence;

#[derive(Clone)]
pub struct Candidate<'a> {
    pub lexical_form: Vec<&'a str>,
    pub surface_forms: Vec<Vec<&'a str>>,
}

impl<'a> Candidate<'a> {
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

type Candidates<'a> = HashMap<String, Candidate<'a>>;
type DedupMap<'a> = HashMap<&'a str, f32>;
pub type LeftRightContext<'a> = HashMap<&'a str, (Vec<&'a str>, Vec<&'a str>)>;
pub type Occurrences<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

fn is_punctuation(word: &str, punctuation: &HashSet<&str>) -> bool {
    word.is_empty() || ((word.graphemes(true).count() == 1) && punctuation.contains(word))
}

pub struct CandidateSelectionAndContextBuilder;

impl<'a> CandidateSelectionAndContextBuilder {
    pub fn select_candidates_and_build_context(
        sentences: &'a [Sentence],
        ngram: usize,
        window_size: usize,
        stop_words: HashSet<&'a str>,
        punctuation: HashSet<&'a str>,
    ) -> (
        // Candidate Selection
        Candidates<'a>,
        DedupMap<'a>,
        // Context Builder
        Occurrences<'a>,
        LeftRightContext<'a>,
    ) {
        sentences.iter().enumerate().fold(
            (
                Candidates::new(),
                DedupMap::new(),
                Occurrences::new(),
                LeftRightContext::new(),
            ),
            |(mut candidates, mut dedup_map, mut occurrences, mut lr_contexts), (i, sentence)| {
                // Candidate Selection
                let sentence_len = sentence.length;
                (0..sentence_len).for_each(|j| {
                    (j + 1..=min(j + ngram, sentence_len)).for_each(|k: usize| {
                        let stems = sentence.stemmed[j..k]
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&'a str>>();
                        if stems.iter().any(|w| {
                            is_punctuation(w, &punctuation)
                                || stop_words.contains(w)
                                || w.parse::<f32>().is_ok()
                        }) {
                            return;
                        }

                        let words = sentence.words[j..k]
                            .iter()
                            .map(|s| s.as_ref())
                            .collect::<Vec<&'a str>>();
                        let key = stems.join(" ");
                        let entry = match candidates.get_mut(&key) {
                            Some(entry) => entry,
                            None => {
                                if stems.len() > 1 {
                                    stems.iter().for_each(|w| {
                                        let entry = dedup_map.entry(*w).or_insert(0.0);
                                        *entry += 1.0;
                                    });
                                }

                                candidates.entry(key).or_insert(Candidate::new(stems))
                            }
                        };
                        entry.add(words);
                    });
                });

                // Context Builder
                sentence.words.iter().enumerate().fold(
                    Vec::<(&str, usize)>::new(),
                    |mut buffer, (j, w1)| {
                        let key1 = sentence.stemmed[j].as_str();
                        let w1_str = w1.as_ref();

                        if !(is_punctuation(key1, &punctuation) || stop_words.contains(key1)) {
                            let entry = occurrences.entry(key1).or_default();
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
                (candidates, dedup_map, occurrences, lr_contexts)
            },
        )
    }
}
