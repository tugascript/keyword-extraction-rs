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

use super::levenshtein::Levenshtein;

fn generate_ngrams(phrases: &[String], n: usize) -> Vec<String> {
    phrases
        .iter()
        .flat_map(|phrase| {
            let words: Vec<&str> = phrase.split_whitespace().collect();

            if words.len() < n {
                vec![phrase.clone()]
            } else {
                words
                    .windows(n)
                    .map(|window| window.join(" "))
                    .collect::<Vec<String>>()
            }
        })
        .collect::<Vec<String>>()
}

fn generate_candidates(phrases: &[String], n: usize) -> Vec<String> {
    (1..(n + 1))
        .flat_map(|n| generate_ngrams(phrases, n))
        .collect::<Vec<String>>()
}

fn is_similar(candidate: &str, filtered_cadidates: &[String], threshold: f32) -> bool {
    filtered_cadidates
        .iter()
        .any(|existent| Levenshtein::new(candidate, existent).ratio() > threshold)
}

fn remove_similar_candidates(candidates: Vec<String>, threshold: f32) -> Vec<String> {
    candidates
        .iter()
        .fold(Vec::<String>::new(), |mut acc, candidate| {
            if !is_similar(candidate, &acc, threshold) {
                acc.push(candidate.clone());
            }
            acc
        })
}

pub struct CandidateSelection(Vec<String>);

impl CandidateSelection {
    pub fn new(phrases: &[String], n: usize, threshold: f32) -> Self {
        Self(remove_similar_candidates(
            generate_candidates(phrases, n),
            threshold,
        ))
    }

    pub fn get_candidates(&self) -> &[String] {
        &self.0
    }
}
