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
    matrix: Vec<Vec<f32>>,
    words_indexes: HashMap<String, usize>,
    indexes_words: HashMap<usize, String>,
}

fn get_window_range(window_size: usize, index: usize, words_length: usize) -> (usize, usize) {
    let window_start = if index < window_size {
        0
    } else {
        index - window_size
    };
    let window_end = if index + window_size + 1 > words_length {
        words_length
    } else {
        index + window_size + 1
    };
    (window_start, window_end)
}

fn create_words_indexes(words: &Vec<String>) -> HashMap<String, usize> {
    words
        .iter()
        .enumerate()
        .map(|(i, w)| (w.to_string(), i))
        .collect::<HashMap<String, usize>>()
}

fn create_indexes_words(labels: &HashMap<String, usize>) -> HashMap<usize, String> {
    labels
        .iter()
        .map(|(w, i)| (i.to_owned(), w.to_string()))
        .collect::<HashMap<usize, String>>()
}

fn get_matrix(
    documents: &Vec<String>,
    words_indexes: &HashMap<String, usize>,
    length: usize,
    window_size: usize,
) -> Vec<Vec<f32>> {
    let mut matrix = vec![vec![0.0_f32; length]; length];
    let mut max = 0.0_f32;

    for document in documents {
        let document_words = document
            .split_whitespace()
            .map(|w| w.to_string())
            .collect::<Vec<String>>();

        for (i, word) in document_words.iter().enumerate() {
            let first_index = match words_indexes.get(word) {
                Some(w) => w.to_owned(),
                None => continue,
            };
            let (window_start, window_end) = get_window_range(window_size, i, document_words.len());

            for j in window_start..window_end {
                if i == j {
                    continue;
                }

                let other_word = match document_words.get(j) {
                    Some(w) => w,
                    None => continue,
                };
                let other_index = match words_indexes.get(other_word) {
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

    for i in 0..length {
        for j in 0..length {
            matrix[i][j] /= max;
        }
    }

    matrix
}

impl CoOccurrence {
    pub fn new(documents: &Vec<String>, words: &Vec<String>, window_size: usize) -> CoOccurrence {
        let words_indexes = create_words_indexes(words);
        let length = words.len();

        Self {
            matrix: get_matrix(documents, &words_indexes, length, window_size),
            indexes_words: create_indexes_words(&words_indexes),
            words_indexes,
        }
    }

    pub fn get_label(&self, word: &str) -> Option<usize> {
        match self.words_indexes.get(word) {
            Some(w) => Some(w.to_owned()),
            None => None,
        }
    }

    pub fn get_word(&self, label: usize) -> Option<String> {
        match self.indexes_words.get(&label) {
            Some(w) => Some(w.to_owned()),
            None => None,
        }
    }

    pub fn get_matrix(&self) -> &Vec<Vec<f32>> {
        &self.matrix
    }

    pub fn get_labels(&self) -> &HashMap<String, usize> {
        &self.words_indexes
    }

    pub fn get_relations(&self, word: &str) -> Option<Vec<(String, f32)>> {
        let label = match self.get_label(word) {
            Some(l) => l,
            None => return None,
        };
        Some(
            self.matrix[label]
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| {
                    if v > 0.0 {
                        if let Some(w) = self.get_word(i) {
                            return Some((w, v));
                        }
                    }

                    None
                })
                .collect::<Vec<(String, f32)>>(),
        )
    }

    pub fn get_matrix_row(&self, word: &str) -> Option<Vec<f32>> {
        let label = match self.get_label(word) {
            Some(l) => l,
            None => return None,
        };
        Some(self.matrix[label].to_owned())
    }

    pub fn get_relation(&self, word1: &str, word2: &str) -> Option<f32> {
        let label1 = match self.get_label(word1) {
            Some(l) => l,
            None => return None,
        };
        let label2 = match self.get_label(word2) {
            Some(l) => l,
            None => return None,
        };
        Some(self.matrix[label1][label2])
    }
}
