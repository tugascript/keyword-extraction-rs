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

use std::collections::HashMap;

pub struct WeightedCandidate<'a> {
    term: &'a str,
    tf: f32,
    c_value: f32,
    pfo: f32,
    plo: f32,
}

fn generate_count_hashmap<'a>(candidates: &'a [String]) -> HashMap<&'a str, f32> {
    candidates.iter().fold(HashMap::new(), |mut acc, word| {
        let count = acc.entry(word).or_insert(0.0);
        *count += 1.0;
        acc
    })
}

fn calculate_tf<'a>(counts: &HashMap<&'a str, f32>) -> HashMap<&'a str, f32> {
    let total_words = counts.values().sum::<f32>();

    counts
        .iter()
        .map(|(word, count)| (*word, count / total_words))
        .collect::<HashMap<&'a str, f32>>()
}

fn calculate_c_value<'a>(tf: &HashMap<&'a str, f32>) -> HashMap<&'a str, f32> {
    tf.iter()
        .map(|(term, frequency)| {
            let length = term.split_whitespace().count();
            let c_value = (1.0 + length as f32).log2() * frequency;
            (*term, c_value)
        })
        .collect::<HashMap<&'a str, f32>>()
}

fn calculate_pfo<'a>(counts: &HashMap<&'a str, f32>, total_terms: f32) -> HashMap<&'a str, f32> {
    counts
        .iter()
        .map(|(term, count)| {
            let pfo = *count / total_terms;
            (*term, pfo)
        })
        .collect::<HashMap<&'a str, f32>>()
}

fn calculate_plo<'a>(counts: &HashMap<&'a str, f32>) -> HashMap<&'a str, f32> {
    counts
        .iter()
        .map(|(term, count)| {
            let length = term.split_whitespace().count() as f32;
            let plo = *count / length.powi(2);
            (*term, plo)
        })
        .collect::<HashMap<&'a str, f32>>()
}

impl<'a> WeightedCandidate<'a> {
    pub fn new(term: &'a str, tf: f32, c_value: f32, pfo: f32, plo: f32) -> WeightedCandidate<'a> {
        WeightedCandidate {
            term,
            tf,
            c_value,
            pfo,
            plo,
        }
    }

    pub fn term(&self) -> &'a str {
        self.term
    }

    pub fn tf(&self) -> f32 {
        self.tf
    }

    pub fn c_value(&self) -> f32 {
        self.c_value
    }

    pub fn pfo(&self) -> f32 {
        self.pfo
    }

    pub fn plo(&self) -> f32 {
        self.plo
    }
}

pub fn calculate_weights(candidates: &[String]) -> Vec<WeightedCandidate> {
    let total_terms = candidates.len() as f32;

    if total_terms == 0.0 {
        return vec![];
    }

    let counts = generate_count_hashmap(candidates);
    let tf = calculate_tf(&counts);
    let c_value = calculate_c_value(&tf);
    let pfo = calculate_pfo(&counts, total_terms);
    let plo = calculate_plo(&counts);

    candidates
        .iter()
        .map(|candidate| {
            let tf = tf.get(candidate.as_str()).unwrap_or(&0.0);
            let c_value = c_value.get(candidate.as_str()).unwrap_or(&0.0);
            let pfo = pfo.get(candidate.as_str()).unwrap_or(&0.0);
            let plo = plo.get(candidate.as_str()).unwrap_or(&0.0);

            WeightedCandidate::new(candidate.as_str(), *tf, *c_value, *pfo, *plo)
        })
        .collect::<Vec<WeightedCandidate>>()
}
