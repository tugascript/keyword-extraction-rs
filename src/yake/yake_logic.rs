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
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use regex::Regex;

use super::{
    candidate_selection::{CandidateSelection, PreCandidate},
    context_builder::ContextBuilder,
    feature_extraction::FeatureExtraction,
    occurrences_builder::OccurrencesBuilder,
    sentences_builder::SentencesBuilder,
};

pub struct YakeLogic;

fn get_space_regex() -> Option<Regex> {
    Regex::new(r"[\n\t\r]").ok()
}

fn process_text<'a>(text: &'a str) -> Cow<'a, str> {
    let space_regex = get_space_regex();
    let trimmed_text = text.trim();

    if let Some(regex) = space_regex {
        regex.replace_all(trimmed_text, " ")
    } else {
        trimmed_text.into()
    }
}

impl YakeLogic {
    pub fn build_yake(
        text: &str,
        stop_words: HashSet<&str>,
        punctuation: HashSet<&str>,
        threshold: f32,
        ngram: usize,
        window_size: usize,
    ) -> HashMap<String, f32> {
        let processed_text = process_text(text);
        let sentences = SentencesBuilder::build_sentences(processed_text.as_ref());
        let feature_extraction = FeatureExtraction::new(
            OccurrencesBuilder::build_occurrences(&sentences, &punctuation, &stop_words),
            ContextBuilder::build_context(&sentences, window_size),
            &sentences,
        );
        let candidates = CandidateSelection::select_candidates(
            &sentences,
            ngram,
            &stop_words,
            &punctuation,
            threshold,
        );
        let dedup_hashset = Self::build_de_duplicate_hashset(&candidates);
        Self::score_candidates(feature_extraction.0, candidates, dedup_hashset)
    }

    fn build_de_duplicate_hashset<'a>(
        candidates: &'a HashMap<String, PreCandidate<'a>>,
    ) -> HashSet<String> {
        candidates.iter().fold(HashSet::new(), |mut acc, (_, pc)| {
            if pc.lexical_form.len() > 1 {
                pc.lexical_form.iter().for_each(|w| {
                    acc.insert(w.to_string());
                });
            }

            acc
        })
    }

    fn score_candidates<'a>(
        feature_extraction: HashMap<&'a str, f32>,
        candidates: HashMap<String, PreCandidate<'a>>,
        dedup_hashset: HashSet<String>,
    ) -> HashMap<String, f32> {
        let mut max = 0.0_f32;
        let values = candidates
            .into_iter()
            .map(|(k, pc)| {
                let (prod, sum) = pc.lexical_form.iter().fold(
                    (if dedup_hashset.contains(&k) { 6.0 } else { 1.0 }, 0.0),
                    |acc, w| {
                        let weight = *feature_extraction.get(*w).unwrap_or(&0.0);

                        (acc.0 * weight, acc.1 + weight)
                    },
                );
                let tf = pc.surface_forms.len() as f32;
                let sum = if sum == -1.0 { 1.0 - f32::EPSILON } else { sum };
                let value = 1.0 / (prod / (tf * (1.0 + sum)));

                if value > max {
                    max = value;
                }

                (k, value)
            })
            .collect::<Vec<(String, f32)>>();
        values.into_iter().map(|(k, v)| (k, v / max)).collect()
    }
}
