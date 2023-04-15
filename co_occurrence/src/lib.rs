// Copyright (C) 2023 Afonso Barracha
//
// This file is part of keyword-extraction.
//
// keyword-extraction is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// keyword-extraction is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with keyword-extraction.  If not, see <http://www.gnu.org/licenses/>.


use std::collections::HashMap;

pub struct CoOccurrence {
    documents: Vec<String>,
    words: HashMap<String, usize>,
    length: usize,
    window: usize,
}

impl CoOccurrence {
    pub fn new(documents: &Vec<String>, words: &Vec<String>, window: usize) -> CoOccurrence {
        CoOccurrence {
            documents: documents.clone(),
            words: HashMap::from_iter(
                words
                    .into_iter()
                    .enumerate()
                    .map(|(i, w)| (w.to_string(), i)),
            ),
            length: words.len(),
            window,
        }
    }

    pub fn get_matrix(&self) -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![0.0; self.length]; self.length];
        let mut max = 0.0;

        for document in &self.documents {
            let document_words: Vec<String> =
                document.split_whitespace().map(|w| w.to_string()).collect();

            for (i, word) in document_words.iter().enumerate() {
                let first_index = match self.words.get(word) {
                    Some(w) => w.to_owned(),
                    None => continue,
                };
                let words_length = document_words.len();
                let window_start = if i < self.window { 0 } else { i - self.window };
                let window_end = if i + self.window + 1 > words_length {
                    words_length
                } else {
                    i + self.window + 1
                };

                for j in window_start..window_end {
                    if i == j {
                        continue;
                    }

                    let other_word = match document_words.get(j) {
                        Some(w) => w,
                        None => continue,
                    };
                    let other_index = match self.words.get(other_word) {
                        Some(w) => w.to_owned(),
                        None => continue,
                    };

                    matrix[first_index][other_index] += 1.0;
                    let current = matrix[first_index][other_index];

                    if current > max {
                        max = current;
                    }
                }
            }
        }

        for i in 0..self.length {
            for j in 0..self.length {
                matrix[i][j] /= max;
            }
        }

        matrix
    }
}
