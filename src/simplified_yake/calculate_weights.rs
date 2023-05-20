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

use crate::co_occurrence::CoOccurrence;

pub struct WeightedCandidate {
    term: String,
    tf: f32,
    c_value: f32,
    pfo: f32,
    plo: f32,
    avg_cooccurrence: f32,
}

fn generate_count_hashmap(candidates: &[String]) -> HashMap<&str, f32> {
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

fn calculate_avg_cooccurrence(
    co_occurrence_matrix: CoOccurrence,
    candidates: &[String],
) -> HashMap<&str, f32> {
    let mut avg_cooccurrence = HashMap::new();

    for candidate in candidates {
        if let Some(cooccurrence_row) = co_occurrence_matrix.get_matrix_row(candidate) {
            let sum: f32 = cooccurrence_row.iter().sum();
            let avg = sum / cooccurrence_row.len() as f32;
            avg_cooccurrence.insert(candidate.as_str(), avg);
        }
    }

    avg_cooccurrence
}

impl WeightedCandidate {
    pub fn new(
        term: String,
        tf: f32,
        c_value: f32,
        pfo: f32,
        plo: f32,
        avg_cooccurrence: f32,
    ) -> WeightedCandidate {
        WeightedCandidate {
            term,
            tf,
            c_value,
            pfo,
            plo,
            avg_cooccurrence,
        }
    }

    pub fn term(&self) -> String {
        self.term.to_owned()
    }
}

pub fn calculate_weights(
    sentences: &[String],
    candidates: &[String],
    window_size: usize,
) -> Vec<WeightedCandidate> {
    let total_terms = candidates.len() as f32;

    if total_terms == 0.0 {
        return vec![];
    }

    let counts = generate_count_hashmap(candidates);
    let co_occurrence_matrix = CoOccurrence::new(sentences, candidates, window_size);

    let tf = calculate_tf(&counts);
    let c_value = calculate_c_value(&tf);
    let pfo = calculate_pfo(&counts, total_terms);
    let plo = calculate_plo(&counts);
    let avg_cooccurrence = calculate_avg_cooccurrence(co_occurrence_matrix, candidates);

    candidates
        .iter()
        .map(|candidate| {
            let tf = tf.get(candidate.as_str()).unwrap_or(&0.0);
            let c_value = c_value.get(candidate.as_str()).unwrap_or(&0.0);
            let pfo = pfo.get(candidate.as_str()).unwrap_or(&0.0);
            let plo = plo.get(candidate.as_str()).unwrap_or(&0.0);
            let avg_cooccurrence = avg_cooccurrence.get(candidate.as_str()).unwrap_or(&0.0);

            WeightedCandidate::new(
                candidate.clone(),
                *tf,
                *c_value,
                *pfo,
                *plo,
                *avg_cooccurrence,
            )
        })
        .collect::<Vec<WeightedCandidate>>()
}

impl WeightedCandidate {
    pub fn calculate_score(&self, w_tf: f32, w_c: f32, w_pf: f32, w_pl: f32, w_avg: f32) -> f32 {
        self.tf.powf(w_tf)
            * self.c_value.powf(w_c)
            * self.pfo.powf(w_pf)
            * self.plo.powf(w_pl)
            * self.avg_cooccurrence.powf(w_avg)
    }
}
