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

type RightLeftContextItem<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;
pub type RightLeftContext<'a> = HashMap<String, RightLeftContextItem<'a>>;

fn process_word<'a>(
    word: &'a str,
    stopwords: &'a HashSet<&'a str>,
    punctuation: &'a HashSet<&'a str>,
) -> Option<&'a str> {
    if word.is_empty()
        || (word.graphemes(true).count() == 1 && punctuation.contains(word))
        || stopwords.contains(word)
    {
        return None;
    }
    Some(word)
}

fn process_ngrams(candidate: Vec<&str>) -> Vec<Vec<&str>> {
    (0..candidate.len())
        .rev()
        .map(|i| candidate[0..=i].to_vec())
        .collect()
}

fn process_candidates<'a>(
    mut candidates: Vec<Vec<&'a str>>,
    mut candidate: Vec<&'a str>,
    word: &'a str,
    punctuation: &'a HashSet<&'a str>,
    stopwords: &'a HashSet<&'a str>,
    ngrams: usize,
) -> (Vec<Vec<&'a str>>, Vec<&'a str>) {
    let word = word.trim();

    if !(word.graphemes(true).count() == 1 && punctuation.contains(word)) {
        if stopwords.contains(word) {
            if !candidate.is_empty() && candidate.len() <= ngrams {
                candidates.extend(process_ngrams(candidate));
                candidate = Vec::new();
            }
        } else {
            candidate.push(word);
            if candidate.len() == ngrams {
                candidates.extend(process_ngrams(candidate));
                candidate = Vec::new();
            }
        }
    }

    (candidates, candidate)
}

fn build_right_left_context<'a>(
    sentences: &'a [Vec<&'a str>],
    window_size: usize,
) -> RightLeftContext<'a> {
    sentences.iter().fold(HashMap::new(), |mut ctx, sentence| {
        sentence.iter().enumerate().for_each(|(i, word)| {
            let entry: &mut RightLeftContextItem = ctx.entry(word.to_lowercase()).or_default();
            let left = sentence
                .iter()
                .take(i)
                .rev()
                .take(window_size)
                .rev()
                .copied()
                .collect();
            let right = sentence
                .iter()
                .skip(i + 1)
                .take(window_size)
                .copied()
                .collect();
            entry.push((left, right));
        });
        ctx
    })
}

pub struct ContextBuilder<'a> {
    text: &'a str,
    stopwords: HashSet<&'a str>,
    punctuation: HashSet<&'a str>,
    window_size: usize,
    ngrams: usize,
}

impl<'a> ContextBuilder<'a> {
    pub fn new(
        text: &'a str,
        stopwords: HashSet<&'a str>,
        punctuation: HashSet<&'a str>,
        window_size: usize,
        ngrams: usize,
    ) -> Self {
        Self {
            text,
            stopwords,
            punctuation,
            window_size,
            ngrams,
        }
    }

    // -- PRE_PROCESSOR START --
    pub fn build_words(&'a self) -> Vec<&'a str> {
        self.text
            .unicode_words()
            .filter_map(|w| process_word(w, &self.stopwords, &self.punctuation))
            .collect()
    }

    pub fn build_sentences(&'a self) -> Vec<Vec<&'a str>> {
        self.text
            .unicode_sentences()
            .map(|s| {
                s.trim()
                    .unicode_words()
                    .filter_map(|w| process_word(w, &self.stopwords, &self.punctuation))
                    .collect()
            })
            .collect()
    }
    // -- PRE_PROCESSOR END --

    // -- CONTEXT BUILDER START --
    pub fn build_pre_candidates(&'a self, words: &'a [&'a str]) -> Vec<Vec<&'a str>> {
        let (mut candidates, last_candidate) = words.iter().fold(
            (Vec::<Vec<&'a str>>::new(), Vec::<&'a str>::new()),
            |(candidates, candidate), word| {
                process_candidates(
                    candidates,
                    candidate,
                    word,
                    &self.punctuation,
                    &self.stopwords,
                    self.ngrams,
                )
            },
        );

        if !last_candidate.is_empty() && last_candidate.len() <= self.ngrams {
            candidates.extend(process_ngrams(last_candidate));
        }

        candidates
    }

    pub fn build_right_left_context(
        &'a self,
        sentences: &'a [Vec<&'a str>],
    ) -> RightLeftContext<'a> {
        build_right_left_context(sentences, self.window_size)
    }
    // -- CONTEXT BUILDER END --
}
